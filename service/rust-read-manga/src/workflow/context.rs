use anyhow::{anyhow, Result};
use serde::{Deserialize, de::DeserializeOwned};
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Context {
    data: HashMap<String, Value>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn set<T: Serialize>(&mut self, key: &str, value: T) -> Result<()> {
        self.data
            .insert(key.to_string(), serde_json::to_value(value)?);
        Ok(())
    }

    pub fn get<T: DeserializeOwned>(&self, path: &str) -> Option<T> {
        let mut current = &self.data;
        let mut parts = path.split('.');
        let key = parts.next_back()?;
        let mut value: Option<&Value> = None;

        for part in parts {
            if let Some(v) = current.get(part) {
                if v.is_object() {
                    current = v.as_object().unwrap();
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
