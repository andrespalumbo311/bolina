pub mod iio;
pub mod tablet_mode;
pub mod tracker;

use crate::common::{acquire_lock, autorotate_lock};
use crate::wallpaper::niri::get_niri_transform;
use crate::wallpaper::renderer::apply_wallpaper;
use std::process::Command;
use std::thread;
use std::time::Duration;

pub fn set_niri_transform(target: &str, output_name: &str) {
    let current = get_niri_transform(output_name);
    if current == target {
        return;
    }

    println!(
        "[autorotate] Applying transform: {} -> {}",
        current, target
    );

    // Launch both Niri transform and DMS wallpaper update concurrently
    // so DMS receives the geometry rebind and new wallpaper URL simultaneously,
    // completely eliminating the intermediate cropped frame.
    let target_owned = target.to_string();
    let output_owned = output_name.to_string();

    let t_dms = thread::spawn(move || {
        let _ = apply_wallpaper(Some(&target_owned), None, &output_owned);
    });

    let status = Command::new("niri")
        .args(["msg", "output", output_name, "transform", target])
        .status();

    if let Err(e) = status {
        eprintln!("[autorotate] Failed to set transform: {}", e);
    }

    let _ = t_dms.join();

    // Se la tastiera virtuale OSK è aperta durante la rotazione fisica, ridimensionala
    let _ = Command::new("bolina-osk").arg("resize").spawn();
}

pub fn is_genuine_laptop_mode(
    screen: Option<(i64, i64, i64)>,
    base: Option<(i64, i64, i64)>,
) -> bool {
    // 1. Verify screen accelerometer posture
    if let Some((sx, sy, sz)) = screen {
        let (sx, sy, sz) = (sx as f64, sy as f64, sz as f64);
        let s_ns = (sx * sx + sy * sy + sz * sz).sqrt();
        if s_ns > 100_000.0 {
            let norm_sx = (sx / s_ns).abs();
            let norm_sy = sy / s_ns;

            // In genuine laptop posture:
            // The screen must be landscape upright: norm_sy < -0.50, and not portrait (norm_sx < 0.45).
            // If held sideways in portrait (|norm_sx| >= 0.45) or inverted/tent (norm_sy >= -0.50),
            // it is definitively NOT being operated in laptop mode.
            if norm_sx >= 0.45 || norm_sy >= -0.50 {
                return false;
            }
        }
    }

    // 2. Verify base accelerometer posture if available
    if let Some((bx, by, bz)) = base {
        let (bx, by, bz) = (bx as f64, by as f64, bz as f64);
        let b_ns = (bx * bx + by * by + bz * bz).sqrt();
        if b_ns > 100_000.0 {
            let norm_bx = (bx / b_ns).abs();
            let norm_bz = bz / b_ns;

            // In genuine laptop posture on desk/lap:
            // The keyboard faces upward: norm_bz is negative (~ -0.98), and not vertical (|norm_bx| < 0.45).
            // If the keyboard faces downward (norm_bz > -0.50) or is vertical (|norm_bx| >= 0.45),
            // the chassis is folded back-to-back in tablet mode or placed on its edge.
            if norm_bz > -0.50 || norm_bx >= 0.45 {
                return false;
            }
        }
    }

    true
}

pub fn run_autorotate(output_name: &str) -> Result<(), String> {
    let mut sensor_dir = match iio::find_accelerometer() {
        Some(s) => s,
        None => {
            println!(
                "[autorotate] No accelerometer detected on this hardware. Auto-rotation disabled."
            );
            return Ok(());
        }
    };

    let _lock = acquire_lock(&autorotate_lock())?;
    let base_sensor_dir = iio::find_base_accelerometer();
    println!(
        "[autorotate] Starting ThinkPad auto-rotation daemon on {:?} (base: {:?}, output={})...",
        sensor_dir, base_sensor_dir, output_name
    );

    let initial_tr = get_niri_transform(output_name);
    let mut tracker = tracker::OrientationTracker::new(&initial_tr);
    let mut is_in_tablet_mode = tablet_mode::is_tablet_mode();

    loop {
        thread::sleep(Duration::from_millis(350));

        let acpi_tablet = tablet_mode::is_tablet_mode();

        if acpi_tablet {
            is_in_tablet_mode = true;
        } else if is_in_tablet_mode {
            // ACPI reported 0, but the daemon was latched in tablet mode.
            // Check if this is a genuine return to laptop posture or the ~340° magnet detach.
            let screen_accel = iio::read_accel_raw(&sensor_dir);
            let base_accel = base_sensor_dir.as_ref().and_then(|d| iio::read_accel_raw(d));

            if is_genuine_laptop_mode(screen_accel, base_accel) {
                println!(
                    "[autorotate] Verified genuine Laptop mode transition. Locking to normal."
                );
                is_in_tablet_mode = false;
                if tracker.current_transform != "normal" {
                    tracker.set_forced("normal");
                    set_niri_transform("normal", output_name);
                    let _ = Command::new("bolina-osk").arg("close").spawn();
                }
                thread::sleep(Duration::from_secs(1));
                continue;
            }
            // Posture confirms the user is still in tablet/portrait orientation (~340° magnet gap).
            // Retain tablet latch and continue auto-rotating.
        } else {
            // In standard laptop mode: remain locked to normal.
            if tracker.current_transform != "normal" {
                println!(
                    "[autorotate] Laptop mode active. Locking to normal."
                );
                tracker.set_forced("normal");
                set_niri_transform("normal", output_name);
                let _ = Command::new("bolina-osk").arg("close").spawn();
            }
            thread::sleep(Duration::from_secs(1));
            continue;
        }

        let accel = match iio::read_accel_raw(&sensor_dir) {
            Some(a) => a,
            None => {
                match iio::find_accelerometer() {
                    Some(s) => sensor_dir = s,
                    None => {
                        println!("[autorotate] Accelerometer disconnected. Exiting.");
                        return Ok(());
                    }
                }
                continue;
            }
        };

        let target_tr = tracker.update(accel);
        if target_tr != get_niri_transform(output_name) {
            set_niri_transform(&target_tr, output_name);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_laptop_mode_detection_on_desk() {
        // Screen upright landscape: X~0, Y~ -980k, Z~ -28k
        let screen = Some((30_000, -980_000, -28_000));
        // Base flat on desk: X~0, Y~0, Z~ -980k
        let base = Some((30_000, -60_000, -980_000));

        assert!(is_genuine_laptop_mode(screen, base));
    }

    #[test]
    fn test_tablet_portrait_gap_340_held_vertically() {
        // Screen held in portrait (X dominant): X~ -964k, Y~ -51k, Z~ -193k
        let screen = Some((-964_494, -51_421, -193_682));
        // Base also vertical on edge: X~ -1008k, Y~ 64k, Z~ +208k
        let base = Some((-1008_000, 64_000, 208_000));

        // Must reject laptop mode transition!
        assert!(!is_genuine_laptop_mode(screen, base));
    }

    #[test]
    fn test_tablet_tent_mode_rejected_from_laptop() {
        // Tent mode: Y positive
        let screen = Some((50_000, 950_000, 200_000));
        let base = Some((50_000, -950_000, 200_000));

        assert!(!is_genuine_laptop_mode(screen, base));
    }

    #[test]
    fn test_tablet_folded_flat_keyboard_down() {
        // Tablet horizontal, keyboard facing down (Z positive on base)
        let screen = Some((10_000, -900_000, 100_000));
        let base = Some((10_000, 900_000, 950_000)); // base Z positive: keyboard faces ground

        assert!(!is_genuine_laptop_mode(screen, base));
    }
}
