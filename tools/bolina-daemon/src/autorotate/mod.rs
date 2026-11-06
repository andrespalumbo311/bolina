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

pub fn run_autorotate(output_name: &str) -> Result<(), String> {
    let _lock = acquire_lock(&autorotate_lock())?;
    let mut sensor_dir = iio::find_accelerometer();
    println!(
        "[autorotate] Starting ThinkPad auto-rotation daemon on {:?} (output={})...",
        sensor_dir, output_name
    );

    let initial_tr = get_niri_transform(output_name);
    let mut tracker = tracker::OrientationTracker::new(&initial_tr);

    loop {
        thread::sleep(Duration::from_millis(350));

        // Check ACPI laptop mode
        if !tablet_mode::is_tablet_mode() {
            if tracker.current_transform != "normal" {
                println!(
                    "[autorotate] Switched to Laptop mode. Locking to normal."
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
                sensor_dir = iio::find_accelerometer();
                continue;
            }
        };

        let target_tr = tracker.update(accel);
        if target_tr != get_niri_transform(output_name) {
            set_niri_transform(&target_tr, output_name);
        }
    }
}
