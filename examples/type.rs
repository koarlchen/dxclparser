use std::env;
use std::process;

/// Parse a spot provided as a commandline argument.
/// Output the type of spot together with the callsign of the spotter.
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
                match spot {
                    dxclparser::Spot::DX(dx) => println!("Found a DX spot from {}", dx.call_de),
                    dxclparser::Spot::WWV(wwv) => println!("Found a WWV spot from {}", wwv.call_de),
                    dxclparser::Spot::WCY(wcy) => println!("Found a WCY spot from {}", wcy.call_de),
                    dxclparser::Spot::WX(wx) => println!("Found a WX spot from {}", wx.call_de),
                    dxclparser::Spot::ToAll(toall) => {
                        println!("Found a ToAll spot from {}", toall.call_de)
                    }
                    dxclparser::Spot::ToLocal(tolocal) => {
                        println!("Found a ToLocal spot from {}", tolocal.call_de)
                    }
                }
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
