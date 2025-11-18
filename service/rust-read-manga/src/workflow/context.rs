use anyhow::{Result};
use serde::{Deserialize, de::DeserializeOwned};
use serde::Serialize;
use serde_json::{Value, Map}; // <-- 1. Import Map
// std::collections::HashMap is no longer needed for `data`

#[derive(Serialize, Deserialize)]
pub struct Context {
    data: Map<String, Value>, // <-- 2. Change HashMap to Map
}

impl Context {
    pub fn new() -> Self {
        Self {
            data: Map::new(), // <-- 3. Change HashMap::new to Map::new
        }
    }

    pub fn set<T: Serialize>(&mut self, key: &str, value: T) -> Result<()> {
        self.data
            .insert(key.to_string(), serde_json::to_value(value)?);
        Ok(())
    }

    pub fn get<T: DeserializeOwned>(&self, path: &str) -> Option<T> {
        let mut current = &self.data; // <-- This is now type `&Map<String, Value>`
        let mut parts = path.split('.');
        let key = parts.next_back()?;
        let mut value: Option<&Value> = None;

        for part in parts {
            if let Some(v) = current.get(part) {
                if v.is_object() {
                    current = v.as_object().unwrap(); // <-- This line now works
                } else {
                    return None;
                }
            } else {
                return None;
            }
        }
        value = current.get(key);

        value.and_then(|v| serde_json::from_value(v.clone()).ok())
    }
}