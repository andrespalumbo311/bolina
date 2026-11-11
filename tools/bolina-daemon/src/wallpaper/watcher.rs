use crate::common::{
    acquire_lock, cache_dir, pictures_dir, wallpaper_base, watcher_lock, xopp_file,
};
use crate::wallpaper::renderer::{apply_wallpaper, get_next_set, prerender_all};
use inotify::{Inotify, WatchMask};
use std::fs;
use std::process::Command;
use std::thread;
use std::time::Duration;

pub fn process_xopp_update(output_name: &str) {
    let xopp = xopp_file();
    println!("[watcher] Detected update to {:?}. Processing...", xopp);
    thread::sleep(Duration::from_millis(400));

    let cache = cache_dir();
    let _ = fs::create_dir_all(&cache);
    let base = wallpaper_base();

    // Clean old temp pages
    if let Ok(entries) = fs::read_dir(&cache) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.starts_with("task_wallpaper_base-") && name.ends_with(".png") {
                    let _ = fs::remove_file(path);
                }
            }
        }
    }

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
                "[watcher] Xournal++ export failed with code: {:?}",
                s.code()
            );
            return;
        }
        Err(e) => {
            eprintln!("[watcher] Error running flatpak export: {}", e);
            return;
        }
    }

    let page1 = cache.join("task_wallpaper_base-1.png");
    if page1.is_file() {
        let _ = fs::rename(&page1, &base);
    }

    // Clean extra pages
    if let Ok(entries) = fs::read_dir(&cache) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.starts_with("task_wallpaper_base-") && name.ends_with(".png") {
                    let _ = fs::remove_file(path);
                }
            }
        }
    }

    println!(
        "[watcher] Export completed. Pre-rendering orientation pool in cache..."
    );
    let next_set = get_next_set();
    if let Err(e) = prerender_all(next_set) {
        eprintln!("[watcher] Error pre-rendering set {}: {}", next_set, e);
    }
    if let Err(e) = apply_wallpaper(None, Some(next_set), output_name) {
        eprintln!("[watcher] Error applying wallpaper: {}", e);
    }
}

pub fn run_edit() -> Result<(), String> {
    let xopp = xopp_file();
    if let Some(parent) = xopp.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let desktop_entry = "/var/lib/flatpak/exports/share/applications/com.github.xournalpp.xournalpp.desktop";
    let status = if std::path::Path::new(desktop_entry).is_file() {
        Command::new("gio")
            .args(["launch", desktop_entry, xopp.to_str().unwrap_or("")])
            .spawn()
    } else {
        Command::new("flatpak")
            .args([
                "run",
                "com.github.xournalpp.xournalpp",
                xopp.to_str().unwrap_or(""),
            ])
            .spawn()
    };

    match status {
        Ok(_) => Ok(()),
        Err(e) => {
            // Fallback to xdg-open
            Command::new("xdg-open")
                .arg(xopp.to_str().unwrap_or(""))
                .spawn()
                .map_err(|e2| format!("Failed to launch Xournal++ editor: {} / {}", e, e2))?;
            Ok(())
        }
    }
}

pub fn run_watcher(output_name: &str) -> Result<(), String> {
    let _lock = acquire_lock(&watcher_lock())?;
    let pictures = pictures_dir();
    let _ = fs::create_dir_all(&pictures);
    if !pictures.is_dir() {
        return Err(format!("Directory missing: {:?}", pictures));
    }

    let xopp = xopp_file();
    println!("[watcher] Starting watcher for {:?}...", xopp);

    if xopp.is_file() && !wallpaper_base().is_file() {
        process_xopp_update(output_name);
    }

    let mut inotify = Inotify::init().map_err(|e| format!("Failed to initialize inotify: {}", e))?;
    inotify
        .watches()
        .add(&pictures, WatchMask::CLOSE_WRITE | WatchMask::MOVED_TO)
        .map_err(|e| format!("Failed to watch {:?}: {}", pictures, e))?;

    let mut buffer = [0u8; 4096];
    loop {
        match inotify.read_events_blocking(&mut buffer) {
            Ok(events) => {
                for event in events {
                    if let Some(name) = event.name {
                        if name.to_string_lossy() == "task_wallpaper.xopp" {
                            process_xopp_update(output_name);
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("[watcher] Inotify error: {}. Retrying in 2s...", e);
                thread::sleep(Duration::from_secs(2));
            }
        }
    }
}
