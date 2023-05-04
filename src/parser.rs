// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::types::*;
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use thiserror::Error;

const REGEX_PATTERN_DX: &str = r#"^DX de +([A-Z0-9/\-#]{3,}):? *(\d*.\d{1,2}) *([A-Z0-9/\-#]{3,}) +(.*\S)? +(\d{4}){1}Z *(\w{2}\d{2})?$"#;

enum RegexDxCaptureIds {
    CallDe = 1,
    Freq = 2,
    CallDx = 3,
    Comment = 4,
    Utc = 5,
    Loc = 6,
}

const REGEX_PATTERN_RBN1: &str =
    r#"^([a-zA-z0-9]{2,}) +([0-9\-]{1,4}) +dB +([0-9]{1,3}) +((?:WPM|BPS)) +([a-zA-Z ]+)$"#;

enum RegexRbn1CaptureIds {
    Mode = 1,
    Db = 2,
    Speed = 3,
    SpeedUnit = 4,
    Info = 5,
}

const REGEX_PATTERN_RBN2: &str =
    r#"^([a-zA-z0-9]{2,}) +([0-9\-]{1,4}) +dB +([A-Z]{2}[0-9]{2})? +([a-zA-Z ]+)$"#;

enum RegexRbn2CaptureIds {
    Mode = 1,
    Db = 2,
    Loc = 3,
    Info = 4,
}

const REGEX_PATTERN_WWV: &str = r#"^WWV de +([A-Z0-9/\-#]*) +<(\d{2})Z?> *: *SFI=(\d{1,3}), A=(\d{1,3}), K=(\d{1,3}), (.*\b) *-> *(.*\b) *$"#;

enum RegexWwvCaptureIds {
    CallDe = 1,
    Utc = 2,
    Sfi = 3,
    A = 4,
    K = 5,
    Info1 = 6,
    Info2 = 7,
}

const REGEX_PATTERN_WCY: &str = r#"^WCY de +([A-Z0-9/\-#]*) +<(\d{2})> *: +K=(\d{1,3}) expK=(\d{1,3}) A=(\d{1,3}) R=(\d{1,3}) SFI=(\d{1,3}) SA=([a-zA-Z]{1,3}) GMF=([a-zA-Z]{1,3}) Au=([a-zA-Z]{2}) *$"#;

enum RegexWcyCaptureIds {
    CallDe = 1,
    Utc = 2,
    K = 3,
    Expk = 4,
    A = 5,
    R = 6,
    Sfi = 7,
    Sa = 8,
    Gmf = 9,
    Au = 10,
}

const REGEX_PATTERN_WX: &str = r#"^WX de +([A-Z0-9/\-#]*)\s?(?:<(\d{4})Z>)?[ :]+(.*)?$"#;

enum RegexWxCaptureIds {
    CallDe = 1,
    Utc = 2,
    Msg = 3,
}

const REGEX_PATTERN_TOALL: &str = r#"^To ALL de +([A-Z0-9/\-#]*)\s?(?:<(\d{4})Z>)?[ :]+(.*)?$"#;

enum RegexToAllCaptureIds {
    CallDe = 1,
    Utc = 2,
    Msg = 3,
}

const REGEX_PATTERN_TOLOCAL: &str =
    r#"^To (?:LOCAL|Local) de +([A-Z0-9/\-#]*)(?: +<(\d{4})Z>)?[ :]+(.*)?$"#;

enum RegexToLocalCaptureIds {
    CallDe = 1,
    Utc = 2,
    Msg = 3,
}

/// Possible errors while parsing spot
#[derive(Error, Debug, PartialEq, Eq)]
pub enum ParseError {
    #[error("Unknown type of spot")]
    UnknownType,

    #[error("The content of the spot does not match the detected type")]
    InvalidContent,

    #[error("Required field of the spot is missing")]
    MissingField,

    #[error("Internal error occurred while parsing")]
    InternalError,
}

/// Parse a spot received from a DX Cluster into a struct.
///
/// ## Arguments
///
/// * `raw`: A raw spot that is already cleaned from newline or bell characters etc.
///
/// ## Result
///
/// In case the spot was parsed successfully, the structure containing the spot shall be returned.
/// In case of an error the occurred error shall be returned.
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

