enum List {
    // ListがListを再帰する最大のListのデータサイズは無限
    // 間接参照にする = ポインタを格納することでデータサイズが決まる
    // Boxはポインタを作る => ポインタを参照することになる
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};


fn main() {
    let list = Box::new(Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil)))))));
}
