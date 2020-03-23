#[macro_use]
extern crate compre_combinee;
extern crate combine;

use std::collections::HashMap;
use combine::{parser, eof};
use combine::parser::range::{take_while1};
use combine::parser::char::*;
use combine::{Parser, many, optional, many1, token, skip_many, skip_many1, sep_by, between};
use std::convert::TryFrom;
use combine::stream::easy;

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
    c_monadde! {
        char('"') => __ |>
        string_part().skip(char('"')) => x |>
        x.iter().fold(
            String::new(),
            |mut acc, s| { acc.push_str(s); acc }
        )
    }

    // c_hx_do! {
    //     __ <- char('"'),
    //     x  <- string_part().skip(char('"'));
    //     x.iter().fold(
    //         String::new(),
    //         |mut acc, s| { acc.push_str(s); acc }
    //     )
    // }
}

fn string_parser<'a>() -> impl Parser<&'a str, Output = Box<Node>> {
    string_parser_inner().map(|x| Box::new(Node::String(x)))
}

fn digit_sequence<'a>() -> impl Parser<&'a str, Output = String> {
    many1(digit()).map(|x: Vec<char>| x.iter().fold(String::new(), |mut acc, c| { acc.push(*c); acc} ))
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
        word <- string("null");
        Box::new(Node::Null)
    }
}

fn primitive_parser<'a>() -> impl Parser<&'a str, Output = Box<Node>> {
    let many_space1 = skip_many(space());
    let many_space2 = skip_many(space());

    let recursive_array_parser = parser(|input| {
        let _: &mut &str = input;
        array_parser().parse_stream(input).into_result()
    });

    let recursive_dict_parser = parser(|input| {
        let _: &mut &str = input;
        dictionary_parser().parse_stream(input).into_result()
    });

    many_space1
    .with(bool_parser()
        .or(number_parser())
        .or(string_parser())
        .or(null_parser())
        .or(recursive_array_parser)
        .or(recursive_dict_parser))
    .skip(many_space2)
}

fn array_parser<'a>() -> impl Parser<&'a str, Output = Box<Node>> {
    between(
        char('[').skip(skip_many(space())),
        skip_many(space()).with(char(']')),
        optional(sep_by(primitive_parser(), char(',')))
    ).map(
        |nodes_opt: Option<Vec<Box<Node>>>| nodes_opt.map(
            |nodes| Box::new(Node::Array(nodes))
        ).unwrap()
    )
}

fn pair_parser<'a>() -> impl Parser<&'a str, Output = (String, Box<Node>)> {
    let many_space1 = skip_many(space());
    let many_space2 = skip_many(space());

    let lhs = many_space1
        .with(string_parser_inner())
        .skip(many_space2);

    let rhs = primitive_parser();

    lhs.skip(char(':')).and(rhs)
}

fn dictionary_parser<'a>() -> impl Parser<&'a str, Output = Box<Node>> {
    between(
        char('{').skip(skip_many(space())),
        skip_many(space()).with(char('}')),
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

pub fn parse_json(content: &str) -> Option<Box<Node>> {
    json_parser().skip(eof()).parse(content).map(|(res,_)| res).ok()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::{Node, parse_json};

    #[test]
    fn it_works() {
        let x = parse_json("\"lol\"");
        assert_eq!(Node::String("lol".to_string()), *x.unwrap());

        let x = parse_json("\"lol");
        assert_eq!(None, x);

        let x = parse_json("\"lol, \\\"it's nested\\\"\"");
        assert_eq!(Some(Box::new(Node::String("lol, \\\"it's nested\\\"".to_string()))), x);

        let s = String::from(r#""who let the dogs out?""#);
        assert_eq!(Node::String("who let the dogs out?".to_string()), *parse_json(&s).unwrap());

        let s = String::from(r#""""#);
        assert_eq!(Node::String("".to_string()), *parse_json(&s).unwrap());

        let z = String::from("123");
        assert_eq!(Node::Number(123.0), *parse_json(&z).unwrap());

        let z = String::from("123.767");
        assert_eq!(Node::Number(123.767), *parse_json(&z).unwrap());

        let z = String::from("true");
        assert_eq!(Node::Boolean(true), *parse_json(&z).unwrap());

        let z = String::from("false");
        assert_eq!(Node::Boolean(false), *parse_json(&z).unwrap());

        let z = String::from("falshe");
        assert_eq!(None, parse_json(&z));

        let z = String::from("[1, false, \"say\"]");
        assert_eq!(
            Some(Box::new(Node::Array(
                vec![
                    Box::new(Node::Number(1.0)),
                    Box::new(Node::Boolean(false)),
                    Box::new(Node::String("say".to_string())),
                ]
            ))),
            parse_json(&z));

        let z = String::from("[1, [1, false, \"say\"], \"say\"]");
        assert_eq!(
            Some(Box::new(Node::Array(
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
        match dict {
            Node::Dictionary(dictionary) => {
                assert_eq!(Box::new(Node::Number(1.0)), dictionary["number"]);
                assert_eq!(Box::new(Node::Boolean(false)), dictionary["bool"]);
                assert_eq!(Box::new(Node::String("say".to_string())), dictionary["string"]);
            },
            _ => assert_eq!(2, 3)
        }

        let z = String::from("null");
        assert_eq!(Some(Box::new(Node::Null)), parse_json(&z));
        let z = String::from("[]");
        assert_eq!(Some(Box::new(Node::Array(vec![]))), parse_json(&z));
        let z = String::from("{}");
        assert_eq!(Some(Box::new(Node::Dictionary(HashMap::new()))), parse_json(&z));
        let z = String::from("{}abra");
        assert_eq!(None, parse_json(&z));
    }
}
