#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 構造体に対してメソッドを定義する
// メソッドは必ず最初の引数がselfとなる
impl Rectangle {
    // ここでも&でborrowする
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // 他のrectよりも大きい?
    fn can_hold_other_rect(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
// implブロックは分けてかける
impl Rectangle {
    // 関連関数は&selfを取らないメソッドのこと
    // これは::を使って呼び出す。
    // つまりファクトリ
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle::square(10);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("rect1  > rect2 is {}", rect1.can_hold_other_rect(&rect2));
}
