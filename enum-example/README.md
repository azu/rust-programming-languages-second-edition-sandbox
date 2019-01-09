# 列挙

enumで列挙

enumは構造体と違って各要素で別の型を定義できる

```rust

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct QuitMessage; // ユニット構造体
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // タプル構造体
struct ChangeColorMessage(i32, i32, i32); // タプル構造体
```

`enum`も`impl`でメソッドを実装できる

```
#[derive(Debug)]
enum IpAddrKind {
    // enumはそれぞれにTuple的なデータ型を指定できる
    // 構造体はできないらしい
    V4(u8, u8, u8, u8),
    V6(String),
}
impl IpAddrKind {
    fn call(&self) {
        return self;
    }
}
```

## Option

nullの問題を解決する`enum` - `Option<T>`

`Option<T>`は優遇されているので初期化処理(prelude)に含まれていて

`Option::Some`を`Some`、`None`とかけるようになっている