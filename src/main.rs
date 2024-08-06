use time::{Date, Month, PrimitiveDateTime, Time};

mod gigasecond;
mod reverse_string;

fn main() {
    let dt = PrimitiveDateTime::new(
        Date::from_calendar_date(2015, Month::January, 24).unwrap(),
        Time::from_hms(22, 0, 0).unwrap(),
    );
    println!("{}", gigasecond::after(dt))
}
