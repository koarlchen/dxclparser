extern crate dxclparser;

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Reads a file containing multiple spots line by line.
/// Each successfully parsed spot will be printed to stdout in their json format.
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Invalid number of arguments.");
        eprintln!("Usage: {} <file>", args[0]);
    } else {
        let filepath = Path::new(&args[1]);
        let file = File::open(filepath).expect("Failed to open file");
        let reader = io::BufReader::new(file).lines();

        for line in reader {
            match dxclparser::parse(line.unwrap().trim().trim_end_matches('\u{0007}')) {
                Ok(spot) => {
                    println!("{}", spot.to_json());
                }
                Err(e) => {
                    eprintln!("Failed to parse spot ({})", e);
                }
            }
        }
    }
}
