extern crate dxclparser;

use std::env;
use std::process;

/// Parse a spot provided as a commandline argument.
/// See also `basic_run.sh` for exemplary use.
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
                eprintln!("Failed to parse spot ({})", e);
                retval = 1;
            }
        }
    }

    process::exit(retval);
}
