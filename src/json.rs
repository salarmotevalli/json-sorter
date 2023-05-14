use serde_json::{Result as SerdeResult, Value};

pub struct Json {}

impl Json {
    pub fn decode(json_string: &str) -> SerdeResult<Value> {
        // Decode to Value struct
        let value: Value = serde_json::from_str(json_string)?;
        Ok(value)
    }

    pub fn encode(data_map: &Value) -> String {
        // Encode Value struct to json string
        data_map.to_string()
    }

    pub fn encode_with_indent(data_map: &Value) -> String {
        serde_json::to_string_pretty(data_map).unwrap()
    }
}
