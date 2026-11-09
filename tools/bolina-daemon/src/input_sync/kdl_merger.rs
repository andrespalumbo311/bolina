use std::fs::{self, File};
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn strip_keyboard_block(text: &str) -> String {
    let mut lines = Vec::new();
    let mut in_keyboard = false;
    let mut depth = 0;

    // Search for line starting with optional whitespace + keyboard + optional whitespace + {
    for line in text.lines() {
        let trimmed = line.trim_start();
        if !in_keyboard && trimmed.starts_with("keyboard") && trimmed.contains('{') {
            in_keyboard = true;
            depth = 0;
            for ch in trimmed.chars() {
                if ch == '{' {
                    depth += 1;
                } else if ch == '}' {
                    depth -= 1;
                }
            }
            if depth == 0 {
                in_keyboard = false;
            }
            continue;
        }

        if in_keyboard {
            for ch in line.chars() {
                if ch == '{' {
                    depth += 1;
                } else if ch == '}' {
                    depth -= 1;
                }
            }
            if depth <= 0 {
                in_keyboard = false;
            }
            continue;
        }

        lines.push(line);
    }

    let mut result = lines.join("\n");
    if text.ends_with('\n') {
        result.push('\n');
    }
    result
}

pub fn strip_focus_follows_mouse(text: &str) -> String {
    let mut lines = Vec::new();
    for line in text.lines() {
        if line.trim_start().starts_with("focus-follows-mouse") {
            continue;
        }
        lines.push(line);
    }
    let mut result = lines.join("\n");
    if text.ends_with('\n') {
        result.push('\n');
    }
    result
}

pub fn default_config_dir() -> PathBuf {
    crate::common::home_dir().join(".config").join("niri").join("dms")
}

pub fn sync_input_kdl(config_dir: &Path) -> Result<bool, String> {
    let input_file = config_dir.join("input.kdl");
    let hw_file = config_dir.join("input-hardware.kdl");

    if !input_file.is_file() || !hw_file.is_file() {
        return Ok(false);
    }

    let content = fs::read_to_string(&input_file)
        .map_err(|e| format!("Failed to read {:?}: {}", input_file, e))?;

    // Avoid duplicate injection
    if content.contains("map-to-output") {
        return Ok(false);
    }

    let mut hw_content = fs::read_to_string(&hw_file)
        .map_err(|e| format!("Failed to read {:?}: {}", hw_file, e))?;

    // Avoid duplicate keyboard block if input.kdl already contains keyboard configuration
    if content.contains("keyboard {") {
        hw_content = strip_keyboard_block(&hw_content);
    }

    // Avoid duplicate focus-follows-mouse if input.kdl already contains it
    if content.contains("focus-follows-mouse") {
        hw_content = strip_focus_follows_mouse(&hw_content);
    }

    let last_brace = match content.rfind('}') {
        Some(idx) => idx,
        None => return Ok(false),
    };

    let new_content = format!(
        "{}{}\n}}\n",
        &content[..last_brace],
        hw_content.trim_end()
    );

    // Atomic write via temporary file in the same directory
    let temp_file = config_dir.join(format!(".input.kdl.tmp.{}", std::process::id()));
    {
        let mut file = File::create(&temp_file)
            .map_err(|e| format!("Failed to create temp file: {}", e))?;
        file.write_all(new_content.as_bytes())
            .map_err(|e| format!("Failed to write temp file: {}", e))?;
        file.flush().map_err(|e| format!("Failed to flush: {}", e))?;
        file.sync_all()
            .map_err(|e| format!("Failed to fsync: {}", e))?;
    }

    let _ = fs::set_permissions(&temp_file, fs::Permissions::from_mode(0o644));
    fs::rename(&temp_file, &input_file)
        .map_err(|e| format!("Failed to replace input.kdl: {}", e))?;

    // Reload niri configuration via IPC
    let _ = Command::new("niri")
        .args(["msg", "action", "load-config-file"])
        .status();

    println!("[dms-input-sync] Successfully synchronized input-hardware.kdl into input.kdl");
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_keyboard_block() {
        let hw_kdl = r#"
touch {
    map-to-output "eDP-1"
}
keyboard {
    xkb {
        layout "de_custom"
    }
}
tablet {
    map-to-output "eDP-1"
}
"#;
        let stripped = strip_keyboard_block(hw_kdl);
        assert!(!stripped.contains("keyboard"));
        assert!(!stripped.contains("de_custom"));
        assert!(stripped.contains("touch {"));
        assert!(stripped.contains("tablet {"));
    }

    #[test]
    fn test_strip_focus_follows_mouse() {
        let hw_kdl = r#"
touch {
    map-to-output "eDP-1"
}
focus-follows-mouse max-scroll-amount="0%"
tablet {
    map-to-output "eDP-1"
}
"#;
        let stripped = strip_focus_follows_mouse(hw_kdl);
        assert!(!stripped.contains("focus-follows-mouse"));
        assert!(stripped.contains("touch {"));
        assert!(stripped.contains("tablet {"));
    }
}
