use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Request {
    pub id: u32,
    pub method: String,
    pub params: Vec<String>,
}

pub fn json_to_toml(json_data: &str) -> Result<String, Box<dyn std::error::Error>> {
    let request: Request = serde_json::from_str(json_data)?;
    let toml_data = toml::to_string_pretty(&request)?;
    Ok(toml_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_json_to_toml() {
        let json = r#"{
            "id": 1,
            "method": "getUser",
            "params": ["id=42", "name=John"]
        }"#;

        let toml = json_to_toml(json).unwrap();
        assert!(toml.contains("method = \"getUser\""));
    }
}
