fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let large = largest(&number_list);
    println!("{}", large);
}

// Copy: cannot move out of type [T], a non-copy slice
// Tが既知のサイズだと、スタックにコピーできるのでCopyを実装してないとrefをmoveしようとする
// ヒープ確保しちゃうので、スタックで解決用にする
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &number in list.iter() {
        if number > largest {
            largest = number;
        }
    }
    return largest;
}

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest number is x = {}", self.x);
        } else {
            println!("The largest number is y = {}", self.y);
        }
    }
}
