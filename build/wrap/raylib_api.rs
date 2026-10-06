use std::error::Error;

use serde_json::Value;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Definition {
    pub name: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub value: Value,
    pub description: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct EnumVariant {
    pub name: String,
    pub description: String,
    pub value: u32,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Enum {
    pub name: String,
    pub description: String,
    pub values: Vec<EnumVariant>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct RayLibApiDefinition {
    pub defines: Vec<Definition>,
    pub enums: Vec<Enum>,
}

impl RayLibApiDefinition {
    pub fn load(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let contents = std::fs::read_to_string(file_path)?;
        // raylib 6.0's bundled JSON leaves the quotes in this description unescaped.
        let contents = contents.replace(
            r#"some filters available: "*.*", "FILES*", "DIRS*""#,
            r#"some filters available: \"*.*\", \"FILES*\", \"DIRS*\""#,
        );
        let api = serde_json::from_str(&contents)?;
        Ok(api)
    }
}
