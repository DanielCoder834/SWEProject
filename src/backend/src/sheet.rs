pub struct Sheet {
    owner: String,
    name: String
    private: bool, // Privacy of Sheet
    values: HashMap<u64, String> // Change this to an Enum of CellFilled, CellEmpty, CellInt, etc.   
}

impl Sheet {
    pub fn default() -> Self {
        owner: "".to_string(),
        name: "".to_string(),
        private: true,
        values: HashMap::new()
    }

    pub fn new(owner: String, name: String) -> Self {
        Result {
            owner,
            name,
            private: true,
            values: HashMap::new(),
        }
    }

    pub fn owner(&self) -> &String {
        &self.owner;
    }

    pub fn name(&self) -> &String {
        &self.name;
    }
    
}