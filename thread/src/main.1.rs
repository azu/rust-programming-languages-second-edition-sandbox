use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    // joinでスレッドの終了を待つ
    handle.join().unwrap();

    for i in 1..10 {
        println!("hi number {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }
}