/// Parse the comment section of a RBN spot into a struct.
///
/// ## Arguments
///
/// * `raw`: A raw comment section of a parsed DX spot. A RBN spot is basically a DX spot with additional data in the comment section)
///
/// ## Result
///
/// In case the comment was parsed successfully, the structure containing the information shall be returned.
/// In case of an error the occurred error shall be returned.
pub fn parse_rbn(raw: &str) -> Result<RBN, ParseError> {
    lazy_static! {
        static ref RE_RBN1: Regex = Regex::new(REGEX_PATTERN_RBN1).unwrap();
        static ref RE_RBN2: Regex = Regex::new(REGEX_PATTERN_RBN2).unwrap();
    }

    if let Some(c) = RE_RBN1.captures(raw) {
        let mut rbn = RBN::new();

        rbn.mode = check_existence_str(&c, RegexRbn1CaptureIds::Mode as u32)?;
        rbn.db = check_existence_num(&c, RegexRbn1CaptureIds::Db as u32)?;
        rbn.speed = check_existence_num_opt(&c, RegexRbn1CaptureIds::Speed as u32)?;
        rbn.speed_unit = check_existence_str_opt(&c, RegexRbn1CaptureIds::SpeedUnit as u32);
        rbn.info = check_existence_str(&c, RegexRbn1CaptureIds::Info as u32)?;

        return Ok(rbn);
    }

    if let Some(c) = RE_RBN2.captures(raw) {
        let mut rbn = RBN::new();

        rbn.mode = check_existence_str(&c, RegexRbn2CaptureIds::Mode as u32)?;
        rbn.db = check_existence_num(&c, RegexRbn2CaptureIds::Db as u32)?;
        rbn.info = check_existence_str(&c, RegexRbn2CaptureIds::Info as u32)?;
        rbn.loc = check_existence_str_opt(&c, RegexRbn2CaptureIds::Loc as u32);

        return Ok(rbn);
    }

    Err(ParseError::InvalidContent)
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
        static ref RE_DX: Regex = Regex::new(REGEX_PATTERN_DX).unwrap();
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
        static ref RE_WWV: Regex = Regex::new(REGEX_PATTERN_WWV).unwrap();
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
        static ref RE_WCY: Regex = Regex::new(REGEX_PATTERN_WCY).unwrap();
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
        static ref RE_WX: Regex = Regex::new(REGEX_PATTERN_WX).unwrap();
    }

    match RE_WX.captures(raw) {
        Some(c) => {
            wx.call_de = check_existence_str(&c, RegexWxCaptureIds::CallDe as u32)?;
            wx.utc = check_existence_num_opt(&c, RegexWxCaptureIds::Utc as u32)?;
            wx.msg = check_existence_str_opt(&c, RegexWxCaptureIds::Msg as u32);

            Ok(Spot::WX(wx))
        }
        None => Err(ParseError::InvalidContent),
    }
}

fn parse_toall(raw: &str, mut ta: ToAll) -> Result<Spot, ParseError> {
    lazy_static! {
        static ref RE_TOALL: Regex = Regex::new(REGEX_PATTERN_TOALL).unwrap();
    }

    match RE_TOALL.captures(raw) {
        Some(c) => {
            ta.call_de = check_existence_str(&c, RegexToAllCaptureIds::CallDe as u32)?;
            ta.utc = check_existence_num_opt(&c, RegexToAllCaptureIds::Utc as u32)?;
            ta.msg = check_existence_str_opt(&c, RegexToAllCaptureIds::Msg as u32);

            Ok(Spot::ToAll(ta))
        }
        None => Err(ParseError::InvalidContent),
    }
}

