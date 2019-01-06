// randクレートの外部依存しているマーク
extern crate rand;

use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");
    // input
    println!("Please input your guess number.");

    // 乱数を取得 1 - 100
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
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

        // guessを数値に変換
        // シャドーイングによって変数名を再利用できる
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        // {} は プレースホルダー
        println!("You guessed: {}", guess);

        // マッチングする
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),       //小さすぎ！
            Ordering::Greater => println!("Too big!"),      //大きすぎ！
            Ordering::Equal => {
                println!("You win!");
                break;
            }        //やったね！
        }
    }
}
