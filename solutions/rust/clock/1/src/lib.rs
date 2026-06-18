#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = hours * 60 + minutes;
        let normalized_minutes = (total_minutes % 1440 + 1440) % 1440;

        Clock {
            hours: normalized_minutes / 60,
            minutes: normalized_minutes % 60,
        }
    }
    
    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut total_minutes = self.hours * 60 + self.minutes;
        total_minutes += minutes;
        let normalized_minutes = (total_minutes % 1440 + 1440) % 1440;

        Clock {
            hours: normalized_minutes / 60,
            minutes: normalized_minutes % 60,
        }
    }
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
