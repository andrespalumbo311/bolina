use crate::common::{cache_dir, wallpaper_base, xopp_file};
use std::fs;
use std::process::Command;

pub fn ensure_base_wallpaper() -> bool {
    let base = wallpaper_base();
    if base.is_file() {
        return true;
    }

    let xopp = xopp_file();
    if !xopp.is_file() {
        return false;
    }

    let cache = cache_dir();
    let _ = fs::create_dir_all(&cache);

    eprintln!(
        "[wallpaper] Base image missing. Self-healing from {:?}...",
        xopp
    );

    let create_arg = format!("--create-img={}", base.to_str().unwrap_or(""));
    let status = Command::new("flatpak")
        .args([
            "run",
            "com.github.xournalpp.xournalpp",
            &create_arg,
            xopp.to_str().unwrap_or(""),
        ])
        .status();

    match status {
        Ok(s) if s.success() => {}
        Ok(s) => {
            eprintln!(
                "[wallpaper] Xournal++ self-heal export returned {}",
                s.code().unwrap_or(-1)
            );
            return false;
        }
        Err(e) => {
            eprintln!("[wallpaper] Failed to execute flatpak xournalpp: {}", e);
            return false;
        }
    }

    let page1 = cache.join("task_wallpaper_base-1.png");
    if page1.is_file() {
        let _ = fs::rename(&page1, &base);
    }

    // Clean up any extra pages
    if let Ok(entries) = fs::read_dir(&cache) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.starts_with("task_wallpaper_base-") && name.ends_with(".png") {
                    let _ = fs::remove_file(&path);
                }
            }
        }
    }

    base.is_file()
}
