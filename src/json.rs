use serde_json::{from_str, Map, Result as SerdeResult, Value};

pub struct Json {
    entry: String,
    dataMap: Option<Value>,
}

impl Json {
    pub fn new(entry: String) -> Json {
        Json {
            entry: entry,
            dataMap: None,
        }
    }

    pub fn decode(&mut self) -> &Json {
        match from_str(&self.entry) {
            Ok(j) => {
                self.dataMap = j;
                self
            }
            Err(_) => {
                self.dataMap = None;
                self
            }
        }
    }

    pub fn get_decoded(&self) -> Value {
        if let Some(d) = &self.dataMap {
            d.clone()
        } else {
            Value::Null
        }
    }

    pub fn encode(&self) -> String {
        todo!();
    }

    pub fn decode_with_indent(&self) -> String {
        todo!();
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
