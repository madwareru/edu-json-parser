#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use edu_json_parser::{parse_json, Node};

    #[test]
    fn proof_of_concept() {
        let x = parse_json("\"lol\"");
        assert_eq!(Node::String("lol".to_string()), *x.unwrap());

        let x = parse_json("\"lol, \\\"it's nested\\\"\"");
        assert_eq!(Ok(Box::new(Node::String("lol, \\\"it's nested\\\"".to_string()))), x);

        let s = String::from(r#""who let the dogs out?""#);
        assert_eq!(Node::String("who let the dogs out?".to_string()), *parse_json(&s).unwrap());

        let s = String::from(r#""""#);
        assert_eq!(Node::String("".to_string()), *parse_json(&s).unwrap());

        let z = String::from("123");
        assert_eq!(Node::Number(123.0), *parse_json(&z).unwrap());

        let z = String::from("123.767");
        assert_eq!(Node::Number(123.767), *parse_json(&z).unwrap());

        let z = String::from("123.767f");
        assert!(if let Err(_) = parse_json(&z){true}else{false});

        let z = String::from("true");
        assert_eq!(Node::Boolean(true), *parse_json(&z).unwrap());

        let z = String::from("false");
        assert_eq!(Node::Boolean(false), *parse_json(&z).unwrap());

        let z = String::from("falshe");
        assert!(if let Err(_) = parse_json(&z){true}else{false});

        let z = String::from("[1, false, \"say\"]");
        let arr = *parse_json(&z).unwrap();
        assert_eq!(Some(3), arr.as_array().map(|a| a.len()));
        assert_eq!(Ok(Box::new(Node::Number(1.0))), arr.get_element_at(0));
        assert_eq!(Ok(Box::new(Node::Boolean(false))), arr.get_element_at(1));
        assert_eq!(Ok(Box::new(Node::String("say".to_string()))), arr.get_element_at(2));

        let z = String::from("[1, [1, false, \"say\"], \"say\"]");
        assert_eq!(
            Ok(Box::new(Node::Array(
                vec![
                    Box::new(Node::Number(1.0)),
                    Box::new(Node::Array(
                        vec![
                            Box::new(Node::Number(1.0)),
                            Box::new(Node::Boolean(false)),
                            Box::new(Node::String("say".to_string())),
                        ]
                    )),
                    Box::new(Node::String("say".to_string())),
                ]
            ))),
            parse_json(&z));

        let z = String::from("{\"number\": 1, \"bool\": false, \"string\": \"say\"}");
        let dict = *(parse_json(&z).unwrap());
        assert_eq!(Ok(Box::new(Node::Number(1.0))), dict.get("number"));
        assert_eq!(Ok(Box::new(Node::Boolean(false))), dict.get("bool"));
        assert_eq!(Ok(Box::new(Node::String("say".to_string()))), dict.get("string"));

        let z = String::from("null");
        assert_eq!(Ok(Box::new(Node::Null)), parse_json(&z));
        let z = String::from("[]");
        assert_eq!(Ok(Box::new(Node::Array(vec![]))), parse_json(&z));
        let z = String::from("{}");
        assert_eq!(Ok(Box::new(Node::Dictionary(HashMap::new()))), parse_json(&z));
        let z = String::from("{}abra");
        assert!(if let Err(_) = parse_json(&z){ true } else { false });
    }
}
