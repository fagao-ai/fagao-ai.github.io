use crate::parser::ConfigParser;
use anyhow::{Context, Result};
use std::collections::HashMap;
use toml::Value;

/// TOML 配置解析器
pub struct TomlParser;

impl TomlParser {
    pub fn new() -> Self {
        Self
    }
}

impl Default for TomlParser {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigParser for TomlParser {
    fn parse(&self, content: &str) -> Result<HashMap<String, String>> {
        let toml_value: Value =
            toml::from_str(content).context("Failed to parse TOML content")?;

        let mut result = HashMap::new();
        self.flatten_value("", &toml_value, &mut result);

        Ok(result)
    }

    fn name(&self) -> &str {
        "toml"
    }
}

impl TomlParser {
    /// 递归扁平化 TOML 值
    fn flatten_value(&self, prefix: &str, value: &Value, result: &mut HashMap<String, String>) {
        match value {
            Value::String(s) => {
                result.insert(prefix.to_string(), s.clone());
            }
            Value::Integer(n) => {
                result.insert(prefix.to_string(), n.to_string());
            }
            Value::Float(n) => {
                result.insert(prefix.to_string(), n.to_string());
            }
            Value::Boolean(b) => {
                result.insert(prefix.to_string(), b.to_string());
            }
            Value::Datetime(dt) => {
                result.insert(prefix.to_string(), dt.to_string());
            }
            Value::Table(table) => {
                for (key, value) in table {
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
    fn test_parse_simple_toml() {
        let parser = TomlParser::new();
        let toml = r#"
name = "test"
port = 8080
debug = true
"#;
        let result = parser.parse(toml).unwrap();

        assert_eq!(result.get("name").unwrap(), "test");
        assert_eq!(result.get("port").unwrap(), "8080");
        assert_eq!(result.get("debug").unwrap(), "true");
    }

    #[test]
    fn test_parse_nested_toml() {
        let parser = TomlParser::new();
        let toml = r#"
[database]
host = "localhost"
port = 5432

[database.credentials]
username = "admin"
password = "secret"
"#;
        let result = parser.parse(toml).unwrap();

        assert_eq!(result.get("database.host").unwrap(), "localhost");
        assert_eq!(result.get("database.port").unwrap(), "5432");
        assert_eq!(
            result.get("database.credentials.username").unwrap(),
            "admin"
        );
        assert_eq!(
            result.get("database.credentials.password").unwrap(),
            "secret"
        );
    }

    #[test]
    fn test_parse_toml_with_array() {
        let parser = TomlParser::new();
        let toml = r#"
servers = [
    { host = "server1.example.com", port = 8001 },
    { host = "server2.example.com", port = 8002 }
]
"#;
        let result = parser.parse(toml).unwrap();

        assert_eq!(result.get("servers.0.host").unwrap(), "server1.example.com");
        assert_eq!(result.get("servers.0.port").unwrap(), "8001");
        assert_eq!(result.get("servers.1.host").unwrap(), "server2.example.com");
        assert_eq!(result.get("servers.1.port").unwrap(), "8002");
    }
}
