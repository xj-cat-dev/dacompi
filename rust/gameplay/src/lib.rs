use crate::fii::CooldownResult;

#[cxx::bridge(namespace = "wow::gameplay")]
mod fii {
    struct CooldownResult {
        active: bool,
        start: u32,
        duration: u32,
    }

    struct CooldownEntry {
        spell_id: i32,
        item_id: i32,
        time: u32,
        recovery_time: u32,
        category: i32,
        category_time: u32,
        category_recovery_time: u32,
        on_hold: bool,
        start_recovery_category: i32,
        start_recovery_time: u32,
    }

    extern "Rust" {
        fn ends_at_or_after(candidate: u32, latest: u32) -> bool;
        fn query_cooldown(now: u32, start: u32, duration: u32) -> CooldownResult;

        type CooldownHistory;
        fn new_cooldown_history() -> Box<CooldownHistory>;
        fn add_history(self: &mut CooldownHistory, entry: CooldownEntry);
    }
}

pub struct CooldownHistory {
    entries: Vec<fii::CooldownEntry>,
}

impl CooldownHistory {
    pub fn add_history(&mut self, entry: fii::CooldownEntry) {
        if entry.recovery_time == 0
            && entry.category_recovery_time == 0
            && !entry.on_hold
            && entry.start_recovery_time == 0
        {
            return;
        }
        self.entries.push(entry);
    }
}

pub fn new_cooldown_history() -> Box<CooldownHistory> {
    Box::new(CooldownHistory {
        entries: Vec::new(),
    })
}

pub fn query_cooldown(now: u32, start: u32, duration: u32) -> fii::CooldownResult {
    let end = start.wrapping_add(duration);

    if duration == 0 || !ends_at_or_after(end, now) {
        return CooldownResult {
            active: false,
            start: 0,
            duration: 0,
        };
    }
    CooldownResult {
        active: end != now,
        start,
        duration,
    }
}

pub fn ends_at_or_after(candidate: u32, latest: u32) -> bool {
    (candidate.wrapping_sub(latest) as i32) >= 0
}

#[cfg(test)]
mod tests {
    use crate::ends_at_or_after;

    #[test]
    fn ends_at_or_after_test() {
        assert!(ends_at_or_after(1200, 1000));
        assert!(!ends_at_or_after(800, 1000));
        assert!(ends_at_or_after(1000, 1000));
        assert!(ends_at_or_after(4, u32::MAX - 5));
    }

    #[test]
    fn query_active_cooldown() {
        let result = super::query_cooldown(3000, 1000, 5000);

        assert!(result.active);
        assert_eq!(result.start, 1000);
        assert_eq!(result.duration, 5000);
    }

    #[test]
    fn query_at_exact_expiry() {
        let result = super::query_cooldown(6000, 1000, 5000);

        assert!(!result.active);
        assert_eq!(result.start, 1000);
        assert_eq!(result.duration, 5000);
    }

    #[test]
    fn add_history_preserves_duplicate_order() {
        let mut history = super::new_cooldown_history();

        let make_entry = |time: u32| super::fii::CooldownEntry {
            spell_id: 1,
            item_id: 2,
            time,
            recovery_time: 5000,
            category: 0,
            category_time: 0,
            category_recovery_time: 0,
            on_hold: false,
            start_recovery_category: 0,
            start_recovery_time: 0,
        };

        history.add_history(make_entry(2000));
        history.add_history(make_entry(1000));

        assert_eq!(history.entries.len(), 2);
        assert_eq!(history.entries[0].time, 2000);
        assert_eq!(history.entries[1].time, 1000);
    }

    #[test]
    fn add_history_keeps_held_entry_without_durations() {
        let mut history = super::new_cooldown_history();

        history.add_history(super::fii::CooldownEntry {
            spell_id: 1,
            item_id: 2,
            time: 1000,
            recovery_time: 0,
            category: 0,
            category_time: 1000,
            category_recovery_time: 0,
            on_hold: true,
            start_recovery_category: 0,
            start_recovery_time: 0,
        });

        assert_eq!(history.entries.len(), 1);
        assert!(history.entries[0].on_hold);
    }

    #[test]
    fn add_history_handles_duration_sources() {
        let cases = [
            ("no cooldown", 0, 0, 0),
            ("category cooldown", 5000, 0, 1),
            ("start recovery", 0, 1500, 1),
        ];

        for (name, category_duration, start_duration, expected_count) in cases {
            let mut history = super::new_cooldown_history();

            history.add_history(super::fii::CooldownEntry {
                spell_id: 1,
                item_id: 2,
                time: 1000,
                recovery_time: 0,
                category: 1,
                category_time: 1000,
                category_recovery_time: category_duration,
                on_hold: false,
                start_recovery_category: 133,
                start_recovery_time: start_duration,
            });

            assert_eq!(history.entries.len(), expected_count, "{name}");
        }
    }
}
