use screaming_float::s64;
use num_traits::Float;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("sqrt(10.0) =");
    println!("  {}\n", s64(10.0).sqrt());
    sleep(Duration::from_secs(2));
    println!("sqrt(0.5) =");
    println!("  {}\n", s64(0.5).sqrt());
    sleep(Duration::from_secs(2));
    println!("sqrt(-1.0) =");
    println!("  {}", s64(-1.0).sqrt());
}
