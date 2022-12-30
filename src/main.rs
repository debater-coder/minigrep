use std::{env, process};

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        if &args[1] == "--help" {
            print_help();
            process::exit(0);
        }
    }

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

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
}
