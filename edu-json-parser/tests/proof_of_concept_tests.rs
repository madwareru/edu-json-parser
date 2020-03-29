#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use edu_json_parser::{parse_json, Node};

    #[test]
    fn it_works() {
        const EPSILON: f64 = 0.00001;

        macro_rules! assert_eq_f64 {
            ($lvalue:expr, $rvalue:expr) => { assert!(($lvalue - $rvalue).abs() < EPSILON) }
        }

        let x = parse_json("\"lol\"");
        assert_eq!(Node::String("lol".to_string()), x.unwrap());

        let x = parse_json("\"lol, \\\"it's nested\\\"\"");
        assert_eq!(Ok(Node::String("lol, \"it's nested\"".to_string())), x);

        let s = String::from(r#""who let the dogs out?""#);
        assert_eq!(Node::String("who let the dogs out?".to_string()), parse_json(&s).unwrap());

        let s = String::from(r#""""#);
        assert_eq!(Node::String("".to_string()), parse_json(&s).unwrap());

        let z = String::from("123");
        assert_eq_f64!(parse_json(&z).unwrap().as_number().unwrap(), 123.0);

        let z = String::from("123.767");
        assert_eq_f64!(parse_json(&z).unwrap().as_number().unwrap(), 123.767);

        let z = String::from("1.23767E+2");
        assert_eq_f64!(parse_json(&z).unwrap().as_number().unwrap(), 123.767);

        let z = String::from("1.23767E2");
        assert_eq_f64!(parse_json(&z).unwrap().as_number().unwrap(), 123.767);

        let z = String::from("1237.67e-1");
        assert_eq_f64!(parse_json(&z).unwrap().as_number().unwrap(), 123.767);

        let z = String::from("-123.767");
        assert_eq_f64!(parse_json(&z).unwrap().as_number().unwrap(), -123.767);

        let z = String::from("123.767f");
        assert!(parse_json(&z).is_err());

        let z = String::from("true");
        assert_eq!(Node::Boolean(true), parse_json(&z).unwrap());

        let z = String::from("false");
        assert_eq!(Node::Boolean(false), parse_json(&z).unwrap());

        let z = String::from("falshe");
        assert!(parse_json(&z).is_err());

        let z = String::from("[1, false, \"say\"]");
        let arr = parse_json(&z).unwrap();
        assert_eq!(Some(3), arr.as_array().map(|a| a.len()));
        assert_eq!(Ok(Node::Number(1.0)), arr.get_element_at(0));
        assert_eq!(Ok(Node::Boolean(false)), arr.get_element_at(1));
        assert_eq!(Ok(Node::String("say".to_string())), arr.get_element_at(2));

        let z = String::from("[1, [1, false, \"say\"], \"say\"]");
        assert_eq!(
            Ok(Node::Array(
                vec![
                    Node::Number(1.0),
                    Node::Array(
                        vec![
                            Node::Number(1.0),
                            Node::Boolean(false),
                            Node::String("say".to_string()),
                        ]
                    ),
                    Node::String("say".to_string()),
                ]
            )),
            parse_json(&z));

        let z = String::from("null");
        assert_eq!(Ok(Node::Null), parse_json(&z));
        let z = String::from("[]");
        assert_eq!(Ok(Node::Array(vec![])), parse_json(&z));
        let z = String::from("{}");
        assert_eq!(Ok(Node::Dictionary(HashMap::new())), parse_json(&z));
        let z = String::from("{}abra");
        assert!(if let Err(_) = parse_json(&z){ true } else { false });
    }
}
