use std::path::Path;
use std::process::Command;

pub fn get_dms_current_wallpaper() -> String {
    let output = Command::new("dms")
        .args(["ipc", "call", "wallpaper", "get"])
        .output();

    match output {
        Ok(out) if out.status.success() => {
            String::from_utf8_lossy(&out.stdout).trim().to_string()
        }
        _ => String::new(),
    }
}

pub fn set_dms_wallpaper(path: &Path) -> Result<String, String> {
    let output = Command::new("dms")
        .args(["ipc", "call", "wallpaper", "set", path.to_str().unwrap_or("")])
        .output()
        .map_err(|e| format!("Failed to invoke dms ipc: {}", e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(format!(
            "DMS IPC call failed with code {}: {}",
            output.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}
