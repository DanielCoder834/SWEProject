// Third Party Libraries
use std::collections::HashMap;

// Our Files
use crate::publisher;
use crate::results;

// Type Aliasing
type Result = results::Result;
type Publisher = publisher::Publisher;

pub struct DataStructure {
    pub storage: HashMap<Publisher, Result>,
}

impl DataStructure {
    pub fn default() -> Self {
        DataStructure {
            storage: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: Publisher, value: Result) -> Option<Result> {
        self.storage.insert(key, value)
    }
    pub fn delete(&mut self, key: Publisher) -> Option<Result> {
        self.storage.remove(&key)
    }
    pub fn get(&mut self, key: Publisher) -> Option<&Result> {
        self.storage.get(&key)
    }
    pub fn update(&mut self, key: Publisher, new_result: Result) {
        if let Some(result) = self.storage.get_mut(&key) {
            *result = new_result;
        }
    }
}
