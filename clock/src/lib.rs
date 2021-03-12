use std::fmt;

// easy to derive as all the canonization happens in the constructor
#[derive(Debug, PartialEq, Eq)]
pub struct Clock(i32);

const HOUR: i32 = 60;
const DAY: i32 = 24 * HOUR;

impl Clock {
    pub fn new(h: i32, m: i32) -> Self {
        // Wrapping and handling negatives here
        // to get the simple derivable binary Eq.
        Self((m + h * HOUR).rem_euclid(DAY))
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(0, self.0 + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.0 / HOUR, self.0 % HOUR)
    }
}
