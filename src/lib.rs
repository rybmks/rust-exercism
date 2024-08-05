pub mod gigasecond {
    use std::ops::Add;
    use time::Duration;
    use time::PrimitiveDateTime as DateTime;

    pub fn after(start: DateTime) -> DateTime {
        const DURATION: time::Duration = Duration::new(1000000000, 0);
        start.add(DURATION)
    }
    println!("{input}!");
}
