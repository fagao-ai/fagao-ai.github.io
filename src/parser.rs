use anyhow::Result;
use std::collections::HashMap;

/// 配置解析器 trait，所有配置格式都需要实现此 trait
pub trait ConfigParser: Send + Sync {
    /// 解析配置字符串，返回通用的键值对映射
    /// 键会使用点号(.)表示嵌套结构，例如 "database.host"
    fn parse(&self, content: &str) -> Result<HashMap<String, String>>;

    /// 返回解析器的名称
    fn name(&self) -> &str;
}

/// ENV 生成器配置
#[derive(Debug, Clone)]
pub struct EnvGeneratorConfig {
    /// 前缀，所有环境变量都会加上此前缀
    pub prefix: String,
    /// 分隔符，用于替换配置键中的点号(.)
    pub separator: String,
    /// 是否将键转换为大写
    pub uppercase: bool,
}

impl Default for EnvGeneratorConfig {
    fn default() -> Self {
        Self {
            prefix: String::new(),
            separator: "_".to_string(),
            uppercase: true,
        }
    }
}

/// ENV 生成器
pub struct EnvGenerator {
    config: EnvGeneratorConfig,
}

impl EnvGenerator {
    pub fn new(config: EnvGeneratorConfig) -> Self {
        Self { config }
    }

    pub fn with_prefix(prefix: String) -> Self {
        Self::new(EnvGeneratorConfig {
            prefix,
            ..Default::default()
        })
    }

    pub fn with_separator(separator: String) -> Self {
        Self::new(EnvGeneratorConfig {
            separator,
            ..Default::default()
        })
    }

    /// 将配置键值对转换为 ENV 语句
    pub fn generate(&self, config_map: &HashMap<String, String>) -> Vec<String> {
        let mut env_lines = Vec::new();

        let mut keys: Vec<_> = config_map.keys().collect();
        keys.sort();

        for key in keys {
            if let Some(value) = config_map.get(key) {
                let env_key = self.format_key(key);
                let env_line = format!("{}={}", env_key, value);
                env_lines.push(env_line);
            }
        }

        env_lines
    }

    /// 格式化配置键为环境变量键
    fn format_key(&self, key: &str) -> String {
        // 替换点号为分隔符
        let formatted = key.replace('.', &self.config.separator);

        // 添加前缀（不添加分隔符）
        let with_prefix = if self.config.prefix.is_empty() {
            formatted
        } else {
            format!("{}{}", self.config.prefix, formatted)
        };

        // 转换为大写（如果需要）
        if self.config.uppercase {
            with_prefix.to_uppercase()
        } else {
            with_prefix
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_key_default() {
        let generator = EnvGenerator::new(EnvGeneratorConfig::default());
        assert_eq!(generator.format_key("database.host"), "DATABASE_HOST");
    }

    #[test]
    fn test_format_key_with_prefix() {
        let generator = EnvGenerator::with_prefix("APP".to_string());
        assert_eq!(generator.format_key("database.host"), "APPDATABASE_HOST");
    }

    #[test]
    fn test_format_key_custom_separator() {
        let generator = EnvGenerator::with_separator("__".to_string());
        assert_eq!(generator.format_key("database.host"), "DATABASE__HOST");
    }

    #[test]
    fn test_generate_env() {
        let generator = EnvGenerator::with_prefix("APP".to_string());
        let mut config = HashMap::new();
        config.insert("database.host".to_string(), "localhost".to_string());
        config.insert("database.port".to_string(), "5432".to_string());

        let env_lines = generator.generate(&config);
        assert_eq!(env_lines, vec!["APPDATABASE_HOST=localhost", "APPDATABASE_PORT=5432"]);
    }
}
