use minigrep::Config;
use std::{env, process};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|error| {
        eprintln!("Problem parsing arguments: {error}");
        process::exit(1);
    });

    if let Err(error) = minigrep::run(config) {
        eprintln!("Applikation error: {error}");
        process::exit(1);
    }
}
