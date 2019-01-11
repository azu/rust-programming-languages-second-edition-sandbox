extern crate iterator_minigrep;

use std::env;
use std::process;

use iterator_minigrep::Config;
use iterator_minigrep::run;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    let run_result = run(config);
    if let Err(e) = run_result {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
