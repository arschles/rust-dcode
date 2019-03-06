use std::option;

#[allow(dead_code)]
pub type Decoder<T> = fn(str) -> option::Option<T>;
