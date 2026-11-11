use std::fs::{self, File};
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn strip_block(text: &str, block_name: &str) -> String {
    let mut lines = Vec::new();
    let mut in_block = false;
    let mut depth: i32 = 0;

    for line in text.lines() {
        let trimmed = line.trim_start();
        if !in_block {
            if let Some(rest) = trimmed.strip_prefix(block_name) {
                if rest.is_empty() || rest.starts_with(|c: char| c.is_whitespace() || c == '{') {
                    if trimmed.contains('{') {
                        in_block = true;
                        depth = 0;
                        for ch in trimmed.chars() {
                            if ch == '{' {
                                depth += 1;
                            } else if ch == '}' {
                                depth -= 1;
                            }
                        }
                        if depth == 0 {
                            in_block = false;
                        }
                        continue;
                    } else {
                        // Single-line token like "trackpoint"
                        continue;
                    }
                }
            }
        }

        if in_block {
            for ch in line.chars() {
                if ch == '{' {
                    depth += 1;
                } else if ch == '}' {
                    depth -= 1;
                }
            }
            if depth <= 0 {
                in_block = false;
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

pub fn strip_keyboard_block(text: &str) -> String {
    strip_block(text, "keyboard")
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
    let active_file = config_dir.join("input-active.kdl");

    if !input_file.is_file() {
        return Ok(false);
    }

    let mut dms_content = fs::read_to_string(&input_file)
        .map_err(|e| format!("Failed to read {:?}: {}", input_file, e))?;

    let new_content = if hw_file.is_file() {
        // Strip any residual hardware blocks that may have been previously injected into input.kdl
        dms_content = strip_block(&dms_content, "touch");
        dms_content = strip_block(&dms_content, "tablet");
        dms_content = strip_block(&dms_content, "trackpoint");

        let mut hw_content = fs::read_to_string(&hw_file)
            .map_err(|e| format!("Failed to read {:?}: {}", hw_file, e))?;

        // Avoid duplicate keyboard block if DMS input.kdl already contains keyboard configuration
        if dms_content.contains("keyboard {") {
            hw_content = strip_keyboard_block(&hw_content);
        }

        // Avoid duplicate focus-follows-mouse if DMS input.kdl already contains it
        if dms_content.contains("focus-follows-mouse") {
            hw_content = strip_focus_follows_mouse(&hw_content);
        }

        let last_brace = match dms_content.rfind('}') {
            Some(idx) => idx,
            None => return Ok(false),
        };

        format!(
            "{}{}\n}}\n",
            &dms_content[..last_brace],
            hw_content.trim_end()
        )
    } else {
        dms_content
    };

    // If input-active.kdl already exists and has identical content, skip redundant writes and IPC reload
    if active_file.is_file() {
        if let Ok(existing) = fs::read_to_string(&active_file) {
            if existing == new_content {
                return Ok(false);
            }
        }
    }

    // Atomic write via temporary file in the same directory
    let temp_file = config_dir.join(format!(".input-active.kdl.tmp.{}", std::process::id()));
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
    fs::rename(&temp_file, &active_file)
        .map_err(|e| format!("Failed to replace input-active.kdl: {}", e))?;

    // Reload niri configuration via IPC
    let _ = Command::new("niri")
        .args(["msg", "action", "load-config-file"])
        .status();

    println!("[dms-input-sync] Successfully synchronized input into input-active.kdl");
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_block_keyboard() {
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
        let stripped = strip_block(hw_kdl, "keyboard");
        assert!(!stripped.contains("keyboard"));
        assert!(!stripped.contains("de_custom"));
        assert!(stripped.contains("touch {"));
        assert!(stripped.contains("tablet {"));
    }

    #[test]
    fn test_strip_block_touch_preserves_touchpad() {
        let text = r#"
input {
    touchpad {
        tap
        accel-speed 0.4
    }
    touch {
        map-to-output "eDP-1"
    }
}
"#;
        let stripped = strip_block(text, "touch");
        assert!(!stripped.contains("touch {"));
        assert!(!stripped.contains("map-to-output"));
        assert!(stripped.contains("touchpad {"));
        assert!(stripped.contains("accel-speed 0.4"));
    }

    #[test]
    fn test_strip_block_single_line_trackpoint() {
        let text = r#"
input {
    trackpoint
    mouse {
        accel-speed 0.0
    }
}
"#;
        let stripped = strip_block(text, "trackpoint");
        assert!(!stripped.contains("trackpoint"));
        assert!(stripped.contains("mouse {"));
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

    #[test]
    fn test_sync_input_kdl_integration() {
        let test_dir = std::env::temp_dir().join(format!("bolina-test-input-sync-{}", std::process::id()));
        let _ = fs::create_dir_all(&test_dir);
        let config_dir = &test_dir;

        let dms_kdl = r#"input {
    focus-follows-mouse max-scroll-amount="0%"
    mouse {
        accel-speed 0.0
    }
    touchpad {
        tap
    }
}
"#;
        let hw_kdl = r#"    touch {
        map-to-output "eDP-1"
    }
    tablet {
        map-to-output "eDP-1"
    }
"#;
        fs::write(config_dir.join("input.kdl"), dms_kdl).unwrap();
        fs::write(config_dir.join("input-hardware.kdl"), hw_kdl).unwrap();

        let updated = sync_input_kdl(config_dir).unwrap();
        assert!(updated);

        let active_file = config_dir.join("input-active.kdl");
        assert!(active_file.is_file());
        let active_content = fs::read_to_string(&active_file).unwrap();
        assert!(active_content.contains("touchpad {"));
        assert!(active_content.contains("map-to-output \"eDP-1\""));

        // Running again without changes should return Ok(false)
        let updated_again = sync_input_kdl(config_dir).unwrap();
        assert!(!updated_again);

        let _ = fs::remove_dir_all(&test_dir);
    }
}
