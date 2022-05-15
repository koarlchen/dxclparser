use regex::{Captures, Regex};
use serde::{Deserialize, Serialize};
use std::fmt;
#[macro_use]
extern crate lazy_static;

/// Structured representation of a parsed spot
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Spot {
    /// Spot of the type DX
    DX(DX),

    /// Spot of the type WWV
    WWV(WWV),

    /// Spot of the type WCY
    WCY(WCY),

    /// Spot of the type WX
    WX(WX),

    /// Spot of the type ToAll
    ToAll(ToAll),

    /// Spot of the type ToLocal
    ToLocal(ToLocal),
}

impl Spot {
    /// Convert structured spot into its corresponding json format
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DX {
    pub call_de: String,
    pub call_dx: String,
    pub freq: u64,
    pub utc: u16,
    pub loc: Option<String>,
    pub comment: Option<String>,
}

impl DX {
    fn new() -> DX {
        DX {
            call_de: String::new(),
            call_dx: String::new(),
            freq: 0,
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct WWV {
    pub call_de: String,
    pub utc: u8,
    pub sfi: u16,
    pub a: u16,
    pub k: u16,
    pub info1: String,
    pub info2: String,
}

impl WWV {
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct WCY {
    pub call_de: String,
    pub utc: u8,
    pub k: u16,
    pub expk: u16,
    pub a: u16,
    pub r: u16,
    pub sfi: u16,
    pub sa: String,
    pub gmf: String,
    pub au: String,
}

impl WCY {
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct WX {
    pub call_de: String,
    pub msg: Option<String>,
}

impl WX {
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ToAll {
    pub call_de: String,
    pub msg: Option<String>,
}

impl ToAll {
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ToLocal {
    pub call_de: String,
    pub utc: Option<u16>,
    pub msg: Option<String>,
}

impl ToLocal {
    fn new() -> ToLocal {
        ToLocal {
            call_de: String::new(),
            utc: None,
            msg: None,
        }
    }
}

enum RegexToLocalCaptureIds {
    CallDe = 3,
    Utc = 4,
    Msg = 5,
}

/// Possible errors while parsing spot
#[derive(Debug, PartialEq)]
pub enum ParseError {
    /// Unknown type of spot
    UnknownType,

    /// The content of the spot does not match the detected type
    InvalidContent,

    /// Required field of the spot is missing
    MissingField,

    /// Internal error occurred while parsing
    InternalError,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error while parsing: {:?}", self)
    }
}

/// Parse a spot received from a DX Cluster into a struct.
///
/// ## Arguments
///
/// * `raw`: A raw spot that is already cleaned from newline or bell characters etc.
///
/// # Result
///
/// In case the spot was parsed successfully, the structure containing the spot shall be returned.
/// In case of a error the occurred error shall be returned.
pub fn parse(raw: &str) -> Result<Spot, ParseError> {
    match ident_type(raw)? {
        Spot::DX(dx) => parse_dx(raw, dx),
        Spot::WWV(wwv) => parse_wwv(raw, wwv),
        Spot::WCY(wcy) => parse_wcy(raw, wcy),
        Spot::WX(wx) => parse_wx(raw, wx),
        Spot::ToAll(ta) => parse_toall(raw, ta),
        Spot::ToLocal(tl) => parse_tolocal(raw, tl),
    }
}

fn ident_type(input: &str) -> Result<Spot, ParseError> {
    if input.starts_with("DX de") {
        Ok(Spot::DX(DX::new()))
    } else if input.starts_with("WWV de") {
        Ok(Spot::WWV(WWV::new()))
    } else if input.starts_with("WCY de") {
        Ok(Spot::WCY(WCY::new()))
    } else if input.starts_with("WX de") {
        Ok(Spot::WX(WX::new()))
    } else if input.starts_with("To ALL de") {
        Ok(Spot::ToAll(ToAll::new()))
    } else if input.starts_with("To LOCAL de") || input.starts_with("To Local de") {
        Ok(Spot::ToLocal(ToLocal::new()))
    } else {
        Err(ParseError::UnknownType)
    }
}

fn parse_dx(raw: &str, mut dx: DX) -> Result<Spot, ParseError> {
    lazy_static! {
        static ref RE_DX: Regex = Regex::new(r#"(^(DX de) +([A-Z0-9/\-#]{3,}):? *(\d*.\d{1,2}) *([A-Z0-9/\-#]{3,}) +(.*\S)? +(\d{4}){1}Z *(\w{2}\d{2})?$)"#).unwrap();
    }

    match RE_DX.captures(raw) {
        Some(c) => {
            dx.call_de = check_existence_str(&c, RegexDxCaptureIds::CallDe as u32)?;
            dx.call_dx = check_existence_str(&c, RegexDxCaptureIds::CallDx as u32)?;
            dx.freq =
                (check_existence_num::<f64>(&c, RegexDxCaptureIds::Freq as u32)? * 1000.0) as u64;
            dx.utc = check_existence_num(&c, RegexDxCaptureIds::Utc as u32)?;
            dx.loc = check_existence_str_opt(&c, RegexDxCaptureIds::Loc as u32);
            dx.comment = check_existence_str_opt(&c, RegexDxCaptureIds::Comment as u32);

            Ok(Spot::DX(dx))
        }
        None => Err(ParseError::InvalidContent),
    }
}

fn parse_wwv(raw: &str, mut wwv: WWV) -> Result<Spot, ParseError> {
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

            Ok(Spot::WWV(wwv))
        }
        None => Err(ParseError::InvalidContent),
    }
}

fn parse_wcy(raw: &str, mut wcy: WCY) -> Result<Spot, ParseError> {
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

            Ok(Spot::WCY(wcy))
        }
        None => Err(ParseError::InvalidContent),
    }
}

fn parse_wx(raw: &str, mut wx: WX) -> Result<Spot, ParseError> {
    lazy_static! {
        static ref RE_WX: Regex = Regex::new(r#"(^(WX de) +([A-Z0-9/\-#]*)[ :]+(.*)?$)"#).unwrap();
    }

    match RE_WX.captures(raw) {
        Some(c) => {
            wx.call_de = check_existence_str(&c, RegexWxCaptureIds::CallDe as u32)?;
            wx.msg = check_existence_str_opt(&c, RegexWxCaptureIds::Msg as u32);

            Ok(Spot::WX(wx))
        }
        None => Err(ParseError::InvalidContent),
    }
}

fn parse_toall(raw: &str, mut ta: ToAll) -> Result<Spot, ParseError> {
    lazy_static! {
        static ref RE_TOALL: Regex =
            Regex::new(r#"(^(To ALL de) +([A-Z0-9/\-#]*)[ :]+(.*)?$)"#).unwrap();
    }

    match RE_TOALL.captures(raw) {
        Some(c) => {
            ta.call_de = check_existence_str(&c, RegexToAllCaptureIds::CallDe as u32)?;
            ta.msg = check_existence_str_opt(&c, RegexToAllCaptureIds::Msg as u32);

            Ok(Spot::ToAll(ta))
        }
        None => Err(ParseError::InvalidContent),
    }
}

fn parse_tolocal(raw: &str, mut tl: ToLocal) -> Result<Spot, ParseError> {
    lazy_static! {
        static ref RE_TOLOCAL: Regex = Regex::new(
            r#"(^(To (?:LOCAL|Local) de) +([A-Z0-9/\-#]*)(?: +<(\d{4})Z>)?[ :]+(.*)?$)"#
        )
        .unwrap();
    }

    match RE_TOLOCAL.captures(raw) {
        Some(c) => {
            tl.call_de = check_existence_str(&c, RegexToLocalCaptureIds::CallDe as u32)?;
            tl.msg = check_existence_str_opt(&c, RegexToLocalCaptureIds::Msg as u32);
            tl.utc = check_existence_num_opt(&c, RegexToLocalCaptureIds::Utc as u32)?;

            Ok(Spot::ToLocal(tl))
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

fn check_existence_num_opt<T>(cap: &Captures, id: u32) -> Result<Option<T>, ParseError>
where
    T: std::str::FromStr,
{
    if let Some(val) = cap.get(id.try_into().unwrap()) {
        match val.as_str().parse::<T>() {
            Ok(v) => Ok(Some(v)),
            Err(_) => Err(ParseError::InternalError),
        }
    } else {
        Ok(None)
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

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn dx_valid() {
        let spot =
            "DX de DF2MX:     18160.0  DL8AW/P      EU-156 Tombelaine Isl.         2259Z RF80";
        let res = parse(spot);
        let exp = Spot::DX(DX {
            call_de: "DF2MX".into(),
            call_dx: "DL8AW/P".into(),
            freq: 18160000,
            utc: 2259,
            loc: Some("RF80".into()),
            comment: Some("EU-156 Tombelaine Isl.".into()),
        });
        assert_eq!(res, Ok(exp));
    }

    #[test]
    fn dx_only_type() {
        let spot = "DX de DF2MX";
        let res = parse(spot);
        assert_eq!(res, Err(ParseError::InvalidContent));
    }

    #[test]
    fn dx_missing_loc() {
        let spot = "DX de DF2MX:     18160.0  DL8AW/P      EU-156 Tombelaine Isl.         2259Z";
        let res = parse(spot);
        let exp = Spot::DX(DX {
            call_de: "DF2MX".into(),
            call_dx: "DL8AW/P".into(),
            freq: 18160000,
            utc: 2259,
            loc: None,
            comment: Some("EU-156 Tombelaine Isl.".into()),
        });
        assert_eq!(res, Ok(exp));
    }

    #[test]
    fn dx_missing_comment() {
        let spot =
            "DX de DF2MX:     18160.0  DL8AW/P                                     2259Z RF80";
        let res = parse(spot);
        let exp = Spot::DX(DX {
            call_de: "DF2MX".into(),
            call_dx: "DL8AW/P".into(),
            freq: 18160000,
            utc: 2259,
            loc: Some("RF80".into()),
            comment: None,
        });
        assert_eq!(res, Ok(exp));
    }

    #[test]
    fn dx_missing_call_de() {
        let spot = "DX de      :     18160.0  DL8AW/P      EU-156 Tombelaine Isl.         2259Z";
        let res = parse(spot);
        assert_eq!(res, Err(ParseError::InvalidContent));
    }

    // FIXME
    // #[test]
    // fn dx_missing_call_dx() {
    //     let spot = "DX de DF2MX:     18160.0               EU-156 Tombelaine Isl.         2259Z";
    //     let res = parse(spot);
    //     assert_eq!(res, Err(ParseError::InvalidContent));
    // }

    #[test]
    fn dx_missing_freq() {
        let spot = "DX de DF2MX:              DL8AW/P      EU-156 Tombelaine Isl.         2259Z";
        let res = parse(spot);
        assert_eq!(res, Err(ParseError::InvalidContent));
    }

    #[test]
    fn dx_missing_utc() {
        let spot = "DX de DF2MX:     18160.0  DL8AW/P      EU-156 Tombelaine Isl.              ";
        let res = parse(spot);
        assert_eq!(res, Err(ParseError::InvalidContent));
    }

    #[test]
    fn wwv_valid() {
        let spot = "WWV de VE7CC <00>:   SFI=69, A=5, K=1, No Storms -> No Storms";
        let res = parse(spot);
        let exp = Spot::WWV(WWV {
            call_de: "VE7CC".into(),
            utc: 0,
            sfi: 69,
            a: 5,
            k: 1,
            info1: "No Storms".into(),
            info2: "No Storms".into(),
        });
        assert_eq!(res, Ok(exp));
    }

    #[test]
    fn wwv_only_type() {
        let spot = "WWV de VE7CC";
        let res = parse(spot);
        assert_eq!(res, Err(ParseError::InvalidContent));
    }

    // FIXME
    // #[test]
    // fn wwv_missing_call_de() {
    //     let spot = "WWV de       <00>:   SFI=69, A=5, K=1, No Storms -> No Storms";
    //     let res = parse(spot);
    //     assert_eq!(res, Err(ParseError::InvalidContent));
    // }

    #[test]
    fn wwv_missing_utc1() {
        let spot = "WWV de VE7CC <>:   SFI=69, A=5, K=1, No Storms -> No Storms";
        let res = parse(spot);
        assert_eq!(res, Err(ParseError::InvalidContent));
    }

    #[test]
    fn wwv_missing_utc2() {
        let spot = "WWV de VE7CC :   SFI=69, A=5, K=1, No Storms -> No Storms";
        let res = parse(spot);
        assert_eq!(res, Err(ParseError::InvalidContent));
    }

    #[test]
    fn wwv_missing_sfi1() {
        let spot = "WWV de VE7CC <00>:   SFI=, A=5, K=1, No Storms -> No Storms";
        let res = parse(spot);
        assert_eq!(res, Err(ParseError::InvalidContent));
    }

    #[test]
    fn wwv_missing_sfi2() {
        let spot = "WWV de VE7CC <00>:   A=5, K=1, No Storms -> No Storms";
        let res = parse(spot);
        assert_eq!(res, Err(ParseError::InvalidContent));
    }

    #[test]
    fn wwv_missing_a1() {
        let spot = "WWV de VE7CC <00>:   SFI=69, A=, K=1, No Storms -> No Storms";
        let res = parse(spot);
        assert_eq!(res, Err(ParseError::InvalidContent));
    }

    #[test]
    fn wwv_missing_a2() {
        let spot = "WWV de VE7CC <00>:   SFI=69, K=1, No Storms -> No Storms";
        let res = parse(spot);
        assert_eq!(res, Err(ParseError::InvalidContent));
    }

    #[test]
    fn wwv_missing_k1() {
        let spot = "WWV de VE7CC <00>:   SFI=69, A=5, K=, No Storms -> No Storms";
        let res = parse(spot);
        assert_eq!(res, Err(ParseError::InvalidContent));
    }

    #[test]
    fn wwv_missing_k2() {
        let spot = "WWV de VE7CC <00>:   SFI=69, A=5, No Storms -> No Storms";
        let res = parse(spot);
        assert_eq!(res, Err(ParseError::InvalidContent));
    }

    #[test]
    fn wwv_missing_info1() {
        let spot = "WWV de VE7CC <00>:   SFI=69, A=5, K=1, -> No Storms";
        let res = parse(spot);
        assert_eq!(res, Err(ParseError::InvalidContent));
    }

    #[test]
    fn wwv_missing_info2() {
        let spot = "WWV de VE7CC <00>:   SFI=69, A=5, K=1, No Storms ->";
        let res = parse(spot);
        assert_eq!(res, Err(ParseError::InvalidContent));
    }

    #[test]
    fn wwv_missing_info12() {
        let spot = "WWV de VE7CC <00>:   SFI=69, A=5, K=1, ->";
        let res = parse(spot);
        assert_eq!(res, Err(ParseError::InvalidContent));
    }

    #[test]
    fn wwv_missing_info_both() {
        let spot = "WWV de VE7CC <00>:   SFI=69, A=5, K=1,";
        let res = parse(spot);
        assert_eq!(res, Err(ParseError::InvalidContent));
    }

    #[test]
    fn wcy_valid() {
        let spot = "WCY de DK0WCY-1 <23> : K=2 expK=3 A=7 R=26 SFI=79 SA=qui GMF=qui Au=no";
        let res = parse(spot);
        let exp = Spot::WCY(WCY {
            call_de: "DK0WCY-1".into(),
            utc: 23,
            k: 2,
            expk: 3,
            a: 7,
            r: 26,
            sfi: 79,
            sa: "qui".into(),
            gmf: "qui".into(), // FIXME: remove double "qui"
            au: "no".into(),
        });
        assert_eq!(res, Ok(exp));
    }

    #[test]
    fn wcy_only_type() {
        let spot = "WCY de DK0WCY-1";
        let res = parse(spot);
        assert_eq!(res, Err(ParseError::InvalidContent))
    }

    // FIXME
    // #[test]
    // fn wcy_missing_call_de() {
    //     let spot = "WCY de          <23> : K=2 expK=3 A=7 R=26 SFI=79 SA=qui GMF=qui Au=no";
    //     let res = parse(spot);
    //     assert_eq!(res, Err(ParseError::InvalidContent));
    // }

    #[test]
    fn wcy_missing_utc1() {
        let spot = "WCY de DK0WCY-1 <> : K=2 expK=3 A=7 R=26 SFI=79 SA=qui GMF=qui Au=no";
        let res = parse(spot);
        assert_eq!(res, Err(ParseError::InvalidContent));
    }

    #[test]
    fn wcy_missing_utc2() {
        let spot = "WCY de DK0WCY-1 : K=2 expK=3 A=7 R=26 SFI=79 SA=qui GMF=qui Au=no";
        let res = parse(spot);
        assert_eq!(res, Err(ParseError::InvalidContent));
    }

    #[test]
    fn wcy_missing_k1() {
        let spot = "WCY de DK0WCY-1 <23> : K= expK=3 A=7 R=26 SFI=79 SA=qui GMF=qui Au=no";
        let res = parse(spot);
        assert_eq!(res, Err(ParseError::InvalidContent));
    }

    #[test]
    fn wcy_missing_k2() {
        let spot = "WCY de DK0WCY-1 <23> : expK=3 A=7 R=26 SFI=79 SA=qui GMF=qui Au=no";
        let res = parse(spot);
        assert_eq!(res, Err(ParseError::InvalidContent));
    }

    #[test]
    fn wcy_missing_expk1() {
        let spot = "WCY de DK0WCY-1 <23> : K=2 expK= A=7 R=26 SFI=79 SA=qui GMF=qui Au=no";
        let res = parse(spot);
        assert_eq!(res, Err(ParseError::InvalidContent));
    }

    #[test]
    fn wcy_missing_expk2() {
        let spot = "WCY de DK0WCY-1 <23> : K=2 A=7 R=26 SFI=79 SA=qui GMF=qui Au=no";
        let res = parse(spot);
        assert_eq!(res, Err(ParseError::InvalidContent));
    }

    #[test]
    fn wcy_missing_a1() {
        let spot = "WCY de DK0WCY-1 <23> : K=2 expK=3 A= R=26 SFI=79 SA=qui GMF=qui Au=no";
        let res = parse(spot);
        assert_eq!(res, Err(ParseError::InvalidContent));
    }

    #[test]
    fn wcy_missing_a2() {
        let spot = "WCY de DK0WCY-1 <23> : K=2 expK=3 R=26 SFI=79 SA=qui GMF=qui Au=no";
        let res = parse(spot);
        assert_eq!(res, Err(ParseError::InvalidContent));
    }

    #[test]
    fn wcy_missing_r1() {
        let spot = "WCY de DK0WCY-1 <23> : K=2 expK=3 A=7 R= SFI=79 SA=qui GMF=qui Au=no";
        let res = parse(spot);
        assert_eq!(res, Err(ParseError::InvalidContent));
    }

    #[test]
    fn wcy_missing_r2() {
        let spot = "WCY de DK0WCY-1 <23> : K=2 expK=3 A=7 SFI=79 SA=qui GMF=qui Au=no";
        let res = parse(spot);
        assert_eq!(res, Err(ParseError::InvalidContent));
    }

    #[test]
    fn wcy_missing_sfi1() {
        let spot = "WCY de DK0WCY-1 <23> : K=2 expK=3 A=7 R=26 SFI= SA=qui GMF=qui Au=no";
        let res = parse(spot);
        assert_eq!(res, Err(ParseError::InvalidContent));
    }

    #[test]
    fn wcy_missing_sfi2() {
        let spot = "WCY de DK0WCY-1 <23> : K=2 expK=3 A=7 R=26 SA=qui GMF=qui Au=no";
        let res = parse(spot);
        assert_eq!(res, Err(ParseError::InvalidContent));
    }

    #[test]
    fn wcy_missing_sa1() {
        let spot = "WCY de DK0WCY-1 <23> : K=2 expK=3 A=7 R=26 SFI=79 SA= GMF=qui Au=no";
        let res = parse(spot);
        assert_eq!(res, Err(ParseError::InvalidContent));
    }

    #[test]
    fn wcy_missing_sa2() {
        let spot = "WCY de DK0WCY-1 <23> : K=2 expK=3 A=7 R=26 SFI=79 GMF=qui Au=no";
        let res = parse(spot);
        assert_eq!(res, Err(ParseError::InvalidContent));
    }

    #[test]
    fn wcy_missing_gmf1() {
        let spot = "WCY de DK0WCY-1 <23> : K=2 expK=3 A=7 R=26 SFI=79 SA=qui Au=no";
        let res = parse(spot);
        assert_eq!(res, Err(ParseError::InvalidContent));
    }

    #[test]
    fn wcy_missing_gmf2() {
        let spot = "WCY de DK0WCY-1 <23> : K=2 expK=3 A=7 R=26 SFI=79 SA=qui GMF= Au=no";
        let res = parse(spot);
        assert_eq!(res, Err(ParseError::InvalidContent));
    }

    #[test]
    fn wcy_missing_au1() {
        let spot = "WCY de DK0WCY-1 <23> : K=2 expK=3 A=7 R=26 SFI=79 SA=qui GMF=qui Au=";
        let res = parse(spot);
        assert_eq!(res, Err(ParseError::InvalidContent));
    }

    #[test]
    fn wcy_missing_au2() {
        let spot = "WCY de DK0WCY-1 <23> : K=2 expK=3 A=7 R=26 SFI=79 SA=qui GMF=qui";
        let res = parse(spot);
        assert_eq!(res, Err(ParseError::InvalidContent));
    }

    #[test]
    fn wx_valid() {
        let spot = "WX de OZ4AEC: FULL";
        let res = parse(spot);
        let exp = Spot::WX(WX {
            call_de: "OZ4AEC".into(),
            msg: Some("FULL".into()),
        });
        assert_eq!(res, Ok(exp));
    }

    #[test]
    fn wx_only_type() {
        let spot = "WX de OZ4AEC";
        let res = parse(spot);
        assert_eq!(res, Err(ParseError::InvalidContent));
    }

    // FIXME
    // #[test]
    // fn wx_missing_call_de() {
    //     let spot = "WX de       : FULL";
    //     let res = parse(spot);
    //     assert_eq!(res, Err(ParseError::InvalidContent));
    // }

    // FIXME
    // #[test]
    // fn wx_missing_msg() {
    //     let spot = "WX de OZ4AEC:";
    //     let res = parse(spot);
    //     let exp = SpotType::WX(WX {
    //         call_de: "OZ4AEC".into(),
    //         msg: None,
    //     });
    //     assert_eq!(res, Ok(exp));
    // }

    #[test]
    fn toall_valid() {
        let spot = "To ALL de SV5FRI-1: SV5FRI-1 DXCluster: telnet dxc.sv5fri.eu 7300";
        let res = parse(spot);
        let exp = Spot::ToAll(ToAll {
            call_de: "SV5FRI-1".into(),
            msg: Some("SV5FRI-1 DXCluster: telnet dxc.sv5fri.eu 7300".into()),
        });
        assert_eq!(res, Ok(exp));
    }

    #[test]
    fn toall_only_type() {
        let spot = "To ALL de SV5FRI-1";
        let res = parse(spot);
        assert_eq!(res, Err(ParseError::InvalidContent));
    }

    // FIXME
    // #[test]
    // fn toall_missing_call_de() {
    //     let spot = "To ALL de         : SV5FRI-1 DXCluster: telnet dxc.sv5fri.eu 7300";
    //     let res = parse(spot);
    //     assert_eq!(res, Err(ParseError::InvalidContent));
    // }

    // FIXME
    // #[test]
    // fn toall_missing_msg() {
    //     let spot = "To ALL de SV5FRI-1:";
    //     let res = parse(spot);
    //     let exp = SpotType::ToAll(ToAll {
    //         call_de: "SV5FRI-1".into(),
    //         msg: None,
    //     });
    //     assert_eq!(res, Ok(exp));
    // }

    #[test]
    fn tolocal_valid_lower_case() {
        let spot = "To Local de N5UXT <1405Z> : rebooting";
        let res = parse(spot);
        let exp = Spot::ToLocal(ToLocal {
            call_de: "N5UXT".into(),
            utc: Some(1405),
            msg: Some("rebooting".into()),
        });
        assert_eq!(res, Ok(exp));
    }

    #[test]
    fn tolocal_valid_upper_case() {
        let spot = "To LOCAL de N5UXT <1405Z> : rebooting";
        let res = parse(spot);
        let exp = Spot::ToLocal(ToLocal {
            call_de: "N5UXT".into(),
            utc: Some(1405),
            msg: Some("rebooting".into()),
        });
        assert_eq!(res, Ok(exp));
    }

    #[test]
    fn tolocal_only_type() {
        let spot = "To Local de N5UXT";
        let res = parse(spot);
        assert_eq!(res, Err(ParseError::InvalidContent));
    }

    // FIXME
    // #[test]
    // fn tolocal_missing_call_de() {
    //     let spot = "To Local de       <1405Z> : rebooting";
    //     let res = parse(spot);
    //     assert_eq!(res, Err(ParseError::InvalidContent));
    // }

    // FIXME
    // #[test]
    // fn tolocal_missing_utc1() {
    //     let spot = "To Local de N5UXT <Z> : rebooting";
    //     let res = parse(spot);
    //     assert_eq!(res, Err(ParseError::InvalidContent));
    // }

    // FIXME
    // #[test]
    // fn tolocal_missing_utc2() {
    //     let spot = "To Local de N5UXT <> : rebooting";
    //     let res = parse(spot);
    //     assert_eq!(res, Err(ParseError::InvalidContent));
    // }

    // FIXME
    // #[test]
    // fn tolocal_missing_utc3() {
    //     let spot = "To Local de N5UXT <1405> : rebooting";
    //     let res = parse(spot);
    //     assert_eq!(res, Err(ParseError::InvalidContent));
    // }

    // FIXME
    // #[test]
    // fn tolocal_missing_utc4() {
    //     let spot = "To Local de N5UXT : rebooting";
    //     let res = parse(spot);
    //     assert_eq!(res, Err(ParseError::InvalidContent));
    // }

    // FIXME
    // #[test]
    // fn tolocal_missing_msg() {
    //     let spot = "To Local de N5UXT <1405Z> :";
    //     let res = parse(spot);
    //     assert_eq!(res, Err(ParseError::InvalidContent));
    // }
}
