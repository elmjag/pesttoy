use std::collections::HashMap;

pub struct SymTable(HashMap<String, i64>);

impl SymTable {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn set_value(&mut self, name: &str, value: i64) {
        self.0.insert(String::from(name), value);
    }

    pub fn get_value(&self, name: &str) -> i64 {
        match self.0.get(name) {
            Some(value) => *value,
            None => panic!("unknown variable '{name}'"),
        }
    }
}
