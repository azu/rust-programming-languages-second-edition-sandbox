# エラー処理

[エラー処理 - The Rust Programming Language](https://doc.rust-jp.rs/book/second-edition/ch09-00-error-handling.html)

- `Result<T, E>` 回復可能なエラー
- `panic!` 回復不能なエラー

パニック時には異常終了するには設定を変更するとできる

```
[profile.release]
panic = 'abort'
```