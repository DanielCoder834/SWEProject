use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Result {
    pub success: bool,
    message: String,
    value: Vec<Argument>,
}

impl Result {
    pub fn default() -> Self {
        Result {
            success: true,
            message: "".to_string(),
            value: vec![],
        }
    }
    pub fn new(success: bool, message: String, value: Vec<Argument>) -> Self {
        Result {
            success,
            message,
            value,
        }
    }

    pub fn error(message: String, value: Vec<Argument>) -> Self {
        Result {
            success: false,
            message,
            value,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Argument {
    pub publisher: String,
    pub sheet: String,
    pub id: String,
    pub payload: String,
}

impl Argument {
    pub fn default() -> Self {
        Self {
            publisher: "".to_string(),
            sheet: "".to_string(),
            id: "".to_string(),
            payload: "".to_string(),
        }
    }
    pub fn new(publisher: String, sheet: String, id: String, payload: String) -> Self {
        Self {
            publisher,
            sheet,
            id,
            payload,
        }
    }
}