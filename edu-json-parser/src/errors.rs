#[derive(PartialEq, Clone, Debug)]
pub enum ErrorCause {
    FieldNotExist(String),
    WrongTypeRequested(String, &'static str),
    IndexOutOfBound(usize),
    NodeIsNotArray,
    NodeIsNotDictionary
}

impl ToString for ErrorCause {
    fn to_string(&self) -> String {
        match self {
            ErrorCause::FieldNotExist(field_name) => format!(
                "Field with a name '{}' do not exist", field_name
            ),
            ErrorCause::WrongTypeRequested(field_name, type_name) => format!(
                "Trying to look at field '{}' like it was of a type `{}`, but found something other instead",
                field_name, type_name
            ),
            ErrorCause::IndexOutOfBound(idx) => format!(
                "Index {} is out of bounds", idx
            ),
            ErrorCause::NodeIsNotArray =>
                "Trying to work with a node like it was an array, but it didn't".to_string(),
            ErrorCause::NodeIsNotDictionary =>
                "Trying to work with a node like it was a dictionary, but it didn't".to_string(),
        }
    }
}