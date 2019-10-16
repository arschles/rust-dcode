use core::result;
use serde_json;
use std::{boxed, string};

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

pub fn Field<'a, T>(name: string::String, dcoder: Decoder<T>) -> Decoder<T> {
    //Box<Decoder<T>> {
    let f = move |json| {
        let func = dcoder;
        let res = func(json);
        res
    };
    f
    // boxed::Box::new(f)
    // boxed::Box::new(move |json: string::String| {
    //     let func = dcoder;
    //     let res = func(json);
    //     res
    //     let matched = match res {
    //         result::Result::Ok(ret) => ret,
    //         result::Result::Err(e) => func(json),
    //     };
    // })
}
