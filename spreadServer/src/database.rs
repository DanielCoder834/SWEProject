use crate::results;
use crate::users;
use std::collections::HashMap;
type Result = results::Result;
type User = users::User;

pub struct DataStructure {
    pub storage: HashMap<User, Result>,
}

impl DataStructure {
    pub fn default() -> Self {
        DataStructure {
            storage: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: User, value: Result) -> Option<Result> {
        self.storage.insert(key, value)
    }
    pub fn delete(&mut self, key: User) -> Option<Result> {
        self.storage.remove(&key)
    }
    pub fn get(&mut self, key: User) -> Option<&Result> {
        self.storage.get(&key)
    }
    pub fn update(&mut self, key: User, new_result: Result) {
        if let Some(result) = self.storage.get_mut(&key) {
            *result = new_result;
        }
    }
}
