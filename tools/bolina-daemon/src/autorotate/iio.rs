use std::fs;
use std::path::{Path, PathBuf};

pub fn find_accelerometer() -> Option<PathBuf> {
    let mut candidates = Vec::new();

    if let Ok(entries) = fs::read_dir("/sys/bus/iio/devices") {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(name_str) = path.file_name().and_then(|n| n.to_str()) {
                if name_str.starts_with("iio:device") {
                    let name_path = path.join("name");
                    let raw_x = path.join("in_accel_x_raw");
                    if raw_x.is_file() && name_path.is_file() {
                        if let Ok(name) = fs::read_to_string(&name_path) {
                            if name.contains("accel") {
                                candidates.push(path);
                            }
                        }
                    }
                }
            }
        }
    }

    candidates.sort();

    if candidates.is_empty() {
        return None;
    }

    // Prefer screen sensor on X390 Yoga (HID-SENSOR-200073.12.auto)
    for dev in &candidates {
        if let Ok(canon) = fs::canonicalize(dev) {
            if canon.to_string_lossy().contains("HID-SENSOR-200073.12.auto") {
                return Some(dev.clone());
            }
        }
    }

    Some(candidates[0].clone())
}

pub fn find_base_accelerometer() -> Option<PathBuf> {
    if let Ok(entries) = fs::read_dir("/sys/bus/iio/devices") {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Ok(canon) = fs::canonicalize(&path) {
                if canon.to_string_lossy().contains("HID-SENSOR-200073.17.auto") {
                    return Some(path);
                }
            }
        }
    }
    None
}

pub fn read_accel_raw(sensor_dir: &Path) -> Option<(i64, i64, i64)> {
    let x_str = fs::read_to_string(sensor_dir.join("in_accel_x_raw")).ok()?;
    let y_str = fs::read_to_string(sensor_dir.join("in_accel_y_raw")).ok()?;
    let z_str = fs::read_to_string(sensor_dir.join("in_accel_z_raw")).ok()?;

    let x = x_str.trim().parse::<i64>().ok()?;
    let y = y_str.trim().parse::<i64>().ok()?;
    let z = z_str.trim().parse::<i64>().ok()?;

    Some((x, y, z))
}
