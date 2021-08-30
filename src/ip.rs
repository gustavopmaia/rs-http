use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Ip {
    address: String,
}

impl Ip {
    pub fn new(addr: &str) -> Ip {
        Ip {
            address: String::from(addr)
        }
    }
}