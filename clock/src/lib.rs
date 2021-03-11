// easy to derive as all the canonization happens in the constructor
#[derive(Debug, PartialEq, Eq)]
pub struct Clock(i32);

const HOUR: i32 = 60;
const DAY: i32 = 24 * HOUR;

impl Clock {
    pub fn new(h: i32, m: i32) -> Self {
        // Wrapping and handling negatives here
        // to get the simple derivable binary Eq.
        let m = (m + h * HOUR) % DAY;
        Self(if m < 0 { m + DAY } else { m })
    }

    pub fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.0 / HOUR, self.0 % HOUR)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(0, self.0 + minutes)
    }
}
