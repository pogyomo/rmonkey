use std::collections::HashMap;
use crate::object::Object;

#[derive(Debug, Clone)]
pub struct Env {
    map: HashMap<String, Object>,
}

impl Env {
    pub fn new() -> Env {
        Env { map: HashMap::new() }
    }

    pub fn get(&self, name: &String) -> Option<&Object> {
        self.map.get(name)
    }

    pub fn set(&mut self, name: String, obj: Object) {
        self.map.insert(name, obj);
    }
}
