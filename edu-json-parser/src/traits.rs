use crate::{Node, parse_json};

pub trait Parsable {
    fn parse_node(json: &Node) -> Result<Self, String>
        where Self: Sized;
    fn parse_node_array(json: &Node) -> Result<Vec<Self>, String>
        where Self: Sized
    {
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
    fn parse(content: &str) -> Result<Self, String>
        where Self: Sized
    {
        let json = parse_json(content);
        match json {
            Ok(valid_json) => Self::parse_node(&*valid_json),
            Err(error_data) => Err(format!("Error during parsing: {}", error_data.to_string())),
        }
    }
    fn parse_array(content: &str) -> Result<Vec<Self>, String>
        where Self: Sized
    {
        let json = parse_json(content);
        match json {
            Ok(valid_json) => Self::parse_node_array(&*valid_json),
            Err(error_data) => Err(format!("Error during parsing: {}", error_data.to_string())),
        }
    }
}