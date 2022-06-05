use serde::{Deserialize, Serialize};

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

/// DX Spot
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DX {
    /// Call of spotting station
    pub call_de: String,

    /// Call of target station
    pub call_dx: String,

    /// Frequency (in Hz)
    pub freq: u64,

    /// Time in UTC
    pub utc: u16,

    /// Locator
    pub loc: Option<String>,

    /// Comment
    pub comment: Option<String>,
}

impl DX {
    pub fn new() -> DX {
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

/// RBN spot
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RBN {
    /// Mode: CW, RTTY or FT8
    pub mode: String,

    /// Signal strength
    pub db: i16,

    /// Speed
    pub speed: Option<u16>,

    /// Unit of speed
    pub speed_unit: Option<String>,

    /// Additional information
    pub info: String,

    /// Locator
    pub loc: Option<String>,
}

impl RBN {
    pub fn new() -> RBN {
        RBN {
            mode: String::new(),
            db: 0,
            speed: None,
            speed_unit: None,
            info: String::new(),
            loc: None,
        }
    }
}

/// WWV spot
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct WWV {
    /// Call of spotting station
    pub call_de: String,

    /// Time in UTC
    pub utc: u8,

    /// SFI index
    pub sfi: u16,

    /// A index
    pub a: u16,

    /// K index
    pub k: u16,

    /// Information 1
    pub info1: String,

    /// Information 2
    pub info2: String,
}

impl WWV {
    pub fn new() -> WWV {
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

/// WCY spot
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct WCY {
    /// Call of spotting station
    pub call_de: String,

    /// Time in UTC
    pub utc: u8,

    /// K index
    pub k: u16,

    /// expK index
    pub expk: u16,

    /// A index
    pub a: u16,

    /// R index
    pub r: u16,

    /// SFI index
    pub sfi: u16,

    /// SA index
    pub sa: String,

    /// GMF
    pub gmf: String,

    /// Aurora
    pub au: String,
}

impl WCY {
    pub fn new() -> WCY {
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

/// WX spot
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct WX {
    /// Call of spotting station
    pub call_de: String,

    /// Time in UTC
    pub utc: Option<u16>,

    /// Message sent with spot
    pub msg: Option<String>,
}

impl WX {
    pub fn new() -> WX {
        WX {
            call_de: String::new(),
            utc: None,
            msg: None,
        }
    }
}

/// To all spot
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ToAll {
    /// Call of spotting station
    pub call_de: String,

    /// Time in UTC
    pub utc: Option<u16>,

    /// Message sent with spot
    pub msg: Option<String>,
}

impl ToAll {
    pub fn new() -> ToAll {
        ToAll {
            call_de: String::new(),
            utc: None,
            msg: None,
        }
    }
}

/// To local spot
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ToLocal {
    /// Call of spotting station
    pub call_de: String,

    /// Time in UTC
    pub utc: Option<u16>,

    /// Message sent with spot
    pub msg: Option<String>,
}

impl ToLocal {
    pub fn new() -> ToLocal {
        ToLocal {
            call_de: String::new(),
            utc: None,
            msg: None,
        }
    }
}
