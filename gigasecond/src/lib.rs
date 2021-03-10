use chrono::{DateTime, Duration, Utc};

const ONE_BILLION: i64 = 1_000_000_000;

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    // The overloaded `+` does both:
    // adds safely and unwraps with a meaningful message.
    start + Duration::seconds(ONE_BILLION)
}
