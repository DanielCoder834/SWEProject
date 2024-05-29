// Third Party Libraries
use std::collections::HashMap;
use std::error::Error;

// Our Files
use crate::publisher;
use crate::results;

// Type Aliasing
type Result = results::Result;
type Publisher = publisher::Publisher;

#[derive(serde::Deserialize)]
pub struct DataStructure {
    pub storage: HashMap<Publisher, Result>,
    // User names to password
    pub credentialStorage: HashMap<String, String>,
}

impl DataStructure {
    pub fn default() -> Self {
        DataStructure {
            storage: HashMap::new(),
            credentialStorage: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: Publisher, value: &Result) -> Option<Result> {
        self.storage.insert(key, value.clone())
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

    pub fn addCredentials(&mut self, username: &str, password: &str) -> std::result::Result<(), Box<dyn Error>> {
        if self.credentialStorage.contains_key(username) {
            return Err(Box::from("Username already exists"));
        } else {
            self.credentialStorage.insert(username.to_string(), password.to_string());
        }
        Ok(())
    }

    pub fn getCredentials(&self, username: &str) -> Option<&String> {
        self.credentialStorage.get(username)
    }
}
