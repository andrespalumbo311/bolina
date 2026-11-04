#[allow(deprecated)]
use nix::fcntl::{flock, FlockArg};
use std::fs::{self, File, OpenOptions};
use std::os::unix::io::AsRawFd;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct FileLock {
    _file: File,
}

#[allow(deprecated)]
pub fn acquire_lock(lock_path: &Path) -> Result<FileLock, String> {
    if let Some(parent) = lock_path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let file = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .truncate(false)
        .open(lock_path)
        .map_err(|e| format!("Failed to open lock file {:?}: {}", lock_path, e))?;

    flock(file.as_raw_fd(), FlockArg::LockExclusiveNonblock)
        .map_err(|_| "Another instance is already running.".to_string())?;

    Ok(FileLock { _file: file })
}

pub fn home_dir() -> PathBuf {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/root"))
}

pub fn cache_dir() -> PathBuf {
    home_dir().join(".cache").join("niri").join("wallpaper")
}

pub fn pictures_dir() -> PathBuf {
    home_dir().join("Pictures")
}

pub fn xopp_file() -> PathBuf {
    pictures_dir().join("task_wallpaper.xopp")
}

pub fn wallpaper_base() -> PathBuf {
    cache_dir().join("task_wallpaper_base.png")
}

pub fn autorotate_lock() -> PathBuf {
    home_dir().join(".cache").join("niri").join("auto-rotate.lock")
}

pub fn watcher_lock() -> PathBuf {
    cache_dir().join("wallpaper_watcher.lock")
}

pub fn get_magick_cmd() -> String {
    let candidates = [
        "/home/linuxbrew/.linuxbrew/bin/magick",
        "/usr/bin/magick",
        "magick",
        "/home/linuxbrew/.linuxbrew/bin/convert",
        "/usr/bin/convert",
        "convert",
    ];

    for candidate in candidates {
        if Path::new(candidate).is_file() {
            return candidate.to_string();
        }
        if Command::new("which")
            .arg(candidate)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
        {
            return candidate.to_string();
        }
    }
    "convert".to_string()
}
