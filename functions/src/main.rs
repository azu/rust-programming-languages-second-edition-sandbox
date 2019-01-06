fn main() {
    let y = {
        let x = 3;
        x + 1 // 式の場合は;をつけない
    };
    println!("y is {}", y);

    another_function(5, 42);

    let five = five();
    println!("five is {}", five);
    let six = plus_one(five);
    println!("five + 1 is {}", six);

}

fn another_function(x: i32, y: i32) {
    println!("{}", x);
    println!("{}", y);
}

fn five() -> i32 {
    5 // これも式
}


fn plus_one(x: i32) -> i32 {
    x + 1
    // x+1; だと() 空のタブルを返す事になっている
}