use serde::{Deserialize, Serialize};

// Written by Daniel Kaplan
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Result {
    pub success: bool,
    message: String,
    value: Vec<Argument>,
}

impl Result {
    // Written by Daniel Kaplan
    pub fn new(success: bool, message: String, value: Vec<Argument>) -> Self {
        Result {
            success,
            message,
            value,
        }
    }

    // Written by Daniel Kaplan
    pub fn error(message: String, value: Vec<Argument>) -> Self {
        Result {
            success: false,
            message,
            value,
        }
    }
}

// Written by Daniel Kaplan
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Argument {
    pub publisher: String,
    pub sheet: String,
    pub id: String,
    pub payload: String,
}

impl Argument {
    // Written by Daniel Kaplan
    pub fn new(publisher: String, sheet: String, id: String, payload: String) -> Self {
        Self {
            publisher,
            sheet,
            id,
            payload,
        }
    }
}
