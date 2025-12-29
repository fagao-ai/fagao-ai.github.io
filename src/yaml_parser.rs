use crate::parser::ConfigParser;
use anyhow::{Context, Result};
use serde_yaml::Value;
use std::collections::HashMap;

/// YAML 配置解析器
pub struct YamlParser;

impl YamlParser {
    pub fn new() -> Self {
        Self
    }
}

impl Default for YamlParser {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigParser for YamlParser {
    fn parse(&self, content: &str) -> Result<HashMap<String, String>> {
        let yaml_value: Value =
            serde_yaml::from_str(content).context("Failed to parse YAML content")?;

        let mut result = HashMap::new();
        self.flatten_value("", &yaml_value, &mut result);

        Ok(result)
    }

    fn name(&self) -> &str {
        "yaml"
    }
}

impl YamlParser {
    /// 递归扁平化 YAML 值
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
            Value::Mapping(map) => {
                for (key, value) in map {
                    let key_str = match key {
                        Value::String(s) => s.clone(),
                        _ => serde_yaml::to_string(key).unwrap_or_default(),
                    };

                    let new_prefix = if prefix.is_empty() {
                        key_str
                    } else {
                        format!("{}.{}", prefix, key_str)
                    };

                    self.flatten_value(&new_prefix, value, result);
                }
            }
            Value::Sequence(seq) => {
                for (index, value) in seq.iter().enumerate() {
                    let new_prefix = format!("{}.{}", prefix, index);
                    self.flatten_value(&new_prefix, value, result);
                }
            }
            Value::Tagged(tagged) => {
                // 对于 tagged value, 我们递归处理其内部值
                self.flatten_value(prefix, &tagged.value, result);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_yaml() {
        let parser = YamlParser::new();
        let yaml = r#"
name: test
port: 8080
debug: true
"#;
        let result = parser.parse(yaml).unwrap();

        assert_eq!(result.get("name").unwrap(), "test");
        assert_eq!(result.get("port").unwrap(), "8080");
        assert_eq!(result.get("debug").unwrap(), "true");
    }

    #[test]
    fn test_parse_nested_yaml() {
        let parser = YamlParser::new();
        let yaml = r#"
database:
  host: localhost
  port: 5432
  credentials:
    username: admin
    password: secret
"#;
        let result = parser.parse(yaml).unwrap();

        assert_eq!(result.get("database.host").unwrap(), "localhost");
        assert_eq!(result.get("database.port").unwrap(), "5432");
        assert_eq!(result.get("database.credentials.username").unwrap(), "admin");
        assert_eq!(result.get("database.credentials.password").unwrap(), "secret");
    }

    #[test]
    fn test_parse_yaml_with_array() {
        let parser = YamlParser::new();
        let yaml = r#"
servers:
  - host: server1.example.com
    port: 8001
  - host: server2.example.com
    port: 8002
"#;
        let result = parser.parse(yaml).unwrap();

        assert_eq!(result.get("servers.0.host").unwrap(), "server1.example.com");
        assert_eq!(result.get("servers.0.port").unwrap(), "8001");
        assert_eq!(result.get("servers.1.host").unwrap(), "server2.example.com");
        assert_eq!(result.get("servers.1.port").unwrap(), "8002");
    }
}
