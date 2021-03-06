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
fn first_word_length(s: &str) -> &str {
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

