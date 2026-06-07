use chrono::{DateTime, Utc};

pub fn whole_minutes_between(start: DateTime<Utc>, end: DateTime<Utc>) -> i64 {
    (end - start).num_minutes().max(0)
}
