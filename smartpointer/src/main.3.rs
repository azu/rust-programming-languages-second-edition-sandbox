use std::ops::Deref;

fn main() {}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Derefトレイトがunboxのキーとなる実装
// Deref = dereference
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

#[test]
fn test_box() {
    let x = 5;
    // Box化
    let y = MyBox::new(x);

    assert_eq!(5, x);
    // Boxの中身を参照する
    assert_eq!(5, *y);
}