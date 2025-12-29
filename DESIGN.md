# 项目设计文档

## 架构概述

本项目采用基于 Trait 的设计模式，确保良好的扩展性和可维护性。

## 核心组件

### 1. ConfigParser Trait

定义在 `src/parser.rs` 中，是所有配置解析器必须实现的核心接口：

```rust
pub trait ConfigParser: Send + Sync {
    fn parse(&self, content: &str) -> Result<HashMap<String, String>>;
    fn name(&self) -> &str;
}
```

**设计优势：**
- 使用 `Box<dyn ConfigParser>` 实现动态分发
- `Send + Sync` 约束确保线程安全
- 统一的接口返回 `HashMap<String, String>`，便于后续处理

### 2. EnvGenerator

负责将解析后的配置转换为 ENV 语句：

```rust
pub struct EnvGenerator {
    config: EnvGeneratorConfig,
}

pub struct EnvGeneratorConfig {
    pub prefix: String,      // 环境变量前缀
    pub separator: String,   // 分隔符，替换嵌套键中的点号
    pub uppercase: bool,     // 是否转为大写
}
```

**功能特性：**
- 支持自定义前缀（前缀直接拼接，不自动添加分隔符）
- 支持自定义分隔符（默认 `_`）
- 自动排序输出结果
- 灵活的大小写转换

### 3. 具体解析器实现

#### YamlParser (`src/yaml_parser.rs`)
- 使用 `serde_yaml` 库
- 处理 YAML 特有的 Tagged 值
- 支持 Mapping、Sequence 等复杂结构

#### TomlParser (`src/toml_parser.rs`)
- 使用 `toml` 库
- 处理 TOML 的 Table 和 Array
- 支持日期时间等 TOML 特有类型

#### JsonParser (`src/json_parser.rs`)
- 使用 `serde_json` 库
- 处理 JSON 的 Object 和 Array
- 作为扩展性示例实现

## 扁平化策略

所有解析器使用统一的扁平化策略：

1. **嵌套对象**：使用点号(`.`)连接，如 `database.host`
2. **数组元素**：使用数字索引，如 `servers.0.host`
3. **最终转换**：在 ENV 生成时将点号替换为自定义分隔符

**示例：**
```yaml
database:
  credentials:
    username: "admin"
```
扁平化后：`database.credentials.username = "admin"`
转换后：`DATABASE_CREDENTIALS_USERNAME=admin` (使用默认分隔符 `_`)

## 错误处理

使用 `anyhow` 库提供上下文丰富的错误信息：

```rust
let config_map = parser.parse(&content)
    .with_context(|| format!("Failed to parse {} content", parser.name()))?;
```

## CLI 设计

使用 `clap` 库实现友好的命令行界面：

- **自动格式检测**：根据文件扩展名自动识别配置格式
- **灵活的选项组合**：支持前缀、分隔符、大小写等选项
- **输出选项**：可输出到 stdout 或文件

## 扩展指南

### 添加新格式支持

1. **创建新模块**：`src/new_format_parser.rs`
2. **实现 ConfigParser trait**：
   ```rust
   impl ConfigParser for NewFormatParser {
       fn parse(&self, content: &str) -> Result<HashMap<String, String>> {
           // 解析逻辑
       }

       fn name(&self) -> &str {
           "new_format"
       }
   }
   ```
3. **在 main.rs 中注册**：
   ```rust
   mod new_format_parser;
   use new_format_parser::NewFormatParser;

   let parser: Box<dyn ConfigParser> = match format.as_str() {
       "new_format" => Box::new(NewFormatParser::new()),
       // ...
   };
   ```
4. **更新格式检测**：在 `detect_format` 函数中添加新扩展名支持

### 添加新功能

#### 示例：添加默认值支持

可以在 `ConfigParser` trait 中添加新方法：

```rust
pub trait ConfigParser: Send + Sync {
    fn parse(&self, content: &str) -> Result<HashMap<String, String>> {
        self.parse_with_defaults(content, &HashMap::new())
    }

    fn parse_with_defaults(
        &self,
        content: &str,
        defaults: &HashMap<String, String>
    ) -> Result<HashMap<String, String>> {
        let mut result = self.parse_content(content)?;
        for (key, value) in defaults {
            result.entry(key.clone()).or_insert_with(|| value.clone());
        }
        Ok(result)
    }

    fn name(&self) -> &str;
}
```

## 测试策略

每个解析器都有完整的单元测试：

- **简单值测试**：字符串、数字、布尔值
- **嵌套结构测试**：多层嵌套的对象
- **数组测试**：验证索引正确性
- **边界情况**：null 值、空数组等

测试使用 `#[cfg(test)]` 模块，确保测试代码不会影响生产构建。

## 性能考虑

1. **最小化克隆**：在可能的情况下使用引用
2. **预分配容量**：对于已知大小的 HashMap
3. **惰性求值**：只在需要时才进行字符串转换

## 未来改进方向

1. **支持更多格式**：INI、Properties、XML 等
2. **模板功能**：支持变量替换
3. **验证规则**：添加配置验证
4. **导出格式**：支持导出为其他格式（如 shell 脚本）
5. **配置合并**：支持多个配置文件的合并
6. **注释保留**：在输出中保留原始注释
