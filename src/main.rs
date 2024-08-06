use clock::Clock;

mod clock;
mod gigasecond;
mod reverse_string;

fn main() {
    let c = Clock::new(1, 1).add_minutes(3500);
    println!("{}", c);
}
