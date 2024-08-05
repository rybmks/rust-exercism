use gigasecond::after;
use rust_exercism::gigasecond;
use time::PrimitiveDateTime;
use time_macros::date;
use time_macros::time;
fn main() {
    let dt = PrimitiveDateTime::new(date!(2015 - 01 - 24), time!(22:00));
    print!("{}", after(dt))
}
