use std::fs;
use std::path::Path;

pub const THINKPAD_TABLET_MODE: &str = "/sys/devices/platform/thinkpad_acpi/hotkey_tablet_mode";

pub fn is_tablet_mode() -> bool {
    let path = Path::new(THINKPAD_TABLET_MODE);
    if path.is_file() {
        if let Ok(content) = fs::read_to_string(path) {
            return content.trim() != "0";
        }
    }
    // Fallback: if sysfs node is absent, assume tablet capability is active
    true
}
