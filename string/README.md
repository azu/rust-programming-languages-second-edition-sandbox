# 文字列

文字列はコレクション

- Rustの文字列型は1つ
- `str` 文字列スライス
- これは別の場所に保持されたUTF-8エンコードされた文字列データへの参照
- `String`型は言語のコアじゃなくて標準ライブラリ
- 可変、所有権のUTF-8エンコードされた文字列型
- Rustにおいての「文字列」は`String`
- 他にも`OsString`や`OsStr`や`CSString`、`CsStr`などがある

## 文字列の生成

```rust
let mut s = String::new()
```