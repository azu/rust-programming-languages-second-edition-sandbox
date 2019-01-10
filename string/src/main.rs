fn main() {
    // 文字列はバイトの塊
    // Stringとコレクション
    {
        let s = String::new();
        println!("{}", s);
    }
    // 文字列リテラルからStringを作成する
    {
        let data = "initial contents";
        let s = data.to_string();
        println!("{}", s);
    }
    // String::fromでも同じ
    {
        let s = String::from("initial contens");
        println!("{}", s);
    }
    // 文字列はコレクションなので可変できる
    {
        let mut s = String::from("foo");
        s.push_str("added");
        println!("{}", s);
    }
    // push_str
    {
        let mut s = String::from("foo");
        let s2 = "bar";
        s.push_str(s2);
        println!("{}", s);
    }
    // + で結合
    {
        let s1 = String::from("Hello, ");
        let s2 = String::from("World!");
        // これはs1の所有権がmoveされる
        // これは s1.add(s2)だから s2の所有権を奪う
        let s3 = s1 + &s2;
        println!("+ {}", s3)
    }
    // formatで結合
    {
        let s1 = String::from("Hello, ");
        let s2 = String::from("World!");
        println!("format {}", format!("{}-{}", s1, s2))
    }
    // インデックス
    {
        let s1 = String::from("hello");
        // これはサポートしてない
        // let h = s1[0];
    }
    // len はバイトの長さをかえす
    {
        let len = String::from("Здравствуйте").len();
        println!("len {}", len); // => 24
    }
}
