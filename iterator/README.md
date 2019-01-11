# イテレータ

反復処理を行う、そのシーケンスが終わったことを決定するロジックの責任を持つ

## [Iteratorとnext](https://doc.rust-jp.rs/book/second-edition/ch13-02-iterators.html#iterator%E3%83%88%E3%83%AC%E3%82%A4%E3%83%88%E3%81%A8next%E3%83%A1%E3%82%BD%E3%83%83%E3%83%89)

`Iterator`トレレトを実装すれば列挙できる。
`type Item`と`Self::Item`は関連型というトレイとの関係性