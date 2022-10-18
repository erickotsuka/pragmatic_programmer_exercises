extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::io::{self, BufRead};

#[derive(Parser)]
#[grammar = "time.pest"]
pub struct TimeParser;

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match TimeParser::parse(Rule::time, line.unwrap().as_str()) {
            Ok(_) => println!("Ok"),
            Err(_) => println!("Error"),
        }
    }
}
