// debugをつけるとPrint :?でき利用になる
// これはDebugトレイとの継承駐車k
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };

    // 四角形の面積は、{}平方ピクセルです
    println!(
        "The area of the rectangle is {} square pixels.",
        // &で渡しているのは、所有権を借用させるため
        // &にしかなった場合は、これ移行rectを使えなくなる
        area(&rect)
    );

    println!("Rect {:#?}", rect);
}

// &Rectangleで所有権を借用する
fn area(rect: &Rectangle) -> u32 {
    return rect.width * rect.height;
}