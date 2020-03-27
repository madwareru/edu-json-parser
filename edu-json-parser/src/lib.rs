#[macro_use]
extern crate compre_combinee;
extern crate combine;

mod errors;
mod details;
mod traits;

use std::collections::HashMap;
use combine::{parser, eof};
use combine::parser::range::{take_while1};
use combine::parser::char::*;
use combine::{Parser, many, optional, skip_many, sep_by, between};

pub use crate::errors::ErrorCause;
pub use crate::details::Node;
pub use crate::traits::*;
use std::rc::Rc;


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

fn string_parser<'a>() -> impl Parser<&'a str, Output = Node> {
    string_parser_inner().map(|x| Node::String(x))
}

fn digit_sequence<'a>() -> impl Parser<&'a str, Output = &'a str> {
    take_while1(|c: char| c >= '0' && c <= '9')
}

fn trailing_digit_sequence<'a>() -> impl Parser<&'a str, Output = &'a str> {
    c_hx_do! {
        __ <- char('.'),
        rest <- digit_sequence();
        rest
    }
}

fn number_parser<'a>() -> impl Parser<&'a str, Output = Node> {
    c_hx_do! {
        minus_sign <- optional(char('-')),
        digs <- digit_sequence(),
        trail <- optional(trailing_digit_sequence());
        {
            Node::Number({
                let mut acc = 0.0;
                for c in digs.chars() {
                    acc *= 10.0;
                    acc += (c as i64 - '0' as i64) as f64;
                }
                if let Some(t_digs) = trail {
                    let mut divider = 1.0;
                    for c in t_digs.chars() {
                        divider /= 10.0;
                        acc += (c as i64 - '0' as i64) as f64 * divider;
                    }
                }
                if let Some(_) = minus_sign {
                    -acc
                } else {
                    acc
                }
            })
        }
    }
}

fn bool_parser<'a>() -> impl Parser<&'a str, Output = Node> {
    c_hx_do!{
        word <- string("true").or(string("false"));
        match word {
            "true" => Node::Boolean(true),
            _ => Node::Boolean(false)
        }
    }
}

fn null_parser<'a>() -> impl Parser<&'a str, Output = Node> {
    c_hx_do!{
        _word <- string("null");
        Node::Null
    }
}

macro_rules! ref_parser {
    ($parser_fn:ident) => {
        parser(|input| {
            let _: &mut &str = input;
            $parser_fn().parse_stream(input).into_result()
        })
    }
}

fn primitive_parser<'a>() -> impl Parser<&'a str, Output = Node> {
    let possible_parser = bool_parser()
        .or(number_parser())
        .or(string_parser())
        .or(null_parser())
        .or(ref_parser!(array_parser))
        .or(ref_parser!(dictionary_parser));

    c_hx_do! {
        __ <- skip_many(space()),
        pars <- possible_parser,
        ___ <- skip_many(space());
        pars
    }
}

fn braced_parser<'a, PBL, P, PBR, O>(pbl: PBL, p: P, pbr: PBR) -> impl Parser<&'a str, Output = O>
    where
        PBL: Parser<&'a str>,
        PBR: Parser<&'a str>,
        P: Parser<&'a str, Output = O>
{
    between(
        c_compre![c; c <- pbl, __ <- skip_many(space())],
        c_compre![c; __ <- skip_many(space()), c <- pbr],
        p
    )
}

fn array_parser<'a>() -> impl Parser<&'a str, Output = Node> {
    braced_parser(
        char('['),
        sep_by(primitive_parser(), char(',')),
        char(']')
    ).map(|nodes: Vec<Node>|
        Node::Array(Rc::from(nodes))
    )
}

fn pair_parser<'a>() -> impl Parser<&'a str, Output = (String, Node)> {
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

fn dictionary_parser<'a>() -> impl Parser<&'a str, Output = Node> {
    braced_parser(
        char('{'),
        sep_by(pair_parser(), char(',')),
        char('}')
    ).map(|nodes: Vec<(String, Node)>| {
        let mut dict = HashMap::with_capacity(nodes.len());
        for i in 0..nodes.len() {
            let (l, r) = nodes[i].clone();
            dict.insert(l, r);
        }
        Node::Dictionary(
            Rc::from(dict)
        )
    })
}

fn json_parser<'a>() -> impl Parser<&'a str, Output = Node> {
    null_parser()
        .or(bool_parser())
        .or(number_parser())
        .or(string_parser())
        .or(array_parser())
        .or(dictionary_parser())
}

pub fn parse_json(content: &str) -> Result<Node, String> {
    let mut parser = c_hx_do!{
        __ <- skip_many(space()),
        json <- json_parser(),
        ___ <- skip_many(space()),
        ____ <- eof();
        json
    };
    let res = parser.parse(content);
    match res {
        Err(x) => Err(format!("{}", x.to_string())),
        Ok((res,_)) => Ok(res)
    }
}