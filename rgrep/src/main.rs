extern crate regex;

use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // skip exec file
    let args: Vec<String> = env::args().skip(1).collect();

    match args.len() {
        1 => help("args is missing"),
        2 => rsgrep(&args[0], &args[1]),
        _ => help("args is too much"),
    }
}

fn rsgrep(pattern: &String, filename: &String) {
    let reg = Regex::new(pattern).unwrap();
    let file = File::open(&filename).unwrap();

    for line in BufReader::new(file).lines() {
        let l = line.unwrap();

        if reg.is_match(&l) {
            println!("{}", l);
        }
    }
}

fn help(msg: &str) {
    println!("{}", msg);
    println!("rsgrep PATTERN FILENAME");
}
