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
    Array(Vec<Node>),
    Dictionary(HashMap<String, Node>)
}

#[macro_export]
macro_rules! parse_many {
    ($node:ident => $method0:ident($lit0:literal), $method1:ident($lit1:literal)) => {
        {
            let v0 = $node.$method0($lit0);
            let v1 = $node.$method1($lit1);
            match v0 {
                Err(err_data) => Err(err_data),
                Ok(ok_data0) => {
                    match v1 {
                        Err(err_data) => Err(err_data),
                        Ok(ok_data1) => Ok((ok_data0, ok_data1))
                    }
                }
            }
        }
    };
    ($node:ident =>
        $method0:ident($lit0:literal),
        $method1:ident($lit1:literal),
        $method2:ident($lit2:literal)) => {
        {
            let v0 = $node.$method0($lit0);
            let v1 = parse_many!($node => $method1($lit1), $method2($lit2));
            match v0 {
                Err(err_data) => Err(err_data),
                Ok(ok_data0) => {
                    match v1 {
                        Err(err_data) => Err(err_data),
                        Ok(ok_data1) => Ok((ok_data0, ok_data1.0, ok_data1.1))
                    }
                }
            }
        }
    };
    ($node:ident =>
        $method0:ident($lit0:literal),
        $method1:ident($lit1:literal),
        $method2:ident($lit2:literal),
        $method3:ident($lit3:literal)) => {
        {
            let v0 = $node.$method0($lit0);
            let v1 = parse_many!($node =>
                $method1($lit1),
                $method2($lit2),
                $method3($lit3));
            match v0 {
                Err(err_data) => Err(err_data),
                Ok(ok_data0) => {
                    match v1 {
                        Err(err_data) => Err(err_data),
                        Ok(ok_data1) => Ok((
                            ok_data0,
                            ok_data1.0,
                            ok_data1.1,
                            ok_data1.2))
                    }
                }
            }
        }
    };
    ($node:ident =>
        $method0:ident($lit0:literal),
        $method1:ident($lit1:literal),
        $method2:ident($lit2:literal),
        $method3:ident($lit3:literal),
        $method4:ident($lit4:literal)) => {
        {
            let v0 = $node.$method0($lit0);
            let v1 = parse_many!($node =>
                $method1($lit1),
                $method2($lit2),
                $method3($lit3),
                $method4($lit4));
            match v0 {
                Err(err_data) => Err(err_data),
                Ok(ok_data0) => {
                    match v1 {
                        Err(err_data) => Err(err_data),
                        Ok(ok_data1) => Ok((
                            ok_data0,
                            ok_data1.0,
                            ok_data1.1,
                            ok_data1.2,
                            ok_data1.3))
                    }
                }
            }
        }
    };
    ($node:ident =>
        $method0:ident($lit0:literal),
        $method1:ident($lit1:literal),
        $method2:ident($lit2:literal),
        $method3:ident($lit3:literal),
        $method4:ident($lit4:literal),
        $method5:ident($lit5:literal)) => {
        {
            let v0 = $node.$method0($lit0);
            let v1 = parse_many!($node =>
                $method1($lit1),
                $method2($lit2),
                $method3($lit3),
                $method4($lit4),
                $method5($lit5));
            match v0 {
                Err(err_data) => Err(err_data),
                Ok(ok_data0) => {
                    match v1 {
                        Err(err_data) => Err(err_data),
                        Ok(ok_data1) => Ok((
                            ok_data0,
                            ok_data1.0,
                            ok_data1.1,
                            ok_data1.2,
                            ok_data1.3,
                            ok_data1.4))
                    }
                }
            }
        }
    };
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
        match self {
            Node::String(s) => Some(s.clone()),
            _ => None
        }
    }

    pub fn to_string(&self) -> Option<String> {
        match self {
            // we are sometimes finding a need to read something which is not string as a string
            // so it might be a good idea to give user an appropriate method
            Node::Null => Some("null".to_string()),
            Node::Boolean(b) => Some(b.to_string()),
            Node::Number(n) => Some(n.to_string()),
            _ => None,
            // ^^^ yeah, it strange that we ignoring string too, but it makes sense,
            // as well as we already have a `as_string()` method. Doing the same thing
            // here would lead to some strange bugs anyway. For details, take a look at
            // example `simple.rs`
        }
    }

    pub fn is_string(&self) -> bool {
        if let Node::String(_) = self {
            true
        } else {
            false
        }
    }

    pub fn as_array(&self) -> Option<&Vec<Node>> {
        if let Node::Array(v) = self {
            Some(&v)
        } else {
            None
        }
    }

    pub fn is_array(&self) -> bool {
        self.as_array().map(|_| true).unwrap_or(false)
    }

    pub fn as_dictionary(&self) -> Option<&HashMap<String, Node>> {
        if let Node::Dictionary(d) = self {
            Some(d)
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

    pub fn get_element_at(&self, idx: usize) -> Result<Node, ErrorCause> {
        if let Some(arr) = self.as_array() {
            if idx < arr.len() {
                Ok(arr[idx].clone())
            } else {
                Err(IndexOutOfBound(idx))
            }
        } else {
            Err(NodeIsNotArray)
        }
    }

    pub fn get(&self, key: &str) -> Result<&Node, ErrorCause> {
        match self {
            Node::Dictionary(d) => {
                match d.get(key) {
                    None => Err(FieldNotExist(key.to_string())),
                    Some(x) => Ok(x)
                }
            },
            _ => Err(NodeIsNotADictionary)
        }
    }

    pub fn get_string(&self, key: &str) -> Result<&str, ErrorCause> {
        match self.get(key) {
            Err(e) => Err(e),
            Ok(node) => match node {
                Node::String(s) => Ok(s),
                _ => Err(ErrorCause::WrongTypeRequested(key.to_string(), "string"))
            },
        }
    }

    pub fn get_as_string(&self, key: &str) -> Result<String, ErrorCause> {
        match self.get(key) {
            Err(e) => Err(e),
            Ok(node) => match node.to_string() {
                Some(data) => Ok(data),
                None => Err(ErrorCause::WrongTypeRequested(key.to_string(), "as string")),
            },
        }
    }

    pub fn get_number(&self, key: &str) -> Result<f64, ErrorCause> {
        match self.get(key) {
            Err(e) => Err(e),
            Ok(node) => match node.as_number() {
                None => Err(ErrorCause::WrongTypeRequested(key.to_string(), "number")),
                Some(data) => Ok(data),
            },
        }
    }

    pub fn get_bool(&self, key: &str) -> Result<bool, ErrorCause> {
        match self.get(key) {
            Err(e) => Err(e),
            Ok(node) => match node.as_bool() {
                None => Err(ErrorCause::WrongTypeRequested(key.to_string(), "bool")),
                Some(data) => Ok(data),
            },
        }
    }
}

impl Index<&str> for Node
{
    type Output = Node;
    fn index(&self, key: &str) -> &Self::Output {
        match self {
            Node::Dictionary(d) => {
                &d[key]
            },
            _ => panic!("fail")
        }
    }
}

impl Index<usize> for Node
{
    type Output = Node;
    fn index(&self, key: usize) -> &Self::Output {
        match self {
            Node::Array(a) => {
                &a[key]
            },
            _ => panic!("fail")
        }
    }
}