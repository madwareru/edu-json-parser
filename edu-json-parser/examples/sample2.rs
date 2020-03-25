use edu_json_parser::{parse_json, Node};
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

impl CardData {
    pub fn parse_card(content: &str) -> Result<Self, String> {
        let json = parse_json(content);
        match json {
            Ok(valid_json) => Self::parse_node(&*valid_json),
            Err(error_data) => Err(format!("Error during parsing: {}", error_data.to_string())),
        }
    }

    pub fn parse_cards(content: &str) -> Result<Vec<Self>, String> {
        let json = parse_json(content);
        match json {
            Ok(valid_json) => Self::parse_nodes(&*valid_json),
            Err(error_data) => Err(format!("Error during parsing: {}", error_data.to_string())),
        }
    }

    fn parse_nodes(json: &Node) -> Result<Vec<Self>, String> {
        if !json.is_array() {
            return Err("Node is not array".to_string());
        }
        let length = json.len();
        let mut result_vec = Vec::with_capacity(length);
        for i in 0..length {
            match Self::parse_node(&*json[i]) {
                Ok(card) => result_vec.push(card),
                Err(error_text) => return Err(
                    format!("Error on parsing element {}: {}", i, error_text)
                ),
            }
        }
        Ok(result_vec)
    }

    fn parse_node(json: &Node) -> Result<Self, String> {
        if !json.is_dictionary() {
            return Err("Node is not a dictionary".to_string());
        }
        let name = match json.get_string("name") {
            Ok(ok_data) => ok_data,
            Err(err_data) => return Err(
                format!("failed to retrieve name: {}", err_data.to_string())
            )
        };
        let last_name = match json.get_string("last_name") {
            Ok(ok_data) => ok_data,
            Err(err_data) => return Err(
                format!("failed to retrieve last_name: {}", err_data.to_string())
            )
        };
        let age = match json.get_number("age")  {
            Ok(ok_data) => ok_data,
            Err(err_data) => return Err(
                format!("failed to retrieve age: {}", err_data.to_string())
            )
        };
        let weight = match json.get_number("weight")  {
            Ok(ok_data) => ok_data,
            Err(err_data) => return Err(
                format!("failed to retrieve weight: {}", err_data.to_string())
            )
        };
        let sizes = match json.get("sizes") {
            Ok(boxed_node) => {
                if !boxed_node.is_array() {
                    return Err(
                        "failed to retrieve sizes: node is not array".to_string()
                    );
                }
                if boxed_node.len() != 3 {
                    return Err(
                        "failed to retrieve sized: length aren't equal 3".to_string()
                    );
                }
                let breast_size = boxed_node[0].as_number();
                let belly_size = boxed_node[1].as_number();
                let booty_size = boxed_node[2].as_number();
                match (breast_size, belly_size, booty_size) {
                    (Some(breast), Some(belly), Some(booty)) => [breast, belly, booty],
                    _ => return Err(
                        "failed to retrieve sized: some of the array elements is not a number".to_string()
                    )
                }
            },
            Err(err_data) => return Err(
                format!("failed to retrieve weight: {}", err_data.to_string())
            )
        };
        Ok(CardData{ name, last_name, age, weight, sizes })
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
    let single_card = CardData::parse_card(SINGLE_CARD);
    let many_cards = CardData::parse_cards(MANY_CARDS);
    let spoiled_cards = CardData::parse_cards(SPOILED_CARDS);
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