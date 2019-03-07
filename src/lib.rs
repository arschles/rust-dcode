pub mod dcode;

#[cfg(test)]
mod tests {
    use super::dcode;
    use core::result::Result;
    use std::string::String;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn string() {
        let json = String::from("\"a\"");
        match dcode::decode_string::<String>(json, dcode::Str) {
            Result::Ok(a) => assert_eq!("a", a),
            Result::Err(e) => panic!("we expected \"a\" but got {}", e),
        }
    }

    #[test]
    fn int() {
        let json = String::from("1");
        match dcode::decode_string::<dcode::Number>(json, dcode::Num) {
            Result::Ok(a) => assert_eq!(serde_json::Number::from(1), a),
            Result::Err(e) => panic!("we expected 1, but got {}", e),
        }
    }
}
