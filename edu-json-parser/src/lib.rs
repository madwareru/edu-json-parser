#[macro_use]
extern crate compre_combinee;
extern crate combine;

mod errors;
mod details;

use std::collections::HashMap;
use combine::{parser, eof};
use combine::parser::range::{take_while1};
use combine::parser::char::*;
use combine::{Parser, many, optional, skip_many, sep_by, between};

pub use crate::errors::ErrorCause;
pub use crate::details::Node;

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