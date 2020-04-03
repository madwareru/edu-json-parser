#[macro_use]
extern crate compre_combinee;
extern crate combine;

mod errors;
mod details;
mod traits;
mod stop_watch;

use std::collections::HashMap;
use combine::{parser, eof, satisfy, choice, attempt};
use combine::parser::range::{take_while1};
use combine::parser::char::*;
use combine::{Parser, many, optional, skip_many, sep_by, between};

pub use crate::errors::ErrorCause;
pub use crate::details::Node;
pub use crate::traits::*;
use std::{f64, mem, slice, str};
use std::convert::TryFrom;
use smol_str::SmolStr;

fn parse_hex<'a>() -> impl Parser<&'a str, Output = u32> {
    satisfy(|c: char|
        (c >= '0' && c <= '9') ||
        (c >= 'a' && c <= 'f') ||
        (c >= 'A' && c <= 'F')
    ).map(|c| if c >= '0' && c <= '9' {
            c as u64 - '0' as u64
        } else if c >= 'a' && c <= 'f' {
            10 + c as u64 - 'a' as u64
        } else {
            10 + c as u64 - 'A' as u64
        } as u32
    )
}

fn unicode_char<'a>() -> impl Parser<&'a str, Output = Option<char>> {
    c_hx_do!{
        __ <- string(r#"\u"#),
        d3 <- parse_hex(),
        d2 <- parse_hex(),
        d1 <- parse_hex(),
        d0 <- parse_hex();
        {
            let unicode = d0 +
                10 * d1 +
                100 * d2 +
                1000 * d3;
            char::try_from(unicode).ok()
        }
    }
}

#[derive(PartialEq)]
enum StringPiece<'a >
{
    Ref(&'a str),
    Char(Option<char>)
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

fn string_part<'a>() -> impl Parser<&'a str, Output = Vec<StringPiece<'a >>> {
    many(
        choice(
            (
                attempt(take_while1(|c: char| c != '\\' && c != '"' && c != '\n' && c != '\r' && c != '\t')
                    .map(|chars: &str| StringPiece::Ref(chars))),
                attempt(string("\\\"").map(|_|StringPiece::Ref("\""))),
                attempt(string("\\\\").map(|_|StringPiece::Ref("\\"))),
                attempt(string("\\n").map(|_|StringPiece::Ref("\n"))),
                attempt(string("\\t").map(|_|StringPiece::Ref("\t"))),
                attempt(string("\\/").map(|_|StringPiece::Ref("/"))),
                attempt(string("\\r").map(|_|StringPiece::Ref("\r"))),
                attempt(string("\\f").map(|_|StringPiece::Ref("\u{000c}"))),
                attempt(string("\\b").map(|_|StringPiece::Ref("\u{0008}"))),
                attempt(unicode_char().map(|s|StringPiece::Char(s))),
            )
        )
    )
}

fn string_parser_inner<'a>() -> impl Parser<&'a str, Output = SmolStr> {
    c_hx_do! {
        x <- between(char('"'), char('"'), string_part());
        {
            let cap = x.iter().fold(0, |acc, s|
                acc +
                match s {
                    StringPiece::Ref(strref) => strref.len(),
                    StringPiece::Char(c) => c.map(|c_inner| c_inner.len_utf8()).unwrap_or(0)
                }
            );
            if cap <= 22 {
                let mut buf: [u8; 22] = [0; 22];
                let mut offset = 0;
                for s in x.iter() {
                    match s {
                        StringPiece::Ref(strref) => {
                            for &b in strref.as_bytes() {
                                buf[offset] = b;
                                offset += 1;
                            }
                        },
                        StringPiece::Char(c) => {
                            if let Some(chr) = c {
                                chr.encode_utf8(&mut buf[offset..]);
                                offset += chr.len_utf8();
                            }
                        }
                    }
                }
                return unsafe {
                    let str = str::from_utf8_unchecked(&buf[0..cap]);
                    SmolStr::new(str)
                };
            }
            let mut str = String::with_capacity(cap);
            for s in x.iter() {
                match s {
                    StringPiece::Ref(strref) => str.push_str(strref),
                    StringPiece::Char(c) => if let Some(chr) = c { str.push(*chr); }
                }
            }
            SmolStr::new(str)
        }
    }
}

fn string_parser<'a>() -> impl Parser<&'a str, Output = Node> {
    string_parser_inner().map(|x| Node::String(x))
}

fn digit_sequence<'a>() -> impl Parser<&'a str, Output = &'a str> {
    take_while1(|c: char| c >= '0' && c <= '9')
}

#[inline(always)]
fn power(lhs: f64, rhs: f64) -> f64 {
    lhs.powf(rhs)
}

fn trailing_digit_sequence<'a>() -> impl Parser<&'a str, Output = &'a str> {
    c_hx_do! {
        __ <- char('.'),
        rest <- digit_sequence();
        rest
    }
}

