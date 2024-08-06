use core::fmt;
#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

#[allow(dead_code)]
impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut m = minutes % 60;
        let mut h = 0;

        if m.is_negative() {
            m += 60;
            h -= 1;
        }

        h += (minutes / 60) + hours;
        h %= 24;

        if h.is_negative() {
            h += 24;
        }

        Self {
            hours: h,
            minutes: m,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        if minutes == 0 {
            return Self {
                minutes: self.minutes,
                hours: self.hours,
            };
        }
        let total_minutes = minutes + self.hours * 60 + self.minutes;
        let mut m = total_minutes % 60;
        let mut h = (total_minutes / 60) % 24;

        if total_minutes.is_negative() {
            m += 60;
            h += 24;
            h -= 1;
        }

        Self {
            hours: h,
            minutes: m,
        }
    }
}
impl fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.minutes == other.minutes && self.hours == other.hours
    }
}
