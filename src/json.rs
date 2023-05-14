use serde_json::json;
use serde_json::{Map, Result as SerdeResult, Value};
use std::{collections::HashMap, fmt::format};

pub struct Json {}

impl Json {
    pub fn decode(json_string: &str) -> SerdeResult<Value> {
        // Decode to Value struct
        let value: Value = serde_json::from_str(json_string)?;
        Ok(value)
    }

    pub fn encode(data_map: &Value) -> String {
        data_map.to_string()
    }

    pub fn encode_with_indent(data_map: &Value) -> String {
        serde_json::to_string_pretty(data_map).unwrap()
    }
}

impl std::fmt::Display for Json {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            fmt,
            "My name is {} and I'm {} years old.",
            "Salar", "Motevalli"
        )
    }
}
