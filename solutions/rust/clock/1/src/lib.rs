use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut new_hours = hours;
        let mut new_minutes = minutes;

        new_hours = (new_hours + new_minutes / 60) % 24;
        new_minutes %= 60;

        if new_minutes < 0 {
            new_minutes += 60;
            new_hours = (new_hours - 1 + 24) % 24;
        }
        if new_hours < 0 {
            new_hours += 24;
        }
        Clock { hours: new_hours, minutes: new_minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let minutes_from_midnight = self.to_minutes() + minutes;
        let new_hours = (minutes_from_midnight / 60) % 24;
        let new_minutes = minutes_from_midnight % 60;
        Clock::new(new_hours, new_minutes)
    }

    fn to_minutes (&self) -> i32 {
        self.hours * 60 + self.minutes
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
