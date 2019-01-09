# 構造体

- 構造体はオブジェクトみたいなもの
- `..struct`でマージもできる
- インターフェースとクラス

## タプル構造体

- フィールドはないけど、構造体
- インターフェース的な感じ

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
```

構造体に`String`を使ってるのはデータの保持しやすさのため。説明的