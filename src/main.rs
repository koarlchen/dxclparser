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
    WWV(WWV),
    WCY(WCY),
    WX(WX),
    ToAll(ToAll),
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

#[derive(Serialize, Deserialize, Debug)]
struct WWV {
    call_de: String,
    utc: u8,
    sfi: u16,
    a: u16,
    k: u16,
    info1: String,
    info2: String,
}

impl Spot for WWV {
    fn new() -> WWV {
        WWV {
            call_de: String::new(),
            utc: 0,
            sfi: 0,
            a: 0,
            k: 0,
            info1: String::new(),
            info2: String::new(),
        }
    }
}

enum RegexWwvCaptureIds {
    CallDe = 3,
    Utc = 4,
    Sfi = 5,
    A = 6,
    K = 7,
    Info1 = 8,
    Info2 = 9,
}

#[derive(Serialize, Deserialize, Debug)]
struct WCY {
    call_de: String,
    utc: u8,
    k: u16,
    expk: u16,
    a: u16,
    r: u16,
    sfi: u16,
    sa: String,
    gmf: String,
    au: String,
}

impl Spot for WCY {
    fn new() -> WCY {
        WCY {
            call_de: String::new(),
            utc: 0,
            k: 0,
            expk: 0,
            a: 0,
            r: 0,
            sfi: 0,
            sa: String::new(),
            gmf: String::new(),
            au: String::new(),
        }
    }
}

enum RegexWcyCaptureIds {
    CallDe = 3,
    Utc = 4,
    K = 5,
    Expk = 6,
    A = 7,
    R = 8,
    Sfi = 9,
    Sa = 10,
    Gmf = 11,
    Au = 12,
}

#[derive(Serialize, Deserialize, Debug)]
struct WX {
    call_de: String,
    msg: Option<String>,
}

impl Spot for WX {
    fn new() -> WX {
        WX {
            call_de: String::new(),
            msg: None,
        }
    }
}

enum RegexWxCaptureIds {
    CallDe = 3,
    Msg = 4,
}

#[derive(Serialize, Deserialize)]
pub struct ToAll {
    call_de: String,
    msg: Option<String>,
}

impl Spot for ToAll {
    fn new() -> ToAll {
        ToAll {
            call_de: String::new(),
            msg: None,
        }
    }
}

enum RegexToAllCaptureIds {
    CallDe = 3,
    Msg = 4,
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
        Ok(SpotType::WWV(WWV::new()))
    } else if input.starts_with("WCY de") {
        Ok(SpotType::WCY(WCY::new()))
    } else if input.starts_with("WX de") {
        Ok(SpotType::WX(WX::new()))
    } else if input.starts_with("To ALL de") {
        Ok(SpotType::ToAll(ToAll::new()))
    } else if input.starts_with("To LOCAL de") || input.starts_with("To Local de") {
        Ok(SpotType::ToLocal)
    } else {
        Err(ParseError::UnknownType)
    }
}

