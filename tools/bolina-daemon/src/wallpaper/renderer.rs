use crate::common::{cache_dir, get_magick_cmd, wallpaper_base};
use crate::wallpaper::dms::{get_dms_current_wallpaper, set_dms_wallpaper};
use crate::wallpaper::niri::get_niri_transform;
use crate::wallpaper::self_heal::ensure_base_wallpaper;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub fn normalize_orientation(raw: &str) -> &str {
    let s = raw.trim().to_lowercase();
    let s = s.trim_start_matches('_');
    match s {
        "90" => "90",
        "180" => "180",
        "270" => "270",
        _ => "normal",
    }
}

pub fn get_wallpaper_path(set_name: &str, orientation: &str) -> PathBuf {
    let norm = normalize_orientation(orientation);
    cache_dir().join(format!("task_wallpaper_{}_{}.png", set_name, norm))
}

pub fn get_current_set() -> &'static str {
    let current_wp = get_dms_current_wallpaper();
    if current_wp.contains("task_wallpaper_b_") {
        "b"
    } else {
        "a"
    }
}

pub fn get_next_set() -> &'static str {
    if get_current_set() == "a" {
        "b"
    } else {
        "a"
    }
}

pub fn cleanup_inactive_sets(active_set: &str) {
    let inactive_set = if active_set == "a" { "b" } else { "a" };
    let prefix = format!("task_wallpaper_{}_", inactive_set);
    let cache = cache_dir();

    if let Ok(entries) = fs::read_dir(&cache) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.starts_with(&prefix) && name.ends_with(".png") {
                    let _ = fs::remove_file(path);
                }
            }
        }
    }
}

pub fn prerender_all(set_name: &str) -> Result<(), String> {
    if !ensure_base_wallpaper() {
        return Err("Cannot pre-render: base wallpaper and xopp missing".to_string());
    }

    let cache = cache_dir();
    fs::create_dir_all(&cache).map_err(|e| e.to_string())?;
    let magick = get_magick_cmd();
    let base = wallpaper_base();

    println!(
        "[wallpaper] Pre-rendering all 4 orientations (1:1 native 1920x1080/1080x1920) for set '{}' in cache...",
        set_name
    );

    // 1. Normal (0° landscape, 1920x1080)
    let norm_file = get_wallpaper_path(set_name, "normal");
    let norm_tmp = norm_file.with_extension("tmp.png");
    let status = Command::new(&magick)
        .args([
            base.to_str().unwrap(),
            "-resize",
            "1920x1080^",
            "-gravity",
            "center",
            "-extent",
            "1920x1080",
            "-define",
            "png:color-type=6",
            norm_tmp.to_str().unwrap(),
        ])
        .status()
        .map_err(|e| format!("ImageMagick normal failed: {}", e))?;

    if !status.success() {
        return Err(format!("ImageMagick normal exited with status {:?}", status));
    }
    fs::rename(&norm_tmp, &norm_file).map_err(|e| e.to_string())?;

    // 2. Tent mode (180: readable upright landscape, 1920x1080) -> identical to normal
    let tent_file = get_wallpaper_path(set_name, "180");
    fs::copy(&norm_file, &tent_file).map_err(|e| e.to_string())?;

    // 3. Portrait 90° (+90° clockwise, 1080x1920)
    let p90_file = get_wallpaper_path(set_name, "90");
    let p90_tmp = p90_file.with_extension("tmp.png");
    let status = Command::new(&magick)
        .args([
            base.to_str().unwrap(),
            "-rotate",
            "90",
            "-resize",
            "1080x1920^",
            "-gravity",
            "center",
            "-extent",
            "1080x1920",
            "-define",
            "png:color-type=6",
            p90_tmp.to_str().unwrap(),
        ])
        .status()
        .map_err(|e| format!("ImageMagick 90° failed: {}", e))?;

    if !status.success() {
        return Err(format!("ImageMagick 90° exited with status {:?}", status));
    }
    fs::rename(&p90_tmp, &p90_file).map_err(|e| e.to_string())?;

    // 4. Portrait 270° (-90° clockwise, 1080x1920)
    let p270_file = get_wallpaper_path(set_name, "270");
    let p270_tmp = p270_file.with_extension("tmp.png");
    let status = Command::new(&magick)
        .args([
            base.to_str().unwrap(),
            "-rotate",
            "-90",
            "-resize",
            "1080x1920^",
            "-gravity",
            "center",
            "-extent",
            "1080x1920",
            "-define",
            "png:color-type=6",
            p270_tmp.to_str().unwrap(),
        ])
        .status()
        .map_err(|e| format!("ImageMagick 270° failed: {}", e))?;

    if !status.success() {
        return Err(format!("ImageMagick 270° exited with status {:?}", status));
    }
    fs::rename(&p270_tmp, &p270_file).map_err(|e| e.to_string())?;

    println!(
        "[wallpaper] Set '{}' ready with all 4 pre-rendered 1:1 orientations.",
        set_name
    );
    Ok(())
}

pub fn apply_wallpaper(
    target_transform: Option<&str>,
    force_set: Option<&str>,
    output_name: &str,
) -> Result<(), String> {
    if !ensure_base_wallpaper() {
        return Err("Cannot apply wallpaper: base wallpaper missing".to_string());
    }

    let raw_transform = match target_transform {
        Some(t) => t.to_string(),
        None => get_niri_transform(output_name),
    };
    let norm_transform = normalize_orientation(&raw_transform);

    let active_set = match force_set {
        Some("a") => "a",
        Some("b") => "b",
        _ => get_current_set(),
    };

    let target_file = get_wallpaper_path(active_set, norm_transform);
    if !target_file.is_file() {
        prerender_all(active_set)?;
    }

    let dms_result = set_dms_wallpaper(&target_file)?;
    println!(
        "[wallpaper] Set to {:?} (transform={}, dms: {})",
        target_file, norm_transform, dms_result
    );

    cleanup_inactive_sets(active_set);
    Ok(())
}
