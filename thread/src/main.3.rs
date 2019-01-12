use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    // チャネルを作成する
    // txは送信側、rxは受信側
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        // 送信側
        let val = String::from("hi");
        tx.send(val).unwrap();
        thread::sleep(Duration::from_millis(1));
        // これは届かない?
        let val = String::from("hi_");
        tx.send(val).unwrap();
    });
    // 受信側
    // recv = receive
    let received = rx.recv().unwrap();
    // 値は{}です
    println!("Got: {}", received);
}
