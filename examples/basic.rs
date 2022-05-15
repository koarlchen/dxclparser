extern crate dxclparser;

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let retval;

    if args.len() != 2 {
        eprintln!("Invalid number of arguments.");
        eprintln!("Usage: {} <spot>", args[0]);
        retval = 1;
    } else {
        match dxclparser::parse(args[1].trim().trim_end_matches('\u{0007}')) {
            Ok(spot) => {
                println!("{}", spot.to_json());
                retval = 0;
            }
            Err(e) => {
                println!("Failed to parse spot ({})", e);
                retval = 1;
            }
        }
    }

    process::exit(retval);
}
