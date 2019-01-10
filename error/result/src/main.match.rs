use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("./hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("作成できない {:?}", error);
                }
            }
        },
        Err(error) => {
            panic!("Open Error {:?}", error);
        }
    };
    println!("{:?}", f);
}
