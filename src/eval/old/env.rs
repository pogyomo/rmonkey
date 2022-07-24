use std::{collections::HashMap, cell::RefCell, rc::Rc};
use crate::object::Object;

#[derive(Debug, Clone)]
pub struct Env<'a> {
    map: RefCell<HashMap<String, Object<'a>>>,
    outer: Option<Rc<Env<'a>>>,
}

impl<'a> Env<'a> {
    pub fn new() -> Env<'a> {
        Env { map: RefCell::new(HashMap::new()), outer: None }
    }

    pub fn new_with_outer(env: Rc<Env<'a>>) -> Env<'a> {
        Env { map: RefCell::new(HashMap::new()), outer: Some(env) }
    }

    pub fn get(&self, name: &String) -> Option<Object<'a>> {
        match self.map.borrow().get(name) {
            Some(obj) => Some(obj.clone()),
            None => match self.outer {
                Some(ref env) => env.get(name),
                None => None,
            }
        }
    }

    pub fn set(&self, name: String, obj: Object<'a>) {
        self.map.borrow_mut().insert(name, obj);
    }
}
