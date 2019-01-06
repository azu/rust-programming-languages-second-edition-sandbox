use std::io;

fn main() {
    println!("Guess the number!");
    // input
    println!("Please input your guess.");

    // Rustはデフォルトでimmutableなのでmutでmutableにしている
    let mut guess = String::new();
    // &は参照を表す記号 = コピーしない
    let result = io::stdin().read_line(&mut guess);
    // InputのResultを返して、Resultは列挙型
    // expectはErrの時にクラッシュさせる
    /* expectしてない場合はwarningがでる = エラー処理しないといけない
      --> src/main.rs:11:5
   |
11 |     io::stdin().read_line(&mut guess);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: #[warn(unused_must_use)] on by default
   = note: this `Result` may be an `Err` variant, which should be handled
   */
    result.expect("Failed to read line");
    // {} は プレースホルダー
    println!("You guessed: {}", guess);
}
