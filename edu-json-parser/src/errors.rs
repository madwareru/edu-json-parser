#[derive(PartialEq, Copy, Clone, Debug)]
pub enum ErrorCause {
    ItemNotExist,
    WrongTypeRequested,
    IndexOutOfBound,
    NodeIsNotArray,
    NodeIsNotDictionary
}

impl ToString for ErrorCause {
    fn to_string(&self) -> String {
        match self {
            ErrorCause::ItemNotExist => "Item not exist".to_string(),
            ErrorCause::WrongTypeRequested => "Wrong type requested".to_string(),
            ErrorCause::IndexOutOfBound => "Index is out of bounds".to_string(),
            ErrorCause::NodeIsNotArray => "Node is not an array".to_string(),
            ErrorCause::NodeIsNotDictionary => "Node is not a dictionary".to_string(),
        }
    }
}