fn parse_tolocal(raw: &str, mut tl: ToLocal) -> Result<Spot, ParseError> {
    lazy_static! {
        static ref RE_TOLOCAL: Regex = Regex::new(REGEX_PATTERN_TOLOCAL).unwrap();
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

    use super::*;

    #[test]
    fn dx_valid_dxspider() {
        let spot =
            "DX de DJ1TO:      3780.0  OH5Z         LSB                            2200Z JO62";
        let res = parse(spot);
        let exp = Spot::DX(DX {
            call_de: "DJ1TO".into(),
            call_dx: "OH5Z".into(),
            freq: 3780000,
            utc: 2200,
            loc: Some("JO62".into()),
            comment: Some("LSB".into()),
        });
        assert_eq!(res, Ok(exp));
    }

    #[test]
    fn dx_valid_arcluster() {
        let spot = "DX de N2CQ:      14036.1  W0BH         OK QSO Party: Major            1624Z";
        let res = parse(spot);
        let exp = Spot::DX(DX {
            call_de: "N2CQ".into(),
            call_dx: "W0BH".into(),
            freq: 14036100,
            utc: 1624,
            loc: None,
            comment: Some("OK QSO Party: Major".into()),
        });
        assert_eq!(res, Ok(exp));
    }

    #[test]
    fn dx_valid_cccluster() {
        let spot = "DX de ZS6WN:     21075.4  CX2DAJ       FT8                            1625Z";
        let res = parse(spot);
        let exp = Spot::DX(DX {
            call_de: "ZS6WN".into(),
            call_dx: "CX2DAJ".into(),
            freq: 21075400,
            utc: 1625,
            loc: None,
            comment: Some("FT8".into()),
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
    fn dx_missing_loc_dxspider() {
        let spot = "DX de KE8GX:     14025.0  3B9FR        599 into N. MI                 1812Z";
        let res = parse(spot);
        let exp = Spot::DX(DX {
            call_de: "KE8GX".into(),
            call_dx: "3B9FR".into(),
            freq: 14025000,
            utc: 1812,
            loc: None,
            comment: Some("599 into N. MI".into()),
        });
        assert_eq!(res, Ok(exp));
    }

    #[test]
    fn dx_missing_comment_dxspider() {
        let spot =
            "DX de OZ1FJB:     3527.6  DL2ASG                                      1815Z JO55";
        let res = parse(spot);
        let exp = Spot::DX(DX {
            call_de: "OZ1FJB".into(),
            call_dx: "DL2ASG".into(),
            freq: 3527600,
            utc: 1815,
            loc: Some("JO55".into()),
            comment: None,
        });
        assert_eq!(res, Ok(exp));
    }

    #[test]
    fn dx_missing_comment_arcluster() {
        let spot = "DX de W9KXQ:     14076.0  HB9AOF                                      1629Z";
        let res = parse(spot);
        let exp = Spot::DX(DX {
            call_de: "W9KXQ".into(),
            call_dx: "HB9AOF".into(),
            freq: 14076000,
            utc: 1629,
            loc: None,
            comment: None,
        });
        assert_eq!(res, Ok(exp));
    }

    #[test]
    fn dx_missing_comment_cccluster() {
        let spot = "DX de RK9UE:      7115.0  RK6BP                                       1625Z";
        let res = parse(spot);
        let exp = Spot::DX(DX {
            call_de: "RK9UE".into(),
            call_dx: "RK6BP".into(),
            freq: 7115000,
            utc: 1625,
            loc: None,
            comment: None,
        });
        assert_eq!(res, Ok(exp));
    }

    #[test]
    fn wwv_valid_dxspider() {
        let spot = "WWV de VE7CC <21>:   SFI=70, A=12, K=3, No Storms -> No Storms";
        let res = parse(spot);
        let exp = Spot::WWV(WWV {
            call_de: "VE7CC".into(),
            utc: 21,
            sfi: 70,
            a: 12,
            k: 3,
            info1: "No Storms".into(),
            info2: "No Storms".into(),
        });
        assert_eq!(res, Ok(exp));
    }

    #[test]
    fn wwv_valid_arcluster() {
        let spot = "WWV de VE7CC <15Z> :   SFI=68, A=9, K=2, No Storms -> Minor w/G1";
        let res = parse(spot);
        let exp = Spot::WWV(WWV {
            call_de: "VE7CC".into(),
            utc: 15,
            sfi: 68,
            a: 9,
            k: 2,
            info1: "No Storms".into(),
            info2: "Minor w/G1".into(),
        });
        assert_eq!(res, Ok(exp));
    }

    #[test]
    fn wwv_valid_cccluster() {
        let spot = "WWV de AE5E <00>:   SFI=69, A=15, K=4, No Storms -> No Storms";
        let res = parse(spot);
        let exp = Spot::WWV(WWV {
            call_de: "AE5E".into(),
            utc: 0,
            sfi: 69,
            a: 15,
            k: 4,
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

    #[test]
    fn wcy_valid_dxspider() {
        let spot = "WCY de DK0WCY-1 <22> : K=4 expK=2 A=14 R=0 SFI=68 SA=qui GMF=act Au=no";
        let res = parse(spot);
        let exp = Spot::WCY(WCY {
            call_de: "DK0WCY-1".into(),
            utc: 22,
            k: 4,
            expk: 2,
            a: 14,
            r: 0,
            sfi: 68,
            sa: "qui".into(),
            gmf: "act".into(),
            au: "no".into(),
        });
        assert_eq!(res, Ok(exp));
    }

    #[test]
    fn wcy_valid_cccluster() {
        let spot = "WCY de DK0WCY-1 <17> : K=2 expK=3 A=15 R=0 SFI=68 SA=qui GMF=min Au=no";
        let res = parse(spot);
        let exp = Spot::WCY(WCY {
            call_de: "DK0WCY-1".into(),
            utc: 17,
            k: 2,
            expk: 3,
            a: 15,
            r: 0,
            sfi: 68,
            sa: "qui".into(),
            gmf: "min".into(),
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

    #[test]
    fn wx_valid_dxspider() {
        let spot = "WX de VA3SAE: va3sub";
        let res = parse(spot);
        let exp = Spot::WX(WX {
            call_de: "VA3SAE".into(),
            utc: None,
            msg: Some("va3sub".into()),
        });
        assert_eq!(res, Ok(exp));
    }

    #[test]
    fn wx_valid_cccluster() {
        let spot = "WX de LA3WAA <1001Z> :  The command WX will send a local weather announcement.  (WX Sunny and Warm)";
        let res = parse(spot);
        let exp = Spot::WX(WX {
            call_de: "LA3WAA".into(),
            utc: Some(1001),
            msg: Some(
                "The command WX will send a local weather announcement.  (WX Sunny and Warm)"
                    .into(),
            ),
        });
        assert_eq!(res, Ok(exp));
    }

    #[test]
    fn wx_only_type() {
        let spot = "WX de OZ4AEC";
        let res = parse(spot);
        assert_eq!(res, Err(ParseError::InvalidContent));
    }

    #[test]
    fn toall_valid_dxspider() {
        let spot = "To ALL de EA8CEN-9: carnaval de tenerife ea8urt";
        let res = parse(spot);
        let exp = Spot::ToAll(ToAll {
            call_de: "EA8CEN-9".into(),
            utc: None,
            msg: Some("carnaval de tenerife ea8urt".into()),
        });
        assert_eq!(res, Ok(exp));
    }

    #[test]
    fn toall_valid_cccluster() {
        let spot = "To ALL de CT2IDL <1044Z> : TNX qso..";
        let res = parse(spot);
        let exp = Spot::ToAll(ToAll {
            call_de: "CT2IDL".into(),
            utc: Some(1044),
            msg: Some("TNX qso..".into()),
        });
        assert_eq!(res, Ok(exp));
    }

    #[test]
    fn toall_only_type() {
        let spot = "To ALL de SV5FRI-1";
        let res = parse(spot);
        assert_eq!(res, Err(ParseError::InvalidContent));
    }

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
        let spot = "To LOCAL de IW5CLM: off";
        let res = parse(spot);
        let exp = Spot::ToLocal(ToLocal {
            call_de: "IW5CLM".into(),
            utc: None,
            msg: Some("off".into()),
        });
        assert_eq!(res, Ok(exp));
    }

    #[test]
    fn tolocal_only_type() {
        let spot = "To Local de N5UXT";
        let res = parse(spot);
        assert_eq!(res, Err(ParseError::InvalidContent));
    }

    #[test]
    fn rbn1_ft8() {
        let spot = "FT8  -15 dB   6 BPS  CQ";
        let res = parse_rbn(spot);
        let exp = RBN {
            mode: "FT8".into(),
            db: -15,
            speed: Some(6),
            speed_unit: Some("BPS".into()),
            info: "CQ".into(),
            loc: None,
        };
        assert_eq!(res, Ok(exp));
    }

    #[test]
    fn rbn1_cw() {
        let spot = "CW     9 dB  21 WPM  NCDXF B";
        let res = parse_rbn(spot);
        let exp = RBN {
            mode: "CW".into(),
            db: 9,
            speed: Some(21),
            speed_unit: Some("WPM".into()),
            info: "NCDXF B".into(),
            loc: None,
        };
        assert_eq!(res, Ok(exp));
    }

    #[test]
    fn rbn1_rtty() {
        let spot = "RTTY  10 dB  45 BPS  CQ";
        let res = parse_rbn(spot);
        let exp = RBN {
            mode: "RTTY".into(),
            db: 10,
            speed: Some(45),
            speed_unit: Some("BPS".into()),
            info: "CQ".into(),
            loc: None,
        };
        assert_eq!(res, Ok(exp));
    }

    #[test]
    fn rbn2() {
        let spot = "FT8  -12 dB  FK68    CQ";
        let res = parse_rbn(spot);
        let exp = RBN {
            mode: "FT8".into(),
            db: -12,
            speed: None,
            speed_unit: None,
            info: "CQ".into(),
            loc: Some("FK68".into()),
        };
        assert_eq!(res, Ok(exp));
    }
}
