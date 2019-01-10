# エラー処理

[エラー処理 - The Rust Programming Language](https://doc.rust-jp.rs/book/second-edition/ch09-00-error-handling.html)

- `Result<T, E>` 回復可能なエラー
- `panic!` 回復不能なエラー

## panic

パニック時には異常終了するには設定を変更するとできる

```
[profile.release]
panic = 'abort'
```

## `Result`

ResultはEnum


```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

## FS

`?`はErrorを自動的にReturnする構文。
Fromトレイトで定義されている。

- 成功ならそのファイル
- 失敗ならその関数でエラーをReturnする

```rust
    let f = File::open("hello.txt")?;
    /*
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    */
```


- `?`演算子は、Resultを返す関数でしか使用できない
