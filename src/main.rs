use serde_json::{Result, Value};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct DX {
    call: String,
}

#[derive(Serialize, Deserialize)]
struct WX {
    sfi: u16,
}

// TODO
// https://stackoverflow.com/questions/53488328/how-to-specify-that-all-implementers-of-a-trait-must-also-implement-serialize
trait Spot /*: serde::Serialize*/ {
    fn get_type(&self) -> &str;
    fn to_string(&self) -> String;
    fn to_json(&self) -> String /*where Self: Serialize*/ {
        serde_json::to_string(&self).unwrap()
    }
    
}

impl Spot for DX {
    fn get_type(&self) -> &str {
        "DX"
    }
    
    fn to_string(&self) -> String {
        format!("type={}, call={}", "DX", self.call)
    }
}

impl Spot for WX {
    fn get_type(&self) -> &str {
        "WX"
    }
    
    fn to_string(&self) -> String {
        format!("type={}, sfi={}", "WX", self.sfi)
    }
}

fn create() -> Box<dyn Spot> {
    Box::new(DX { call: String::from("DF2MX") })
}


fn main() {
    println!("{}", create().to_string())
}

