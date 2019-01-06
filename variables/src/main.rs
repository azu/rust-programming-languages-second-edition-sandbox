fn main() {
    const MAX_POINTS: u32 = 100_100;
    let mut x = 5;
    println!("This value x is {}", x);
    x = 6;
    println!("This value x is {}", x);

    // シャドーイングで型違いもいける
    let spaces = "    ";
    let spaces = spaces.len();

    println!("MAX_POINTS {}", MAX_POINTS);
    println!("space len {}", spaces);
}
