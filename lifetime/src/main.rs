struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
//    {
//        let r;
//        {
//            let x = 6;
//            r = &x;
//        }
//        // r は上記のスコープを抜けた時に落ちている
//        // これが借用精査機(borrow checker)
//        // println!("r: {}", r);
//    }
    {
        let string1 = String::from("abcd");
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        // 最長の文字列は、{}です
        println!("The longest string is {}", result);
    }

    {
        let string1 = String::from("long string i slog");
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
    {
        // 僕をイシュマエルとお呼び。何年か前・・・
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.')
            .next()
            .expect("Could not find a '.'");  // '.'が見つかりませんでした
        let i = ImportantExcerpt { part: first_sentence };
    }
}

// xとyどっちが長生きかわからない = 返り値はランダム
// なのでxとyが'aで生きることを保証する構文
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

