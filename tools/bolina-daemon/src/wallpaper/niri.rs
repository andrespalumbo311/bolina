use serde_json::Value;
use std::process::Command;

pub fn get_niri_transform(output_name: &str) -> String {
    let output = match Command::new("niri").args(["msg", "-j", "outputs"]).output() {
        Ok(out) if out.status.success() => out.stdout,
        _ => return "normal".to_string(),
    };

    let data: Value = match serde_json::from_slice(&output) {
        Ok(v) => v,
        Err(_) => return "normal".to_string(),
    };

    let raw_tr = data
        .get(output_name)
        .and_then(|o| o.get("logical"))
        .and_then(|l| l.get("transform"))
        .and_then(|t| t.as_str())
        .unwrap_or("Normal");

    let norm = raw_tr.trim().to_lowercase();
    let norm = norm.trim_start_matches('_');

    match norm {
        "normal" | "90" | "180" | "270" => norm.to_string(),
        _ => "normal".to_string(),
    }
}
