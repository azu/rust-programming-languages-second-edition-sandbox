enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let two = plus_one(Some(2));
    // matchの省略ができる
    // matchに対してdefaultケースがなくてもエラーにならいあ
    if let Some(3) = two {
        println!("Yay");
    }

    let coin = Coin::Dime;
    if let Coin::Dime = coin {
        println!("DimeDime");
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    return match x {
        None => None,
        Some(i) => Some(i + 1)
    };
}