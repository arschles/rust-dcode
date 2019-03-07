use core::result;
use serde_json;
use std::string;

pub type Number = serde_json::Number;
pub type Decoder<T> = fn(string::String) -> serde_json::Result<T>;

pub fn decode_string<T>(json: string::String, decoder: Decoder<T>) -> serde_json::Result<T> {
    let func = decoder;
    func(json)
}

pub const Str: Decoder<string::String> =
    |data: string::String| serde_json::from_str::<string::String>(data.as_str());

pub const Num: Decoder<Number> =
    |data: string::String| serde_json::from_str::<serde_json::Number>(data.as_str());

pub fn Field<T>(name: string::String, dcoder: Decoder<T>) -> Decoder<T> {
    |s: string::String| result::Result::Err("not yet implemented")
}
