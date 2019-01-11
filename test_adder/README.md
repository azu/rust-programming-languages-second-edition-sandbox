# Assert

- `assert` マクロ
- `assert_eq` と `assert_ne`

## テストをバイナリから除外

```
#[cfg(test)]
```
をつければいい

## 結合テスト

- Rustのテストはファイル内に書ける
- src全体をテストするなら`tests`ディレクトリをつくてやる