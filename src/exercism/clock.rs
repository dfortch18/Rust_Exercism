use std::fmt;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            hours: (hours + minutes.div_euclid(60)).rem_euclid(24),
            minutes: minutes.rem_euclid(60)
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
