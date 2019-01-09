# 所有権

- [所有権とは？ - The Rust Programming Language](https://doc.rust-jp.rs/book/second-edition/ch04-01-what-is-ownership.html)

## スタックとヒープ

スタックはFIFOな構造で、push,popな構造なので、アクセスが早い

ヒープは可変サイズを格納するためにallocateする必要がある。

> テーブルAで注文を受け、 それからテーブルBの注文、さらにまたA、それからまたBと渡り歩くのは、かなり低速な過程になってしまうでしょう。 同じ意味で、プロセッサは、 データが隔離されている(ヒープではそうなっている可能性がある)よりも近くにある(スタックではこうなる)ほうが、 仕事をうまくこなせるのです。ヒープに大きな領域を確保する行為も時間がかかることがあります。

ポインターの旅をする必要があるのでヒープは遅くなる

## 所有権のルール

- Rustの各値は、所有者と呼ばれる変数と対応している。
- いかなる時も所有者は一つである。
- 所有者がスコープから外れたら、値は破棄される。

所有者 = 変数 は1対1、所有者がスコープから抜けると値は破棄される

## String型

所有者の話はスタックよりもヒープのような複雑なところで話がおきる


文字列リテラルはハードコードなので、そのままバイナリに書き込まれる。
`String::from`はユーザー入力からヒープにメモリを確保して`String`型を作る。
ユーザー入力をうける場合は、そうじゃないのでヒープに内容を保持する。

```rust
let mut s = String::from("hello");
// Stringは可変
s.push_str(", world!");
println!("{}", s);
```

## ムーブ

他の言語だとこれはコピーにも見える。

```rust
let s1 = String::from("hello");
let s2 = s1;
```

- s1 -> * -> Hello
- s2 -> * -> Hello

Rustはスコープを抜けたらその変数の値を開放する。
s1とs2が同じ値を参照してるなら、2重開放してしまう

実際は所有権がムーブしてるので、コンパイルエラーとなる。

`s2 = s1`の時点で、s1は無効化されている。なので2重開放されない

```
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:5:28
  |
3 |     let s2 = s1;
  |              -- value moved here
4 | 
5 |     println!("{}, world!", s1);
  |                            ^^ value borrowed here after move
  |
  = note: move occurs because `s1` has type `std::string::String`, which does not implement the `Copy` trait
```

コピーしたい場合はクローンすればOK

```rust
let s1 = String::from("hello");
let s2 = s1.clone();
println!("s1 = {}, s2 = {}", s1, s2);
```


ヒープじゃなくてスタック = リテラルの場合は、これがコピーとして動く。
つまり自動的に`clone`される。これはRustの`Copy`トレイとという実装によって挙動が違う。

```rust
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);
```

次の型には`Copy`トレイドが実装されている



    あらゆる整数型。u32など。
    論理値型、bool、trueとfalseという値がある。
    あらゆる浮動小数点型、f64など。
    文字型、char。
    タプル。ただ、Copyの型だけを含む場合。例えば、(i32, i32)はCopyだが、 (i32, String)は違う。


## 所有権と関数
   
同じような話は代入だけじゃなくて関数呼び出しでも起きる

- 関数へ渡す = その引数は関数へ所有権がmoveされる
- 関数が値を返す = 関数から呼び出し元へ所有権が渡る
- 入力をそのまま出力にする = 関数の呼び出しと返り値は同じなので所有権はいじできる
    - これは設計として微妙なので、参照を使う

```rust
fn main() {
    let s1 = gives_ownership();
    // gives_ownershipは、戻り値をs1に
    // ムーブする
    let s2 = String::from("hello");     // s2がスコープに入る

    // s2はmoveしてる
    let s3 = takes_and_gives_back(s2);  // s2はtakes_and_gives_backにムーブされ
    // 戻り値もs3にムーブされる
    // s2だけはmoveされて帰ってきてない
    println!("{} {}", s1, s3)
} // ここで、s3はスコープを抜け、ドロップされる。s2もスコープを抜けるが、ムーブされているので、
// 何も起きない。s1もスコープを抜け、ドロップされる。

fn gives_ownership() -> String {             // gives_ownershipは、戻り値を
    // 呼び出した関数にムーブする

    let some_string = String::from("hello"); // some_stringがスコープに入る

    some_string                              // some_stringが返され、呼び出し元関数に
    // ムーブされる
}

// takes_and_gives_backは、Stringを一つ受け取り、返す。
fn takes_and_gives_back(a_string: String) -> String { // a_stringがスコープに入る。

    a_string  // a_stringが返され、呼び出し元関数にムーブされる
}

```

## [参照と借用](https://doc.rust-jp.rs/book/second-edition/ch04-02-references-and-borrowing.html)

moveするんじゃなくて一時的に貸す = 参照を渡す `&` <-> 参照外し `&`

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    // '{}'の長さは、{}です
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

> 関数の引数に参照を取ることを借用と呼びます。


`s: &String`は`Stringを借用してる

借用した値はデフォルトでは変更できない、。
参照もデフォルトimmubtaleなので。

## 可変な参照

これは`mut`な`&`を定義すればいいだけ

`&mut`で変更可能な参照を定義できる

```rust
fn main() {
    let mut s = String::from("test");
    change(&mut s);
    println!("{}", s);
}

fn change(s: &mut String) {
    s.push_str("-test?");
}
```

いつでも可変な参照を作れるわけじゃない。
不変な参照 + 可変な参照 の組み合わせはNG。
型定義的な意味じゃなくて値的に可変な可能性はコンパイルエラー
```rust

let mut s = String::from("hello");

let r1 = &s; // 問題なし
let r2 = &s; // 問題なし
let r3 = &mut s; // 大問題！

```

つぎのように参照をかえしてもスコープを抜ければ値そのものが消える。
つまり `&s`は無を参照 = ダングリング参照 となるためコンパイルエラー。

```rust

fn main() {
    // これはライフタイムエラー
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}

```


- 任意のタイミングで、一つの可変参照か不変な参照いくつでものどちらかを行える。
- 参照は常に有効でなければならない。


## [スライス](https://doc.rust-jp.rs/book/second-edition/ch04-03-slices.html)

所有権のない別のデータ型 = スライス

次の例は`s`の一部として`word_length`がある。
けど`s`を消したあとも`word_length`は参照できる。

これをどうにかするのがスライス

```rust

fn main() {
    let mut s = String::from("hello world");
    let word_length = first_word_length(&s);
    println!("{}", word_length);
    println!("{}", s);
    s.clear();
    // sは消えてもwordはまだ参照できてる
    // wordはsの一部をとってるはずなのに => 文字列スライスでかいけつ
    println!("{}", word_length);
}



fn first_word_length(s: &String) -> usize {
    // バイトへ変換
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        // b" " 空白バイトがあったら、indexを返す
        if item == b' ' {
            return i;
        }
    }
    // 空白バイトがない = sの長さが文字の長さ
    s.len()
}

```

### 文字列スライス

Stringの一部を参照する。`開始..終点`

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

色々スライスできる

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[0..len];
let slice = &s[..];
```

`&str`はこの文字列スライスを意味する型。
スライスを返すときは `-> &str`みたいになる。

さっきの例は文字列スライスを返すようにすれば、`s`のスライスである`word_length`も自動的に消える

```rust
fn main() {
    let mut s = String::from("hello world");
    let word_length = first_word_length(&s);
    println!("{}", word_length.len());
    println!("{}", s);
    s.clear();
    // sは消えたらword_lengthは参照できなくなる
    // println!("{}", word_length);
}



// &strは文字列スライス
fn first_word_length(s: &String) -> &str {
    // バイトへ変換
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        // b" " 空白バイトがあったら、indexを返す
        if item == b' ' {
            return &s[0..i];
        }
    }
    // 空白バイトがない = sの長さが文字の長さ
    &s[..]
}

```

`&str`を受け取るようにすれば文字列と文字列スライスどっちも受け取れる


配列にもスライスがある、。