use std::collections::HashMap;
use crate::errors::ErrorCause;
use crate::errors::ErrorCause::*;
use std::ops::Index;

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

    pub fn is_bool(&self) -> bool {
        if let Node::Boolean(_) = self {
            true
        } else {
            false
        }
    }

    pub fn as_number(&self) -> Option<f64> {
        if let Node::Number(n) = self {
            Some(*n)
        } else {
            None
        }
    }

    pub fn is_number(&self) -> bool {
        if let Node::String(_) = self {
            true
        } else {
            false
        }
    }

    pub fn as_string(&self) -> Option<String> {
        if let Node::String(s) = self {
            Some(s.clone())
        } else {
            None
        }
    }

    pub fn is_string(&self) -> bool {
        if let Node::String(_) = self {
            true
        } else {
            false
        }
    }

    pub fn as_array(&self) -> Option<&Vec<Box<Node>>> {
        if let Node::Array(v) = self {
            Some(&v)
        } else {
            None
        }
    }

    pub fn is_array(&self) -> bool {
        self.as_array().map(|_| true).unwrap_or(false)
    }

    pub fn as_dictionary(&self) -> Option<&HashMap<String, Box<Node>>> {
        if let Node::Dictionary(d) = self {
            Some(&d)
        } else {
            None
        }
    }

    pub fn is_dictionary(&self) -> bool {
        self.as_dictionary().map(|_| true).unwrap_or(false)
    }

    pub fn len(&self) -> usize {
        match self {
            Node::Null => None,
            Node::Boolean(_) => None,
            Node::Number(_) => None,
            Node::String(_) => None,
            Node::Array(v) => Some(v.len()),
            Node::Dictionary(d) => Some(d.len()),
        }.expect("it appears that node is not array or dictionary so it has no len!")
    }

    pub fn get_element_at(&self, idx: usize) -> Result<Box<Node>, ErrorCause> {
        if let Some(arr) = self.as_array() {
            if idx <= arr.len() {
                Ok(arr[idx].clone())
            } else {
                Err(IndexOutOfBound)
            }
        } else {
            Err(NodeIsNotArray)
        }
    }

    pub fn get(&self, key: &str) -> Result<Box<Node>, ErrorCause> {
        if let Some(dict) = self.as_dictionary() {
            match dict.get(key).map(|x| x.clone()) {
                None => Err(ItemNotExist),
                Some(x) => Ok(x),
            }
        } else {
            Err(NodeIsNotDictionary)
        }
    }

    pub fn get_string(&self, key: &str) -> Result<String, ErrorCause> {
        match self.get(key) {
            Err(e) => Err(e),
            Ok(node) => match node.as_string() {
                None => Err(ErrorCause::WrongTypeRequested),
                Some(data) => Ok(data),
            },
        }
    }

    pub fn get_number(&self, key: &str) -> Result<f64, ErrorCause> {
        match self.get(key) {
            Err(e) => Err(e),
            Ok(node) => match node.as_number() {
                None => Err(ErrorCause::WrongTypeRequested),
                Some(data) => Ok(data),
            },
        }
    }

    pub fn get_bool(&self, key: &str) -> Result<bool, ErrorCause> {
        match self.get(key) {
            Err(e) => Err(e),
            Ok(node) => match node.as_bool() {
                None => Err(ErrorCause::WrongTypeRequested),
                Some(data) => Ok(data),
            },
        }
    }
}

impl Index<&str> for Node
{
    type Output = Box<Node>;
    fn index(&self, key: &str) -> &Self::Output {
        self.as_dictionary()
            .map(|v| &v[key])
            .expect("trying to use non-dictionary node as dictionary")
    }
}

impl Index<usize> for Node
{
    type Output = Box<Node>;
    fn index(&self, key: usize) -> &Self::Output {
        self.as_array()
            .map(|v| &v[key])
            .expect("trying to use non-array node as dictionary")
    }
}