
use std::ops::Deref;
use std::collections::HashMap;

include!(concat!(env!("OUT_DIR"), "/charmaps.rs"));

#[derive(Debug)]
pub struct Encoder {
    table: Option<&'static HashMap<u16, u8>>,
}

impl Encoder {
    pub fn new(from_code: &str) -> Encoder {
        Encoder{
            table: get_encode_map(from_code),
        }
    }
}
