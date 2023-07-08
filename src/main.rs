extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments {}", err); //eprintln!() prints to standard error
        process::exit(1);
    }); //use pattern for error handling code that has a return value

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application Error {}", e); //eprintln!() prints to standard error

        process::exit(1);
    } //use pattern when error handling code that is only ran for side effects
}
