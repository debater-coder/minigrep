use std::{env, process};

use minigrep::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if config.help {
        print_help()
    }

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1)
    };
}

fn print_help() {
    println!(
        "Search for text in files

Usage: minigrep <query> <file> [OPTIONS]

Options:
--ignore-case      Performs a case-insensitive search
--case-sensitive   Performs a case-sensitive search

Case insensitivity can be enabled by setting the IGNORE_CASE environment variable. 
The case options passed to minigrep will override these.
"
    );
    process::exit(0)
}
