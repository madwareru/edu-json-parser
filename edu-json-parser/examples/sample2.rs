#[macro_use]
extern crate edu_json_parser;

use edu_json_parser::{Node, Parsable};
use std::fmt::{Display, Formatter};


#[derive(Debug)]
pub struct CardData {
    pub name: String,
    pub last_name: String,
    pub age: f64,
    pub weight: f64,
    pub sizes: [f64;3]
}

impl Display for CardData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(name: {}, last_name: {}, age: {}, weight: {}, sizes: [{}, {}, {}])",
            self.name,
            self.last_name,
            self.age,
            self.weight,
            self.sizes[0],
            self.sizes[1],
            self.sizes[2]
        )
    }
}

impl Parsable for CardData {
    fn parse_node(json: &Node) -> Result<Self, String> {
        if !json.is_object() {
            return Err("Node is not a dictionary".to_string());
        }
        match parse_many!(json =>
            get_string("name"),
            get_string("last_name"),
            get_number("age"),
            get_number("weight"),
            get("sizes")
        ) {
            Ok((name, last_name, age, weight, sizes_node)) => {
                let sizes = {
                    if !sizes_node.is_array() {
                        return Err(
                            "failed to retrieve sizes: node is not array".to_string()
                        );
                    }
                    if sizes_node.len() != 3 {
                        return Err(
                            "failed to retrieve sizes: length aren't equal 3".to_string()
                        );
                    }
                    let breast_size = sizes_node[0].as_number();
                    let belly_size = sizes_node[1].as_number();
                    let booty_size = sizes_node[2].as_number();
                    match (breast_size, belly_size, booty_size) {
                        (Some(breast), Some(belly), Some(booty)) => [breast, belly, booty],
                        _ => return Err(
                            "failed to retrieve sizes: some of the array elements is not a number".to_string()
                        )
                    }
                };
                Ok(CardData{
                    name: name.to_string(),
                    last_name: last_name.to_string(),
                    age,
                    weight,
                    sizes
                })
            },
            Err(err_data) => Err(format!("parse failed: {}", err_data.to_string())),
        }
    }
}

const SINGLE_CARD: &'static str =
    r#"{"name": "Santa", "last_name": "Clous", "age": 99, "weight": 150, "sizes":[120, 120, 120] }"#;
const MANY_CARDS: &'static str =
    r#"
    [
        {"name": "Santa", "last_name": "Clous", "age": 99, "weight": 150, "sizes":[120, 120, 120] },
        {"name": "John ", "last_name": "Snow", "age": 13, "weight": 60, "sizes":[90, 60, 90] }
    ]
    "#;

const SPOILED_CARDS: &'static str =
    r#"
    [
        {"name": "John ", "last_name": "Snow", "age": 13, "weight": 60, "sizes":[90, 60, 90] },
        {"name": "Jazz", "last_name": "Spoily", "age": 10, "weight": 25 },
        {"name": "Vitaly ", "last_name": "Samotokin", "age": 29, "weight": 94, "sizes":[110, 90, 110] }
    ]
    "#;

fn main() {
    let single_card = CardData::parse(SINGLE_CARD);
    let many_cards = CardData::parse_array(MANY_CARDS);
    let spoiled_cards = CardData::parse_array(SPOILED_CARDS);
    match single_card {
        Ok(card) => println!("This card contains: {}", card),
        Err(error) => println!("Oops! Error on parse single card! {}", error),
    }
    match many_cards {
        Ok(cards) => {
            for (i, card) in cards.iter().enumerate() {
                println!("Card {} contains: {}", i, card)
            }
        },
        Err(error) => println!("Oops! Error on parse many cards! {}", error),
    }
    match spoiled_cards {
        Ok(cards) => {
            for (i, card) in cards.iter().enumerate() {
                println!("Card {} contains: {}", i, card)
            }
        },
        Err(error) => println!("Oops! Error on parse spoiled cards! {}", error),
    }
}