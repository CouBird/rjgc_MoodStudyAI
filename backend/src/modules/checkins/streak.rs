use chrono::NaiveDate;

pub fn is_consecutive(previous: NaiveDate, current: NaiveDate) -> bool {
    current.signed_duration_since(previous).num_days() == 1
}
