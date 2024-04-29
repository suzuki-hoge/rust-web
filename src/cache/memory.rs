use std::cell::RefCell;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::cache::Cache;

pub struct Memory {
    map: RefCell<HashMap<String, String>>,
}

impl Cache for Memory {
    fn get<T: for<'a> Deserialize<'a>>(&self, key: &str) -> Option<T> {
        self.map.borrow().get(key).map(|s| serde_json::from_str(s).unwrap())
    }

    fn set<S: Serialize>(&self, key: &str, s: &S) {
        self.map.borrow_mut().insert(key.to_string(), serde_json::to_string(s).unwrap());
    }

    fn clear(&self) {
        self.map.borrow_mut().clear();
    }
}

impl Memory {
    pub fn new() -> Self {
        Self { map: RefCell::new(HashMap::new()) }
    }
}