fn exponent_parser<'a>() -> impl Parser<&'a str, Output = f64> {
    c_hx_do!{
        __ <- satisfy(|c: char| c == 'e' || c == 'E'),
        sign_char <- optional(satisfy(|c: char| c == '+' || c == '-')),
        digits <- digit_sequence();
        {
            let sign = match sign_char {
                Some('-') => -1.0,
                _ => 1.0
            };
            let mut acc = 0;
            for c in digits.as_bytes() {
                acc = acc * 10 + (c - b'0') as u64;
            }
            power(10.0, sign * acc as f64)
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
enum NumberPrefix<'a >
{
    LeadingZero,
    Digits(char, &'a str)
}

fn leading_zero_parser <'a>() -> impl Parser<&'a str, Output = NumberPrefix<'a >> {
    char('0').map(|_| NumberPrefix::LeadingZero)
}

fn leading_digits_parser <'a>() -> impl Parser<&'a str, Output = NumberPrefix<'a >> {
    c_hx_do! {
        leading_digit <- satisfy(|c: char| c >= '1' && c <= '9'),
        digs <- optional(digit_sequence());
        NumberPrefix::Digits(leading_digit, digs.unwrap_or(""))
    }
}

fn leading_parser <'a>() -> impl Parser<&'a str, Output = NumberPrefix<'a >> {
    choice((
        attempt(leading_digits_parser()),
        attempt(leading_zero_parser()),
    ))
}

fn number_parser<'a>() -> impl Parser<&'a str, Output = Node> {
    c_hx_do! {
        minus_sign <- optional(char('-')),
        leading <- leading_parser(),
        trail <- optional(trailing_digit_sequence()),
        exp <- optional(exponent_parser());
        {
            Node::Number({
                let mut acc = match leading {
                    NumberPrefix::LeadingZero => 0.0,
                    NumberPrefix::Digits(leading_digit, l_digs) => {
                        let mut l = (leading_digit as u8 - b'0') as u64;
                        for c in l_digs.as_bytes() {
                            l =  l * 10 + (c  - b'0') as u64;
                        }
                        l as f64
                    }
                };
                if let Some(t_digs) = trail {
                    let mut divider = 1.0;
                    for c in t_digs.as_bytes() {
                        divider /= 10.0;
                        acc += (c  - b'0') as f64 * divider;
                    }
                }
                if let Some(exponent) = exp {
                    acc *= exponent;
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

fn array_parser<'a>() -> impl Parser<&'a str, Output = Node> {
    braced_parser(
        char('['),
        sep_by(primitive_parser(), char(',')),
        char(']')
    ).map(|nodes: Vec<Node>|
        Node::Array(nodes)
    )
}

fn pair_parser<'a>() -> impl Parser<&'a str, Output = Option<(SmolStr, Node)>> {
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
        Some((l, r))
    }
}

fn dictionary_parser<'a>() -> impl Parser<&'a str, Output = Node> {
    braced_parser(
        char('{'),
        sep_by(pair_parser(), char(',')),
        char('}')
    ).map(|mut nodes: Vec<Option<(SmolStr, Node)>>| {
        let mut dict = HashMap::with_capacity(nodes.len());
        for i in 0..nodes.len() {
            let (l, r) = mem::replace(&mut nodes[i], None).unwrap();
            dict.insert(l, r);
        }
        Node::Dictionary(
            dict
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