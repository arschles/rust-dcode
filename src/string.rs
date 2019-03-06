use std::option;
use serde_json::{Result, Value}

const String: Decoder<str> = fn(data: str) -> option::Option<str> {
    let v: Value = serde_json::from_str(data);
    match v {
        Value::String(s) => std::option::Option::Some(s.to_str());
        _ => std::option::Option::None;
    }
}
