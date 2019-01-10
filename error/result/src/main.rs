use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    // 簡単panic
    let f = File::open("./hello.txt").expect("開けなかった");
    println!("{:?}", f);
    let f = read_username_from_file();
    println!("{:?}", f);
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt")?;
    /*
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    */
    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}