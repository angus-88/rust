use std::{env, process};

use minigrep::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");

        // Return some default config
        // return Config {
        //     query: String::from("query"),
        //     file_path: String::from("defualt"),
        // };

        // Or
        process::exit(1);
    });


    // Not using `unwrap_or_else` as we don't want to unwrap an empty return if okay
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}


