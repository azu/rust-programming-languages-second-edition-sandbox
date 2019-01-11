use std::thread;
use std::time::Duration;

fn main() {
    println!("calculating slowly...");
    // Duration::from_secs(2)は関数を返している
    thread::sleep(Duration::from_secs(2));
    println!("calculating done");
}
