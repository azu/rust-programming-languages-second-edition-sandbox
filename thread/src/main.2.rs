use std::thread;

fn main() {
    let v = vec![1, 2, 3];
    // クロージャーへ所有権を渡す
    let handle = thread::spawn(move || {
        println!("v is {:?}", v);
    });
    handle.join().unwrap();
}