fn parse(raw: &str) -> Result<SpotType, ParseError> {
    match ident_type(raw)? {
        SpotType::DX(dx) => parse_dx(raw, dx),
        SpotType::WWV(wwv) => parse_wwv(raw, wwv),
        SpotType::WCY(wcy) => parse_wcy(raw, wcy),
        SpotType::WX(wx) => parse_wx(raw, wx),
        SpotType::ToAll(ta) => parse_toall(raw, ta),
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

fn parse_wwv(raw: &str, mut wwv: WWV) -> Result<SpotType, ParseError> {
    lazy_static! {
        static ref RE_WWV: Regex = Regex::new(r#"(^(WWV de) +([A-Z0-9/\-#]*) +<(\d{2})Z?> *: *SFI=(\d{1,3}), A=(\d{1,3}), K=(\d{1,3}), (.*\b) *-> *(.*\b) *$)"#).unwrap();
    }

    match RE_WWV.captures(raw) {
        Some(c) => {
            wwv.call_de = check_existence_str(&c, RegexWwvCaptureIds::CallDe as u32)?;
            wwv.utc = check_existence_num(&c, RegexWwvCaptureIds::Utc as u32)?;
            wwv.sfi = check_existence_num(&c, RegexWwvCaptureIds::Sfi as u32)?;
            wwv.a = check_existence_num(&c, RegexWwvCaptureIds::A as u32)?;
            wwv.k = check_existence_num(&c, RegexWwvCaptureIds::K as u32)?;
            wwv.info1 = check_existence_str(&c, RegexWwvCaptureIds::Info1 as u32)?;
            wwv.info2 = check_existence_str(&c, RegexWwvCaptureIds::Info2 as u32)?;

            Ok(SpotType::WWV(wwv))
        }
        None => Err(ParseError::InvalidContent),
    }
}

fn parse_wcy(raw: &str, mut wcy: WCY) -> Result<SpotType, ParseError> {
    lazy_static! {
        static ref RE_WCY: Regex = Regex::new(r#"(^(WCY de) +([A-Z0-9/\-#]*) +<(\d{2})> *: +K=(\d{1,3}) expK=(\d{1,3}) A=(\d{1,3}) R=(\d{1,3}) SFI=(\d{1,3}) SA=([a-zA-Z]{1,3}) GMF=([a-zA-Z]{1,3}) Au=([a-zA-Z]{2}) *$)"#).unwrap();
    }

    match RE_WCY.captures(raw) {
        Some(c) => {
            wcy.call_de = check_existence_str(&c, RegexWcyCaptureIds::CallDe as u32)?;
            wcy.utc = check_existence_num(&c, RegexWcyCaptureIds::Utc as u32)?;
            wcy.k = check_existence_num(&c, RegexWcyCaptureIds::K as u32)?;
            wcy.expk = check_existence_num(&c, RegexWcyCaptureIds::Expk as u32)?;
            wcy.a = check_existence_num(&c, RegexWcyCaptureIds::A as u32)?;
            wcy.r = check_existence_num(&c, RegexWcyCaptureIds::R as u32)?;
            wcy.sfi = check_existence_num(&c, RegexWcyCaptureIds::Sfi as u32)?;
            wcy.sa = check_existence_str(&c, RegexWcyCaptureIds::Sa as u32)?;
            wcy.gmf = check_existence_str(&c, RegexWcyCaptureIds::Gmf as u32)?;
            wcy.au = check_existence_str(&c, RegexWcyCaptureIds::Au as u32)?;

            Ok(SpotType::WCY(wcy))
        }
        None => Err(ParseError::InvalidContent),
    }
}

fn parse_wx(raw: &str, mut wx: WX) -> Result<SpotType, ParseError> {
    lazy_static! {
        static ref RE_WX: Regex = Regex::new(r#"(^(WX de) +([A-Z0-9/\-#]*)[ :]+(.*)?$)"#).unwrap();
    }

    match RE_WX.captures(raw) {
        Some(c) => {
            wx.call_de = check_existence_str(&c, RegexWxCaptureIds::CallDe as u32)?;
            wx.msg = check_existence_str_opt(&c, RegexWxCaptureIds::Msg as u32);

            Ok(SpotType::WX(wx))
        }
        None => Err(ParseError::InvalidContent),
    }
}

fn parse_toall(raw: &str, mut ta: ToAll) -> Result<SpotType, ParseError> {
    lazy_static! {
        static ref RE_TOALL: Regex =
            Regex::new(r#"(^(To ALL de) +([A-Z0-9/\-#]*)[ :]+(.*)?$)"#).unwrap();
    }

    match RE_TOALL.captures(raw) {
        Some(c) => {
            ta.call_de = check_existence_str(&c, RegexToAllCaptureIds::CallDe as u32)?;
            ta.msg = check_existence_str_opt(&c, RegexToAllCaptureIds::Msg as u32);

            Ok(SpotType::ToAll(ta))
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
    //let input = "DX de DF2MX:     18160.0  DL8AW/P      EU-156 Tombelaine Isl.         2259Z RF80";
    //let input = "WWV de VE7CC <00>:   SFI=69, A=5, K=1, No Storms -> No Storms";
    //let input = "WCY de DK0WCY-1 <23> : K=2 expK=2 A=7 R=26 SFI=79 SA=qui GMF=qui Au=no";
    //let input = "WX de OZ4AEC: FULL";
    let input = "To ALL de SV5FRI-1: SV5FRI-1 DXCluster: telnet dxc.sv5fri.eu 7300";

    match parse(input) {
        Ok(spot) => {
            match &spot {
                SpotType::DX(dx) => {
                    println!("Found DX spot from {}", dx.call_de)
                }
                SpotType::WWV(wwv) => {
                    println!("Found WWV spot from {}", wwv.call_de)
                }
                SpotType::WCY(wcy) => {
                    println!("Found WCY spot from {}", wcy.call_de)
                }
                SpotType::WX(wx) => {
                    println!("Found WX spot from {}", wx.call_de)
                }
                SpotType::ToAll(ta) => {
                    println!("Found ToAll spot from {}", ta.call_de)
                }
                _ => {
                    println!("Unknown SpotType found");
                }
            }

            println!("{}", serde_json::to_string(&spot).unwrap());
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}
