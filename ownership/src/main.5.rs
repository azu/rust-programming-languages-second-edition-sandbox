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

