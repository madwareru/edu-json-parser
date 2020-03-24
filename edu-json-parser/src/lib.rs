#[macro_use]
extern crate compre_combinee;
extern crate combine;

use std::collections::HashMap;
use combine::{parser, eof};
use combine::parser::range::{take_while1};
use combine::parser::char::*;
use combine::{Parser, many, optional, skip_many, sep_by, between};

#[derive(PartialEq, Clone, Debug)]
pub enum Node
{
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Box<Node>>),
    Dictionary(HashMap<String, Box<Node>>)
}

impl Node {
    pub fn is_null(&self) -> bool {
        *self == Node::Null
    }

    pub fn as_bool(&self) -> Option<bool> {
        if let Node::Boolean(b) = self {
            Some(*b)
        } else {
            None
        }
    }

    pub fn as_number(&self) -> Option<f64> {
        if let Node::Number(n) = self {
            Some(*n)
        } else {
            None
        }
    }

    pub fn as_string(&self) -> Option<String> {
        if let Node::String(s) = self {
            Some(s.clone())
        } else {
            None
        }
    }

    pub fn as_array(&self) -> Option<&Vec<Box<Node>>> {
        if let Node::Array(v) = self {
            Some(&v)
        } else {
            None
        }
    }

    pub fn as_dictionary(&self) -> Option<&HashMap<String, Box<Node>>> {
        if let Node::Dictionary(d) = self {
            Some(&d)
        } else {
            None
        }
    }

    pub fn get_element_at(&self, idx: usize) -> Option<Box<Node>> {
        if let Some(arr) = self.as_array() {
            if idx <= arr.len() {
                Some(arr[idx].clone())
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get(&self, key: &str) -> Option<Box<Node>> {
        if let Some(dict) = self.as_dictionary() {
            dict.get(key).map(|x| x.clone())
        } else {
            None
        }
    }
}

fn word_part<'a>() -> impl Parser<&'a str, Output = &'a str> {
    take_while1(|c: char| c != '\\' && c != '"')
}

fn escaped<'a>() -> impl Parser<&'a str, Output = &'a str> {
    string("\\\"")
}

fn string_part<'a>() -> impl Parser<&'a str, Output = Vec<&'a str>> {
    many(escaped().or(word_part()))
}

fn string_parser_inner<'a>() -> impl Parser<&'a str, Output = String> {
    c_hx_do! {
        __ <- char('"'),
        x  <- string_part(),
        ___ <- char('"');
        {
            let cap = x.iter().fold(0, |acc, s| acc + s.len());
            let mut str = String::with_capacity(cap);
            for s in x.iter() {
                str.push_str(s);
            }
            str
        }
    }
}

fn string_parser<'a>() -> impl Parser<&'a str, Output = Box<Node>> {
    string_parser_inner().map(|x| Box::new(Node::String(x)))
}

fn digit_sequence<'a>() -> impl Parser<&'a str, Output = String> {
    take_while1(|c: char| c >= '0' && c <= '9').map(|x| String::from(x))
}

fn trailing_digit_sequence<'a>() -> impl Parser<&'a str, Output = String> {
    c_hx_do! {
        __ <- char('.'),
        rest <- digit_sequence();
        rest
    }
}

fn number_parser<'a>() -> impl Parser<&'a str, Output = Box<Node>> {
    c_hx_do! {
        digs <- digit_sequence(),
        trail <- optional(trailing_digit_sequence());
        {
            let s = match trail {
                Some(t_digs) => format!("{}.{}", &digs, &t_digs),
                _ => digs.clone()
            };
            Box::new(Node::Number((&s).parse().unwrap()))
        }
    }
}

fn bool_parser<'a>() -> impl Parser<&'a str, Output = Box<Node>> {
    c_hx_do!{
        word <- string("true").or(string("false"));
        match word {
            "true" => Box::new(Node::Boolean(true)),
            _ => Box::new(Node::Boolean(false))
        }
    }
}

fn null_parser<'a>() -> impl Parser<&'a str, Output = Box<Node>> {
    c_hx_do!{
        _word <- string("null");
        Box::new(Node::Null)
    }
}

fn primitive_parser<'a>() -> impl Parser<&'a str, Output = Box<Node>> {
    let recursive_array_parser = parser(|input| {
        let _: &mut &str = input;
        array_parser().parse_stream(input).into_result()
    });

    let recursive_dict_parser = parser(|input| {
        let _: &mut &str = input;
        dictionary_parser().parse_stream(input).into_result()
    });

    let possible_parser = bool_parser()
        .or(number_parser())
        .or(string_parser())
        .or(null_parser())
        .or(recursive_array_parser)
        .or(recursive_dict_parser);

    c_hx_do! {
        __ <- skip_many(space()),
        pars <- possible_parser,
        ___ <- skip_many(space());
        pars
    }
}

fn array_parser<'a>() -> impl Parser<&'a str, Output = Box<Node>> {
    between(
        c_compre![c; c <- char('['), __ <- skip_many(space())],
        c_compre![c; __ <- skip_many(space()), c <- char(']')],
        optional(sep_by(primitive_parser(), char(',')))
    ).map(
        |nodes_opt: Option<Vec<Box<Node>>>| nodes_opt.map(
            |nodes| Box::new(Node::Array(nodes))
        ).unwrap()
    )
}

fn pair_parser<'a>() -> impl Parser<&'a str, Output = (String, Box<Node>)> {
    let str_parser = c_hx_do!{
        __ <- skip_many(space()),
        stp <- string_parser_inner(),
        ___ <- skip_many(space());
        stp
    };

    c_hx_do!{
        l <- str_parser,
        __ <- char(':'),
        r <- primitive_parser();
        (l, r)
    }
}

fn dictionary_parser<'a>() -> impl Parser<&'a str, Output = Box<Node>> {
    between(
        c_compre![c; c <- char('{'), __ <- skip_many(space())],
        c_compre![c; __ <- skip_many(space()), c <- char('}')],
        optional(sep_by(pair_parser(), char(',')))
    ).map(
        |nodes_opt: Option<Vec<(String, Box<Node>)>>| nodes_opt.map(
            |nodes| {
                let mut dict = HashMap::new();
                for i in 0..nodes.len() {
                    let (l, r) = nodes[i].clone();
                    dict.insert(l, r);
                }
                Box::new(
                    Node::Dictionary(
                        dict
                    )
                )
            }
        ).unwrap()
    )
}

fn json_parser<'a>() -> impl Parser<&'a str, Output = Box<Node>> {
    null_parser()
        .or(bool_parser())
        .or(number_parser())
        .or(string_parser())
        .or(array_parser())
        .or(dictionary_parser())
}

pub fn parse_json(content: &str) -> Result<Box<Node>, String> {
    let mut parser = c_hx_do!{
        json <- json_parser(),
        __ <- eof();
        json
    };
    let res = parser.parse(content);
    match res {
        Err(x) => Err(format!("{}", x.to_string())),
        Ok((res,_)) => Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::{Node, parse_json};

    #[test]
    fn it_works() {
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
        assert_eq!(Some(Box::new(Node::Number(1.0))), arr.get_element_at(0));
        assert_eq!(Some(Box::new(Node::Boolean(false))), arr.get_element_at(1));
        assert_eq!(Some(Box::new(Node::String("say".to_string()))), arr.get_element_at(2));

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
        assert_eq!(Some(Box::new(Node::Number(1.0))), dict.get("number"));
        assert_eq!(Some(Box::new(Node::Boolean(false))), dict.get("bool"));
        assert_eq!(Some(Box::new(Node::String("say".to_string()))), dict.get("string"));

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
