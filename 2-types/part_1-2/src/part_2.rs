use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Request {
    pub id: u32,
    pub method: String,
    pub path: String,
    pub headers: Headers,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Headers {
    pub host: String,
    pub user_agent: String,
}

pub fn json_to_toml(input: &str) -> String {
    let req: Request = serde_json::from_str(input).unwrap();
    toml::to_string(&req).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_json_to_toml() {
        let json = r#"
        {
            "id": 42,
            "method": "GET",
            "path": "/home",
            "headers": {
                "host": "example.com",
                "user_agent": "curl"
            }
        }
        "#;

        let toml = json_to_toml(json);
        assert!(toml.contains("id = 42"));
        assert!(toml.contains("method = \"GET\""));
    }
}
