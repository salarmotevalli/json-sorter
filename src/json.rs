use serde_json::{Result as SerdeResult, Value};

pub struct Json {}

impl Json {
    #[allow(dead_code)]
    pub fn decode(json_string: &str) -> SerdeResult<Value> {
        // Decode to Value struct
        let value: Value = serde_json::from_str(json_string)?;
        Ok(value)
    }

    #[allow(dead_code)]
    pub fn encode(data_map: &Value) -> String {
        // Encode Value struct to json string
        data_map.to_string()
    }

    #[allow(dead_code)]
    pub fn encode_with_indent(data_map: &Value) -> String {
        serde_json::to_string_pretty(data_map).unwrap()
    }
}
