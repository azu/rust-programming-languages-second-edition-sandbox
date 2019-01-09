fn main() {
    let mut s = String::from("test");
    change(&mut s);
    println!("{}", s);
}

fn change(s: &mut String) {
    s.push_str("-test?");
}