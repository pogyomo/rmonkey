use std::{cell::RefCell, collections::HashMap, rc::Rc};
use super::object::Object;

#[derive(Debug, Clone)]
pub struct Env {
    pub map: HashMap<String, Object>,
    pub outer: Option<Rc<RefCell<Env>>>,
}

impl Env {
    pub fn new() -> Env {
        Env { map: HashMap::new(), outer: None }
    }

    pub fn new_with_outer(env: Rc<RefCell<Env>>) -> Env {
        Env { map: HashMap::new(), outer: Some(env) }
    }

    pub fn get(&self, name: &String) -> Option<Object> {
        match self.map.get(name) {
            Some(obj) => Some(obj.clone()),
            None => match self.outer {
                Some(ref env) => env.borrow().get(name),
                None => None,
            }
        }
    }

    pub fn set(&mut self, name: String, obj: Object) {
        self.map.insert(name, obj);
    }
}
