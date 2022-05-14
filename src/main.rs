use regex::{Captures, Regex};
use serde::{Deserialize, Serialize};
use std::fmt;
#[macro_use]
extern crate lazy_static;

trait Spot {
    fn new() -> Self;
}

#[derive(Serialize, Deserialize)]
enum SpotType {
    DX(DX),
    WWV,
    WCY,
    WX,
    ToAll,
    ToLocal,
}

#[derive(Serialize, Deserialize, Debug)]
struct DX {
    call_de: String,
    call_dx: String,
    freq: f32,
    utc: u16,
    loc: Option<String>,
    comment: Option<String>,
}

impl Spot for DX {
    fn new() -> DX {
        DX {
            call_de: String::new(),
            call_dx: String::new(),
            freq: 0.0,
            utc: 0,
            loc: None,
            comment: None,
        }
    }
}

enum RegexDxCaptureIds {
    CallDe = 3,
    Freq = 4,
    CallDx = 5,
    Comment = 6,
    Utc = 7,
    Loc = 8,
}

#[derive(Debug)]
enum ParseError {
    UnknownType,
    InvalidContent,
    MissingField,
    InternalError,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error while parsing: {:?}", self)
    }
}

fn ident_type(input: &str) -> Result<SpotType, ParseError> {
    if input.starts_with("DX de") {
        Ok(SpotType::DX(DX::new()))
    } else if input.starts_with("WWV de") {
        Ok(SpotType::WWV)
    } else if input.starts_with("WCY de") {
        Ok(SpotType::WCY)
    } else if input.starts_with("WX de") {
        Ok(SpotType::WX)
    } else if input.starts_with("To ALL de") {
        Ok(SpotType::ToAll)
    } else if input.starts_with("To LOCAL de") || input.starts_with("To Local de") {
        Ok(SpotType::ToLocal)
    } else {
        Err(ParseError::UnknownType)
    }
}

fn parse(raw: &str) -> Result<SpotType, ParseError> {
    match ident_type(raw)? {
        SpotType::DX(dx) => parse_dx(raw, dx),
        SpotType::WWV => {
            todo!()
        }
        SpotType::WCY => {
            todo!()
        }
        SpotType::WX => {
            todo!()
        }
        SpotType::ToAll => {
            todo!()
        }
        SpotType::ToLocal => {
            todo!()
        }
    }
}

fn parse_dx(raw: &str, mut dx: DX) -> Result<SpotType, ParseError> {
    lazy_static! {
        static ref RE_DX: Regex = Regex::new(r#"(^(DX de) +([A-Z0-9/\-#]{3,}):? *(\d*.\d{1,2}) *([A-Z0-9/\-#]{3,}) +(.*\S)? +(\d{4}){1}Z *(\w{2}\d{2})?$)"#).unwrap();
    }

    match RE_DX.captures(raw) {
        Some(c) => {
            dx.call_de = check_existence_str(&c, RegexDxCaptureIds::CallDe as u32)?;
            dx.call_dx = check_existence_str(&c, RegexDxCaptureIds::CallDx as u32)?;
            dx.freq = check_existence_num(&c, RegexDxCaptureIds::Freq as u32)?;
            dx.utc = check_existence_num(&c, RegexDxCaptureIds::Utc as u32)?;
            dx.loc = check_existence_str_opt(&c, RegexDxCaptureIds::Loc as u32);
            dx.comment = check_existence_str_opt(&c, RegexDxCaptureIds::Comment as u32);

            Ok(SpotType::DX(dx))
        }
        None => Err(ParseError::InvalidContent),
    }
}

fn check_existence_num<T>(cap: &Captures, id: u32) -> Result<T, ParseError>
where
    T: std::str::FromStr,
{
    match cap.get(id.try_into().unwrap()) {
        Some(val) => match val.as_str().parse::<T>() {
            Ok(v) => Ok(v),
            Err(_) => Err(ParseError::InternalError),
        },
        None => Err(ParseError::MissingField),
    }
}

fn check_existence_str(cap: &Captures, id: u32) -> Result<String, ParseError> {
    match cap.get(id.try_into().unwrap()) {
        Some(val) => Ok(String::from(val.as_str())),
        None => Err(ParseError::MissingField),
    }
}

fn check_existence_str_opt(cap: &Captures, id: u32) -> Option<String> {
    cap.get(id.try_into().unwrap())
        .map(|val| String::from(val.as_str()))
}

fn main() {
    let input = "DX de DF2MX:     18160.0  DL8AW/P      EU-156 Tombelaine Isl.         2259Z RF80";

    match parse(input) {
        Ok(spot) => {
            match &spot {
                SpotType::DX(dx) => {
                    println!("Found DX spot from {}", dx.call_de)
                }
                _ => {
                    println!("Unknown SpotType found");
                }
            }

            println!("{}", serde_json::to_string(&spot).unwrap());
        }
        Err(e) => {
            eprint!("{}", e);
        }
    }
}
