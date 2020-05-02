use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
mod lcat;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
    } else {
        let file = BufReader::new(File::open(&args[1]).unwrap());
        for line in file.lines() {
            println!("{}", lcat::parse_and_format(line.unwrap()));
        }
    }
}
