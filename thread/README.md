# スレッド

- グリーンスレッド: ソフトウェアスレッド
- ノーランタイム =? 小さいランタイム

## `spawn`

- スレッドで実行するのは `||` でいける
- 値を渡すには 所有権を `move` する

## データのやり取り

- [メッセージ受け渡し - The Rust Programming Language](https://doc.rust-jp.rs/book/second-edition/ch16-02-message-passing.html)

- tx: sender
- rx: receiver

```rust
let (tx, rx) = mpsc::channel();
```

受け取る時: 受け取る終わるまで進まない

```rust
for recived in rx {
    println!("Got {}", recived);
}
println!("End");
```

## 状態の共有

Memory Shared

- ミューテックスで相互排他アクセス(mutual exclusion)を行う