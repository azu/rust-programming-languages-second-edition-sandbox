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
