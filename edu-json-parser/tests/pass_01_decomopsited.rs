#[cfg(test)]
mod tests {
    use edu_json_parser::parse_json;

    #[test]
    fn pass_0() { assert!(parse_json(r##""JSON Test Pattern pass1""##).is_ok()); }

    #[test]
    fn pass_1() { assert!(parse_json(r##"{"object with 1 member":["array with 1 element"]}"##).is_ok()); }

    #[test]
    fn pass_2() { assert!(parse_json(r##"{}"##).is_ok()); }

    #[test]
    fn pass_3() { assert!(parse_json(r##"[]"##).is_ok()); }

    #[test]
    fn pass_4() { assert!(parse_json(r##"-42"##).is_ok()); }

    #[test]
    fn pass_5() { assert!(parse_json(r##"true"##).is_ok()); }

    #[test]
    fn pass_6() { assert!(parse_json(r##"false"##).is_ok()); }

    #[test]
    fn pass_7() { assert!(parse_json(r##"null"##).is_ok()); }

    #[test]
    fn integer() { assert!(parse_json(r##"1234567890"##).is_ok()); }

    #[test]
    fn real() { assert!(parse_json(r##"-9876.543210"##).is_ok()); }

    #[test]
    fn one() { assert!(parse_json(r##"1"##).is_ok()); }

    #[test]
    fn zero() { assert!(parse_json(r##"0"##).is_ok()); }

    #[test]
    fn alpha() { assert!(parse_json(r##""abcdefghijklmnopqrstuvwyz""##).is_ok()); }

    #[test]
    fn alpha2() { assert!(parse_json(r##""ABCDEFGHIJKLMNOPQRSTUVWYZ""##).is_ok()); }

    #[test]
    fn digit() { assert!(parse_json(r##""0123456789""##).is_ok()); }

    #[test]
    fn special() { assert!(parse_json(r##""`1~!@#$%^&*()_+-={':[,]}|;.</>?""##).is_ok()); }

    #[test]
    fn space() { assert!(parse_json(r##"" ""##).is_ok()); }

    #[test]
    fn quote() { assert!(parse_json(r##""\"""##).is_ok()); }

    #[test]
    fn backslash() { assert!(parse_json("\"\\\\\"").is_ok()); }

    #[test]
    fn controls() { assert!(parse_json(r##""\b\f\n\r\t""##).is_ok()); }

    #[test]
    fn slash() { assert!(parse_json(r##""/ & \/""##).is_ok()); }

    #[test]
    fn exponent_0() { assert!(parse_json(r##"1.23456789E+34"##).is_ok()); }

    #[test]
    fn exponent_1() { assert!(parse_json("23456789012E66").is_ok()); }

    #[test]
    fn exponent_2() { assert!(parse_json(r##"0.123456789e-12"##).is_ok()); }

    #[test]
    fn hex() {assert!(parse_json(r#""\u0123\u4567\u89AB\uCDEF\uabcd\uef4A""#).is_ok());}

    #[test]
    fn hex_simple() {assert!(parse_json(r#""\u0123""#).is_ok());}

    #[test]
    fn test_on_twitter() { assert!(parse_json(include!("twitter.json")).is_ok()); }
}