// easy to derive as all the canonization happens in the constructor
#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // the goal was to keep everything immutable
        let hours = (hours + minutes / 60) % 24;
        let minutes = minutes % 60;
        let (minutes, hours) = if minutes < 0 {
            (minutes + 60, hours - 1)
        } else {
            (minutes, hours)
        };
        let hours = if hours < 0 { hours + 24 } else { hours };
        Self { hours, minutes }
    }

    pub fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.hours, self.minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hours, self.minutes + minutes)
    }
}
