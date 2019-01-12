fn main() {
    // ヒープに i32の値が保存される
    let b = Box::new(5);
    println!("b = {}", b);
} // bはここで警報される
