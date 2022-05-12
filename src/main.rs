use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct DX {
    call: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct WX {
    sfi: u16,
}

#[derive(Serialize, Deserialize)]
enum Spot {
    DX(DX),
    WX(WX),
}

#[derive(Debug)]
enum ParseError {
    UnknownType,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error while parsing: {:?}", self)
    }
}


fn parse(input: &str) -> std::result::Result<Spot, ParseError> {
    match input {
        "dx" => Ok(Spot::DX(DX {
            call: String::from("DF2MX"),
        })),
        "wx" => Ok(Spot::WX(WX { sfi: 123 })),
        _ => Err(ParseError::UnknownType),
    }
}

fn main() {
    let input = "dx";

    match parse(input) {
        Ok(spot) => {
            match &spot {
                Spot::DX(dx) => {
                    println!("Found DX spot from {}", dx.call)
                }
                Spot::WX(wx) => {
                    println!("Found WX spot with sfi={}", wx.sfi)
                }
            }

            println!("{}", serde_json::to_string(&spot).unwrap());
        }
        Err(e) => {
            eprint!("{}", e);
        }
    }
}
