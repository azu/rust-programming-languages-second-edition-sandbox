#![allow(unused_variables)]
fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    /*
     - i8/u8: 8bit
     - i16/u16: 16bit
     - i32/u32: 32bit
     - i64/u64: 64bit
     - isize: arch
    */

    // 32, 64 倍精度
    let x = 2.0;
    let y: f32 = 3.0;

    // 数値演算
    let sum = 5 + 10;

    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    // 論理値
    let t = true;
    let f: bool = false;
    // 文字列型
    // char型? = ユニコードのスカラーチ
    let c = "z";
    let z = "Z";
    let heart = "❤️";
    // 複合型
    // - タプル型
    // - 配列
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // タプルから値をパターンマッチ的に取り出す
    let (x, y, z) = tup;
    println!("x is {}", x);
    // タブルからインデックスで取り出す
    let five = tup.0;
    // 配列
    // 一度定義したサイズを変更できない
    // ベクター型はサイズを変更できる
    let a = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
    let jan = months[0];

    println!("month 0 {} ", jan);
    // 無効なインデックスへのアクセス
    let i = 22; // variable経由だとRuntimeエラーになる = パニック
    let element = months[i];

}
