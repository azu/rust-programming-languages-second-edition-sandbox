fn main() {
    let number = 3;
    if number < 5 {
        println!("success");
    } else {
        println!("failure");
    }
    // 暗黙的な変換はしない
    // if number {}
    // if式

    let condition = true;
    let number = if condition {
        5
    } else {
        10
    };
    println!("number is {}", number);
    // 異なる型はを混ぜるのは無理、変数の型は単独でないといけない

    // ループ
    loop {
        println!("again");
        break;
    }
    // while
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
    // for in
    let array = [1, 2, 3, 4, 5];
    // forループはもっとも使う
    for element in array.iter() {
        print!("{}", element);
    }
    // Range型を使う
    for number in (1..5).rev() {
        println!("{}", number);
    }
}
