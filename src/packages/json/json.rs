use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_cw_value::{to_value, Value};
use serde_json_wasm::{from_str, to_string};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JSON(Value);

impl JSON {
    pub fn from(value: Value) -> Self {
        let v = Self(value);
        // Parse and store again to properly deserialize
        Self::parse_json(v.to_json_string().unwrap().as_str()).unwrap()
    }

    pub fn parse_json(json_str: &str) -> Result<Self, serde_json_wasm::de::Error> {
        let parsed_json: Value = from_str(json_str)?;
        Ok(Self(parsed_json))
    }

    pub fn to_json_string(&self) -> Result<String, serde_json_wasm::ser::Error> {
        let json_str = to_string(&self.0)?;
        Ok(json_str)
    }

    pub fn get<'a>(&'a self, key: &'a str) -> Option<&'a Value> {
        self.get_nested(&self.0, key.split('.'))
    }

    fn get_nested<'a, 'b, I>(&'a self, json: &'b Value, mut keys: I) -> Option<&'a Value>
    where
        'b: 'a,
        I: Iterator<Item = &'a str>,
    {
        match json {
            Value::Map(map) => {
                if let Some(key) = keys.next() {
                    map.get(&to_value(key).unwrap())
                        .and_then(|next_json| self.get_nested(next_json, keys))
                } else {
                    Some(json)
                }
            }
            Value::Seq(list) => {
                if let Some(index) = keys.next() {
                    if let Ok(idx) = index.parse::<usize>() {
                        list.get(idx)
                            .and_then(|next_json| self.get_nested(next_json, keys))
                    } else {
                        None
                    }
                } else {
                    Some(json)
                }
            }
            _ => match keys.next() {
                Some(_) => None,
                None => Some(json),
            },
        }
    }

    pub fn update(&mut self, key: &str, value: Value) -> Result<&Self, serde_json_wasm::de::Error> {
        let keys: Vec<&str> = key.split('.').collect();
        // Perform the update
        self.0 = self.update_nested(&self.0, &keys, value)?;
        Ok(self)
    }

    fn update_nested(
        &self,
        json: &Value,
        keys: &[&str],
        value: Value,
    ) -> Result<Value, serde_json_wasm::de::Error> {
        let result: Value = if let Some(current_key) = keys.first() {
            match json.clone() {
                Value::Map(mut map) => {
                    if keys.len() == 1 {
                        map.insert(Value::String(current_key.to_string()), value);
                    } else {
                        let next_json = map
                            .entry(Value::String(current_key.to_string()))
                            .or_insert(Value::Map(BTreeMap::new()));

                        let new_value = self.update_nested(next_json, &keys[1..], value)?;
                        map.insert(Value::String(current_key.to_string()), new_value);
                    }
                    Value::Map(map)
                }
                Value::Seq(mut list) => {
                    if let Ok(index) = current_key.parse::<usize>() {
                        if keys.len() == 1 {
                            if index < list.len() {
                                list[index] = value;
                            } else {
                                return Err(serde_json_wasm::de::Error::Custom(format!(
                                    "Array index out of bounds: {}",
                                    index
                                )));
                            }
                        } else if index < list.len() {
                            list[index] = self.update_nested(&list[index], &keys[1..], value)?;
                        } else {
                            return Err(serde_json_wasm::de::Error::Custom(format!(
                                "Array index out of bounds: {}",
                                index
                            )));
                        }
                        Value::Seq(list)
                    } else {
                        return Err(serde_json_wasm::de::Error::Custom(format!(
                            "Invalid array index: {}",
                            current_key
                        )));
                    }
                }
                _ => {
                    // Handle other cases here if needed
                    return Err(serde_json_wasm::de::Error::Custom(format!(
                        "Invalid JSON structure at key: {}",
                        current_key
                    )));
                }
            }
        } else {
            json.clone()
        };
        Ok(result)
    }
}
