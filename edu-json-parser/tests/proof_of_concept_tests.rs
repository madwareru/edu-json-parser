#[macro_use]
extern crate edu_json_parser;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use edu_json_parser::{parse_json, Node};
    use smol_str::SmolStr;

    const EPSILON: f64 = 0.00001;

    macro_rules! assert_eq_f64 {
        ($lvalue:expr, $rvalue:expr) => { assert!(($lvalue - $rvalue).abs() < EPSILON) }
    }

    #[test]
    fn it_works() {

        let x = parse_json("\"lol\"");
        assert_eq!("lol", &x.unwrap().as_string().unwrap());

        let x = parse_json("\"lol, \\\"it's nested\\\"\"");
        assert_eq!("lol, \"it's nested\"", x.unwrap().as_string().unwrap());

        let s = String::from(r#""""#);
        assert_eq!("", parse_json(&s).unwrap().as_string().unwrap());

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

        let z = String::from("[1, [1, false, \"say\"], \"say\"]");
        let array_node = parse_json(&z);
        assert_eq!(
            Ok(Node::Array(
                vec![
                    Node::Number(1.0),
                    Node::Array(
                        vec![
                            Node::Number(1.0),
                            Node::Boolean(false),
                            Node::String(SmolStr::from("say")),
                        ]
                    ),
                    Node::String(SmolStr::from("say")),
                ]
            )),
            array_node);
        let array_node = array_node.unwrap();
        assert!(array_node.is_array());
        let arr = array_node.as_array();
        assert!(arr.is_some());
        assert!(array_node.get_element_at(0).is_ok());
        assert!(array_node.get_element_at(0).unwrap().is_number());
        assert!(array_node.get_element_at(1).unwrap().is_array());
        assert!(array_node.get_element_at(2).unwrap().is_string());
        assert!(array_node.get_element_at(3).is_err());
        assert!(array_node.get("something").is_err());
        assert!(array_node[0].is_number());
        assert_eq!(3, array_node.len());

        let z = String::from("null");
        assert_eq!(Ok(Node::Null), parse_json(&z));
        let z = String::from("[]");
        assert_eq!(Ok(Node::Array(vec![])), parse_json(&z));
        let z = String::from("{}");
        assert_eq!(Ok(Node::Object(HashMap::new())), parse_json(&z));
        let z = String::from("{}abra");
        assert!(parse_json(&z).is_err());
    }

    #[test]
    fn test_null_parsing() {
        let node = parse_json("null");
        assert_eq!(Ok(Node::Null), node);
        let node = node.unwrap();
        assert!(node.is_null());
        assert!(!node.is_array());
        assert!(!node.is_object());
        assert!(!node.is_number());
        assert!(!node.is_bool());
        assert!(!node.is_string());
    }

    #[test]
    fn test_bool_parsing() {
        let (a, b, c) = (
            parse_json("true"),
            parse_json("false"),
            parse_json("42")
        );
        assert!(a.is_ok() && b.is_ok() && c.is_ok());
        let (a, b, c) = (a.unwrap(), b.unwrap(), c.unwrap());

        assert!(a.is_bool());
        assert!(b.is_bool());
        assert!(!c.is_bool());

        assert!(!a.is_array());
        assert!(!a.is_object());
        assert!(!a.is_number());
        assert!(!a.is_null());
        assert!(!a.is_string());

        assert!(!b.is_array());
        assert!(!b.is_object());
        assert!(!b.is_number());
        assert!(!b.is_null());
        assert!(!b.is_string());

        assert_eq!(Node::Boolean(true), a);
        assert_eq!(Node::Boolean(false), b);

        assert_eq!(None, c.as_bool());
        assert_eq!(Some(true), a.as_bool());
        assert_eq!(Some(false), b.as_bool());
    }

    #[test]
    fn test_number_parsing() {
        let (
            natural,
            negative,
            float,
            exponential,
            zero,
            one,
            non_number
        ) = (
            parse_json("123"),
            parse_json("-123"),
            parse_json("123.456"),
            parse_json("1.23e+2"),
            parse_json("0"),
            parse_json("1"),
            parse_json("null"),
        );
        assert!(natural.is_ok());
        assert!(negative.is_ok());
        assert!(float.is_ok());
        assert!(exponential.is_ok());
        assert!(zero.is_ok());
        assert!(one.is_ok());
        assert!(non_number.is_ok());

        let (
            natural,
            negative,
            float,
            exponential,
            zero,
            one,
            non_number
        ) = (
            natural.unwrap(),
            negative.unwrap(),
            float.unwrap(),
            exponential.unwrap(),
            zero.unwrap(),
            one.unwrap(),
            non_number.unwrap()
        );

        assert!(natural.is_number());
        assert!(negative.is_number());
        assert!(float.is_number());
        assert!(exponential.is_number());
        assert!(zero.is_number());
        assert!(one.is_number());
        assert!(!non_number.is_number());

        assert_eq_f64!(123.0, natural.as_number().unwrap());
        assert_eq_f64!(-123.0, negative.as_number().unwrap());
        assert_eq_f64!(123.456, float.as_number().unwrap());
        assert_eq_f64!(123.0, exponential.as_number().unwrap());
        assert_eq_f64!(0.0, zero.as_number().unwrap());
        assert_eq_f64!(1.0, one.as_number().unwrap());
        assert_eq!(None, non_number.as_number());
    }

    const STRING_TEST_CASES: [(&'static str, &'static str); 10] = [
        (r##""trivial""##, r##"trivial"##),
        (r##""hello \"nested\" ""##, r##"hello "nested" "##),
        (r##""\uabcd""##, "\u{ABCD}"),
        (r##""\n""##, "\n"),
        (r##""\r""##, "\r"),
        (r##""\\""##, "\\"),
        (r##""\/""##, "/"),
        (r##""/""##, "/"),
        (r##""\f""##, "\u{000c}"),
        (r##""\b""##, "\u{0008}")
    ];

    #[test]
    fn test_string_parsing() {
        for (source, expected) in STRING_TEST_CASES.iter() {
            assert_eq!(
                Ok(Node::String(SmolStr::from(*expected))),
                parse_json(*source)
            );
        }

        let node = parse_json(STRING_TEST_CASES[0].0);
        let number_node = parse_json("10");
        assert!(node.is_ok() && number_node.is_ok());
        let node = node.unwrap();
        let number_node = number_node.unwrap();

        assert!(node.is_string());
        assert!(!node.is_null());
        assert!(!node.is_number());
        assert!(!node.is_array());
        assert!(!node.is_object());
        assert!(!node.is_bool());

        assert_eq!(Some(SmolStr::from(STRING_TEST_CASES[0].1)), node.as_string());
        assert_eq!(None, node.as_number());
        assert_eq!(None, node.as_bool());
        assert_eq!(None, node.as_object());
        assert_eq!(None, node.as_array());

        assert_eq!(None, number_node.as_string());
        assert_eq!(Some("10".to_string()), number_node.to_string());
        assert_eq!(Some("false".to_string()), parse_json("false").unwrap().to_string());
        assert_eq!(Some("null".to_string()), parse_json("null").unwrap().to_string());
        assert_eq!(None, node.to_string());
    }

    #[test]
    fn test_object_parsing() {
        let obj_text = r#####"{
            "x": null,
            "y": false,
            "z": 123,
            "w": "some_string",
            "arr": [1, 2, 3],
            "obj": {"a": "Hello, ", "b": "world!"}
        }"#####;
        let obj_node = parse_json(obj_text);
        assert!(obj_node.is_ok());
        let obj_node = obj_node.unwrap();
        assert!(obj_node.is_object());
        assert!(obj_node.get_element_at(0).is_err());
        assert_eq!(6, obj_node.len());

        let parse_many_result = parse_many!{
            obj_node =>
            get("x"),
            get_bool("y"),
            get_number("z"),
            get_string("w"),
            get("arr"),
            get("obj")
        };
        assert!(parse_many_result.is_ok());
        let parse_many_result = parse_many_result.unwrap();

        assert!(parse_many_result.0.is_null());
        assert_eq!(false, parse_many_result.1);
        assert_eq!(123.0, parse_many_result.2);
        assert_eq!("some_string", parse_many_result.3);
        assert!(parse_many_result.4.is_array());
        assert!(parse_many_result.5.is_object());

        assert!(obj_node.get_string("z").is_err());
        assert!(obj_node.get_as_string("z").is_ok());
        assert!(obj_node.get_as_string("w").is_err());
        assert!(obj_node.get_as_string("obj").is_err());
        assert!(obj_node.get_as_string("arr").is_err());
        assert!(obj_node["x"].is_null());
    }
}
