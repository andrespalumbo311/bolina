pub struct OrientationTracker {
    pub current_transform: String,
    pub pending_transform: Option<String>,
    pub pending_count: usize,
    pub debounce_threshold: usize,
}

impl OrientationTracker {
    pub fn new(initial: &str) -> Self {
        Self {
            current_transform: initial.to_string(),
            pending_transform: None,
            pending_count: 0,
            debounce_threshold: 2, // 2 consecutive cycles (~0.7s)
        }
    }

    pub fn set_forced(&mut self, tr: &str) {
        self.current_transform = tr.to_string();
        self.pending_transform = None;
        self.pending_count = 0;
    }

    pub fn update(&mut self, s: (i64, i64, i64)) -> String {
        let (x, y, z) = (s.0 as f64, s.1 as f64, s.2 as f64);
        let ns = (x * x + y * y + z * z).sqrt();
        if ns < 100_000.0 {
            return self.current_transform.clone();
        }

        let norm_x = x / ns;
        let norm_y = y / ns;
        let norm_z = z.abs() / ns;

        // 1. Table Flat-Lock: if tilted < 37° from horizontal (|z| > 0.80), freeze orientation!
        if norm_z > 0.80 {
            self.pending_transform = None;
            self.pending_count = 0;
            return self.current_transform.clone();
        }

        let cur = self.current_transform.as_str();
        let is_portrait = cur == "90" || cur == "270";
        let is_landscape = cur == "normal" || cur == "180";

        const RATIO: f64 = 1.28;
        const MIN_FORCE: f64 = 0.40;

        let desired: &str = if is_portrait {
            // To switch to landscape, Y must decisively dominate X
            if norm_y.abs() > RATIO * norm_x.abs() && norm_y.abs() > MIN_FORCE {
                if norm_y < 0.0 {
                    "normal"
                } else {
                    "180"
                }
            } else if norm_x > MIN_FORCE && cur == "270" {
                "90"
            } else if norm_x < -MIN_FORCE && cur == "90" {
                "270"
            } else {
                cur
            }
        } else if is_landscape {
            // To switch to portrait, X must decisively dominate Y
            if norm_x.abs() > RATIO * norm_y.abs() && norm_x.abs() > MIN_FORCE {
                if norm_x > 0.0 {
                    "90"
                } else {
                    "270"
                }
            } else if norm_y > MIN_FORCE && cur == "normal" {
                "180"
            } else if norm_y < -MIN_FORCE && cur == "180" {
                "normal"
            } else {
                cur
            }
        } else {
            if norm_x.abs() > norm_y.abs() {
                if norm_x > 0.0 {
                    "90"
                } else {
                    "270"
                }
            } else if norm_y < 0.0 {
                "normal"
            } else {
                "180"
            }
        };

        if desired == cur {
            self.pending_transform = None;
            self.pending_count = 0;
            return self.current_transform.clone();
        }

        // Temporal debounce filter
        if let Some(ref pending) = self.pending_transform {
            if pending == desired {
                self.pending_count += 1;
                if self.pending_count >= self.debounce_threshold {
                    self.current_transform = desired.to_string();
                    self.pending_transform = None;
                    self.pending_count = 0;
                    return self.current_transform.clone();
                }
            } else {
                self.pending_transform = Some(desired.to_string());
                self.pending_count = 1;
            }
        } else {
            self.pending_transform = Some(desired.to_string());
            self.pending_count = 1;
        }

        self.current_transform.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_flat_lock() {
        let mut tracker = OrientationTracker::new("normal");
        // Accelerometer flat on table: z dominates (|z| / ns > 0.80)
        let s = (50_000, 50_000, 990_000);
        let res = tracker.update(s);
        assert_eq!(res, "normal");
        assert!(tracker.pending_transform.is_none());
    }

    #[test]
    fn test_debounce_transition_to_portrait_90() {
        let mut tracker = OrientationTracker::new("normal");
        // Rotate to portrait (x positive dominates decisively)
        let s = (950_000, 100_000, 200_000);

        // Cycle 1: pending count becomes 1, transform remains normal
        let res1 = tracker.update(s);
        assert_eq!(res1, "normal");
        assert_eq!(tracker.pending_transform.as_deref(), Some("90"));
        assert_eq!(tracker.pending_count, 1);

        // Cycle 2: pending count reaches 2 (debounce threshold), transform flips to 90
        let res2 = tracker.update(s);
        assert_eq!(res2, "90");
        assert!(tracker.pending_transform.is_none());
    }

    #[test]
    fn test_tent_mode_transition_180() {
        let mut tracker = OrientationTracker::new("normal");
        // Tent mode: y positive dominates
        let s = (100_000, 950_000, 200_000);

        tracker.update(s);
        let res = tracker.update(s);
        assert_eq!(res, "180");
    }
}
