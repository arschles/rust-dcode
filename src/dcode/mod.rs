use std::{option, string};
use serde_json::{Result, Value};
use super::Decoder;

type Decoder<T> = fn(string::String) -> option::Option<T>;

pub fn decode_string<T>(
    json: string::String,
    decoder: Decoder<T>,
) -> option::Option<T> {
    let func = decoder.func;
    func(json)
}

pub const Str: Decoder<string::String> = |data: string::String| {    
    let v: Value = serde_json::from_str(data);
    match v {
        Value::String(s) => option::Option::Some(s.to_str()),
        _ => option::Option::None,
    }
};

const Num: Decoder<Number> = |data: string::String| {
    let v: Value = serde_json::from_str(data);
    match v {
        Value::Number(n) => std::option::Option::Some(n),
        _ => std::option::Option::None,
    }
};

fn Field<T>(name: string::String, dcoder: Decoder<T>) -> Decoder<T> {
    |s: string::String| {
        option::None
    }
}
