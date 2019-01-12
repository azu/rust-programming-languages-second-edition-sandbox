use std::sync::Mutex;

fn main() {
    // Mutexはスマートポインタ
    // lockの呼び出しがMutexGuardというスマートポインタを返す
    let mutex = Mutex::new(5);
    {
        // スレッドをブロックしてロックを獲得
        // 獲得できなかったときはErrになる
        let mut num = mutex.lock().unwrap();
        *num = 6;
    }
    println!("mutex: {:?}", mutex);
}