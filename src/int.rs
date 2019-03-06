use std::option;
use serde_json::{Result, Value, Number}

const Num: Decoder<Number> = fn(data: str) -> option::Option<Number> {
    let v: Value = serde_json::from_str(data);
    match v {
        Value::Number(n) => std::option::Option::Some(n);
        _ => std::option::Option::None;
    }
}
