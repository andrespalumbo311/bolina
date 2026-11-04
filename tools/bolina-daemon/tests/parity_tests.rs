use bolina_daemon_lib::autorotate::tracker::OrientationTracker;
use bolina_daemon_lib::input_sync::kdl_merger::strip_keyboard_block;

#[test]
fn test_tracker_complete_rotation_lifecycle() {
    let mut tracker = OrientationTracker::new("normal");

    // 1. Initial flat on table -> stays normal
    let flat = (10_000, 20_000, 980_000);
    assert_eq!(tracker.update(flat), "normal");

    // 2. Tilted on lap (sub-threshold tilt) -> stays normal
    let lap = (50_000, 150_000, 850_000); // |z|/ns > 0.80
    assert_eq!(tracker.update(lap), "normal");

    // 3. User rotates tablet into Portrait 90°
    let p90 = (880_000, 100_000, 200_000);
    assert_eq!(tracker.update(p90), "normal"); // Debounce cycle 1
    assert_eq!(tracker.update(p90), "90");     // Debounce cycle 2 -> switches to 90!

    // 4. User lays tablet down flat on table while in portrait mode
    // Crucial ThinkPad behavior: table flat-lock MUST FREEZE the portrait mode!
    let flat_table = (15_000, 10_000, 990_000);
    assert_eq!(tracker.update(flat_table), "90");
    assert_eq!(tracker.update(flat_table), "90");

    // 5. User picks up tablet and flips into Tent mode (180°)
    let tent = (80_000, 920_000, 180_000);
    assert_eq!(tracker.update(tent), "90");
    assert_eq!(tracker.update(tent), "180");

    // 6. User rotates to Portrait 270° (other side)
    let p270 = (-900_000, 80_000, 150_000);
    assert_eq!(tracker.update(p270), "180");
    assert_eq!(tracker.update(p270), "270");

    // 7. ACPI switch: User opens laptop mode, triggering forced "normal"
    tracker.set_forced("normal");
    assert_eq!(tracker.current_transform, "normal");
    assert_eq!(tracker.update(flat), "normal");
}

#[test]
fn test_kdl_merger_logic() {
    let hw_kdl = r#"
touch {
    map-to-output "eDP-1"
}

tablet {
    map-to-output "eDP-1"
}

keyboard {
    xkb {
        layout "de_custom"
    }
}
"#;

    let stripped = strip_keyboard_block(hw_kdl);
    assert!(!stripped.contains("keyboard"));
    assert!(!stripped.contains("de_custom"));
    assert!(stripped.contains("map-to-output \"eDP-1\""));
    assert!(stripped.contains("touch {"));
    assert!(stripped.contains("tablet {"));
}
