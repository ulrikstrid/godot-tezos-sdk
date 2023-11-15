use godot::prelude::*;
use tezos_michelson::{micheline::Micheline, michelson::Michelson};

use hex;
use serde_json::{from_str, json};

#[derive(GodotClass)]
#[class(base=Node)]
pub struct TezosMichelson {}

#[godot_api]
impl TezosMichelson {
    #[func]
    fn unpack(text: String) -> String {
        let bytes = hex::decode(text).unwrap();
        // Skip first byte
        let value: Micheline = bytes[1..].try_into().unwrap();
        json!(value).to_string()
    }

    #[func]
    fn pack(text: String) -> String {
        let json: Micheline = from_str(&text).unwrap();
        let m: Michelson = json.try_into().unwrap();
        let bytes: Vec<u8> = m.pack(None).unwrap();
        hex::encode(bytes)
    }
}
