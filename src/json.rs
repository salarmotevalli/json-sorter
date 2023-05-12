pub struct Json {
    entry: String,
}

impl Json {
    pub fn new(entry: String) -> Json {
        Json { entry: entry }
    }

    pub fn encode(&self) {
        todo!();
    }

    pub fn decode(&self) {
        todo!();
    }

    pub fn display(&self) {}

    pub fn display_indent(&self) {}
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
