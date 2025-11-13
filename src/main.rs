use clap::{App, Arg};
use regex::Regex;
use std::{
    fs::File,
    io::{self, BufReader, prelude::*, stdin},
};

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    println!("Searching for '{}'...", re);
    let mut found = false;
    for line_ in reader.lines() {
        let line = line_.unwrap();
        match re.find(&line) {
            Some(_) => {
                found = true;
                println!("{}", line);
            },
            None => (),
        }
    }
    if !found {
        println!("No match for {}", re);
    }
}

fn main() {
    let args = App::new("Grep-Lite")
        .arg(Arg::with_name("pattern").help("Pattern to search for").takes_value(true).required(true))
        .arg(Arg::with_name("input").help("Path/filename of text").takes_value(true).required(true))
        .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let input = args.value_of("input").unwrap();
    let re = Regex::new(pattern).unwrap();
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);
    process_lines(reader, re);
}
