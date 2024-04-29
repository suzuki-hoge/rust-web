use std::collections::HashMap;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};

use crate::cache::Cache;

pub struct Memory {
    map: Mutex<HashMap<String, String>>,
}

impl Cache for Memory {
    fn get<T: for<'a> Deserialize<'a>>(&self, key: &str) -> Option<T> {
        let map = self.map.lock().unwrap();
        map.get(key).map(|s| serde_json::from_str(s).unwrap())
    }

    fn set<S: Serialize>(&self, key: &str, s: &S) {
        let mut map = self.map.lock().unwrap();
        map.insert(key.to_string(), serde_json::to_string(s).unwrap());
    }

    fn clear(&self) {
        let mut map = self.map.lock().unwrap();
        map.clear();
    }
}

impl Memory {
    pub fn new() -> Self {
        Self { map: Mutex::new(HashMap::new()) }
    }
}
