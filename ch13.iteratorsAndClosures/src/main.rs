#![allow(unused)]
fn main() {
    use std::thread;
    use std::time::Duration;
    fn simulated_expensive_calculation(intensity: u32)->u32{
        println!("calculating slowly...");
        thread::sleep(Duration::fron_secs(2));
        intensity
    }
}
