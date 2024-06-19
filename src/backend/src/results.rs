use serde::{Deserialize, Serialize};

// @author Daniel Kaplan
// Represents the response to server requests from the client
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Result {
    pub success: bool,
    pub message: Option<String>,
    pub value: Option<Vec<Argument>>,
}

impl Result {
    // @author Daniel Kaplan
    // Returns a new Result
    pub fn new(success: bool, message: String, value: Vec<Argument>) -> Self {
        Result {
            success,
            message: string_to_optional(message),
            value: vector_to_optional(value),
        }
    }

    // @author Daniel Kaplan
    // Returns the result with the success equal to false
    pub fn error(message: String, value: Vec<Argument>) -> Self {
        Result {
            success: false,
            message: string_to_optional(message),
            value: vector_to_optional(value),
        }
    }
}

// @author Daniel Kaplan
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Argument {
    pub publisher: String,
    pub sheet: Option<String>,
    pub id: Option<String>,
    pub payload: Option<String>,
}

impl Argument {
    // @author Daniel Kaplan
    // Creates a new argument
    pub fn new(publisher: String, sheet: String, id: String, payload: String) -> Self {
        Self {
            publisher,
            sheet: string_to_optional(sheet),
            id: string_to_optional(id),
            payload: string_to_optional(payload),
        }
    }
}

// @author Daniel Kaplan
// Converts a string to an optional of a string
// If is an empty string, it will return None
pub fn string_to_optional(str: String) -> Option<String> {
    if str.is_empty() {
        None
    } else {
        Some(str)
    }
}

// @author Daniel Kaplan
// Converts an optional of a string to a string
// If it is None, it will return an empty string
pub fn optional_to_string(opt: Option<String>) -> String {
    if let Some(str) = opt {
        str
    } else {
        "".to_string()
    }
}

// @author Daniel Kaplan
// Converts a vector to an optional of a vector
// If is an empty vector, it will return None
pub fn vector_to_optional<T>(vec: Vec<T>) -> Option<Vec<T>> {
    if vec.is_empty() {
        None
    } else {
        Some(vec)
    }
}

// @author Daniel Kaplan
// Converts an optional of a vector to a vector
// If it is None, it will return an empty vector
pub fn optional_to_vector<T>(vec: Option<Vec<T>>) -> Vec<T> {
    if let Some(value) = vec {
        value
    } else {
        vec![]
    }
}