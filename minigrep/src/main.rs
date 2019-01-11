extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;
use minigrep::run;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    let run_result = run(config);
    if let Err(e) = run_result {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
