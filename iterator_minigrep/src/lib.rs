use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let path = env::current_dir();
    println!("The current directory is {}", path.unwrap().display());
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let mut f = File::open(config.filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something wen wrong reading the file");

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    for line in results {
        println!("{}", line);
    }
    // () はvoidみたいなもの
    Ok(())
}

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    // iteratorそのものを受け取って消費していくことで、cloneをなくせる
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Does not get query string")
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Does not get filename")
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        return Ok(Config {
            query,
            filename,
            case_sensitive,
        });
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    return contents.lines()
        .filter(|line| line.contains(query))
        .collect();
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        // Rustは
        // 安全で速く生産性も高い。
        // 3つ選んで。
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
// Rust
// 安全かつ高速で生産的
// 三つを選んで
// ガムテープ
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
// (最後の行のみ)
// 私を信じて
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}