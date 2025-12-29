use crate::parser::ConfigParser;
use anyhow::{Context, Result};
use serde_json::Value;
use std::collections::HashMap;

/// JSON 配置解析器
pub struct JsonParser;

impl JsonParser {
    pub fn new() -> Self {
        Self
    }
}

impl Default for JsonParser {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigParser for JsonParser {
    fn parse(&self, content: &str) -> Result<HashMap<String, String>> {
        let json_value: Value =
            serde_json::from_str(content).context("Failed to parse JSON content")?;

        let mut result = HashMap::new();
        self.flatten_value("", &json_value, &mut result);

        Ok(result)
    }

    fn name(&self) -> &str {
        "json"
    }
}

impl JsonParser {
    /// 递归扁平化 JSON 值
    fn flatten_value(&self, prefix: &str, value: &Value, result: &mut HashMap<String, String>) {
        match value {
            Value::String(s) => {
                result.insert(prefix.to_string(), s.clone());
            }
            Value::Number(n) => {
                result.insert(prefix.to_string(), n.to_string());
            }
            Value::Bool(b) => {
                result.insert(prefix.to_string(), b.to_string());
            }
            Value::Null => {
                result.insert(prefix.to_string(), "".to_string());
            }
            Value::Object(obj) => {
                for (key, value) in obj {
                    let new_prefix = if prefix.is_empty() {
                        key.clone()
                    } else {
                        format!("{}.{}", prefix, key)
                    };

                    self.flatten_value(&new_prefix, value, result);
                }
            }
            Value::Array(arr) => {
                for (index, value) in arr.iter().enumerate() {
                    let new_prefix = format!("{}.{}", prefix, index);
                    self.flatten_value(&new_prefix, value, result);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_json() {
        let parser = JsonParser::new();
        let json = r#"
{
  "name": "test",
  "port": 8080,
  "debug": true
}
"#;
        let result = parser.parse(json).unwrap();

        assert_eq!(result.get("name").unwrap(), "test");
        assert_eq!(result.get("port").unwrap(), "8080");
        assert_eq!(result.get("debug").unwrap(), "true");
    }

    #[test]
    fn test_parse_nested_json() {
        let parser = JsonParser::new();
        let json = r#"
{
  "database": {
    "host": "localhost",
    "port": 5432,
    "credentials": {
      "username": "admin",
      "password": "secret"
    }
  }
}
"#;
        let result = parser.parse(json).unwrap();

        assert_eq!(result.get("database.host").unwrap(), "localhost");
        assert_eq!(result.get("database.port").unwrap(), "5432");
        assert_eq!(result.get("database.credentials.username").unwrap(), "admin");
        assert_eq!(result.get("database.credentials.password").unwrap(), "secret");
    }

    #[test]
    fn test_parse_json_with_array() {
        let parser = JsonParser::new();
        let json = r#"
{
  "servers": [
    { "host": "server1.example.com", "port": 8001 },
    { "host": "server2.example.com", "port": 8002 }
  ]
}
"#;
        let result = parser.parse(json).unwrap();

        assert_eq!(result.get("servers.0.host").unwrap(), "server1.example.com");
        assert_eq!(result.get("servers.0.port").unwrap(), "8001");
        assert_eq!(result.get("servers.1.host").unwrap(), "server2.example.com");
        assert_eq!(result.get("servers.1.port").unwrap(), "8002");
    }
}
