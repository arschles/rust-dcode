pub mod dcode;

#[cfg(test)]
mod tests {
    use super::dcode;
    use std::string::String;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn string() {
        let json = String::from("\"a\"");
        let a = dcode::decode_string(json, dcode::Str());
        assert_eq!(Some("a"), a);
    }
}
