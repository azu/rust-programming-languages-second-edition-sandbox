use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

fn main() {
    // 複数のスレッドで所有権を共有したい = Rc or Arc
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // スレッドを大量に作ってる = counterがmoveされてる = コピーしないと?
        // or 複数スレッドで複数の所有権
        let counter = Arc::clone(&counter);
        // このRcでも所有権は共有できたけど、スレッドセーフじゃない
        // ArcでAtomicな所有権の共有する = Rcよりは遅いけど安全
        let handle = thread::spawn(move || {
            // mutexに対して値を書き込んでいく
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    // すべてのスレッド処理が終わるまで待つ
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}