use std::fs;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

pub const WACOM_VID: &str = "056a";
pub const WACOM_PID: &str = "51af";
pub const FALLBACK_PORT: &str = "/sys/bus/usb/devices/1-6";

pub fn find_wacom_device() -> Option<PathBuf> {
    if let Ok(entries) = fs::read_dir("/sys/bus/usb/devices") {
        for entry in entries.flatten() {
            let path = entry.path();
            let vid_path = path.join("idVendor");
            let pid_path = path.join("idProduct");

            if vid_path.is_file() && pid_path.is_file() {
                if let (Ok(vid), Ok(pid)) =
                    (fs::read_to_string(&vid_path), fs::read_to_string(&pid_path))
                {
                    if vid.trim() == WACOM_VID && pid.trim() == WACOM_PID {
                        return Some(path);
                    }
                }
            }
        }
    }

    let fallback = Path::new(FALLBACK_PORT);
    if fallback.is_dir() {
        return Some(fallback.to_path_buf());
    }

    None
}

pub fn reset_wacom() -> Result<(), String> {
    let target = find_wacom_device().ok_or_else(|| {
        "Wacom USB device (056a:51af) not found in /sys/bus/usb/devices".to_string()
    })?;

    let auth_path = target.join("authorized");
    if !auth_path.is_file() {
        return Err(format!(
            "Device {:?} does not expose an 'authorized' control attribute",
            target
        ));
    }

    eprintln!(
        "[wacom-reset] Resetting Wacom USB device at {:?} on resume...",
        target
    );

    fs::write(&auth_path, "0\n").map_err(|e| {
        format!(
            "Failed to de-authorize Wacom device at {:?}: {}",
            auth_path, e
        )
    })?;

    thread::sleep(Duration::from_millis(500));

    fs::write(&auth_path, "1\n").map_err(|e| {
        format!(
            "Failed to re-authorize Wacom device at {:?}: {}",
            auth_path, e
        )
    })?;

    eprintln!("[wacom-reset] Wacom USB device re-authorized successfully.");
    Ok(())
}
