use time::Duration;
use time::PrimitiveDateTime as DateTime;

const GIGASECOND: time::Duration = Duration::new(1_000_000_000, 0);
pub fn after(start: DateTime) -> DateTime {
    start + GIGASECOND
}
