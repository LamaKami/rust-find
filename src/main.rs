mod config;

use find::{search_for_file};
use std::{env, process};
use crate::config::Config;

/*

-p : path from where to search, if not provided then searching form current
-s : is substring, if not provided then searching whole string
-d : depth search (default)
-w : wide search

Usage:
fd query -p path/path1
fd query -s

*/


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    search_for_file(config.query, config.path, config.is_substring);

}
