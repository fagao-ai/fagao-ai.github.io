mod parser;
mod toml_parser;
mod yaml_parser;
mod json_parser;

use anyhow::{bail, Context, Result};
use clap::Parser;
use parser::{ConfigParser, EnvGenerator, EnvGeneratorConfig};
use std::fs;
use std::path::PathBuf;
use yaml_parser::YamlParser;
use toml_parser::TomlParser;
use json_parser::JsonParser;

/// 配置转 ENV 工具
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 配置文件路径
    #[arg(short, long)]
    input: PathBuf,

    /// 输出文件路径（可选，默认输出到 stdout）
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// 配置文件类型 (yaml, toml, json)
    #[arg(short, long)]
    format: Option<String>,

    /// 环境变量前缀
    #[arg(short, long, default_value = "")]
    prefix: String,

    /// 分隔符（用于替换嵌套键中的点号）
    #[arg(short = 's', long, default_value = "_")]
    separator: String,

    /// 保持键的小写（默认转为大写）
    #[arg(long)]
    no_uppercase: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // 读取配置文件
    let content = fs::read_to_string(&args.input)
        .with_context(|| format!("Failed to read file: {}", args.input.display()))?;

    // 确定配置格式
    let format = detect_format(&args.input, &args.format)?;

    // 选择解析器
    let parser: Box<dyn ConfigParser> = match format.as_str() {
        "yaml" | "yml" => Box::new(YamlParser::new()),
        "toml" => Box::new(TomlParser::new()),
        "json" => Box::new(JsonParser::new()),
        _ => bail!("Unsupported format: {}", format),
    };

    // 解析配置
    let config_map = parser.parse(&content)
        .with_context(|| format!("Failed to parse {} content", parser.name()))?;

    // 创建 ENV 生成器
    let generator_config = EnvGeneratorConfig {
        prefix: args.prefix,
        separator: args.separator,
        uppercase: !args.no_uppercase,
    };
    let generator = EnvGenerator::new(generator_config);

    // 生成 ENV 语句
    let env_lines = generator.generate(&config_map);

    // 输出结果
    let output_content = env_lines.join("\n");

    if let Some(output_path) = args.output {
        fs::write(&output_path, output_content)
            .with_context(|| format!("Failed to write to file: {}", output_path.display()))?;
        eprintln!("Successfully written to: {}", output_path.display());
    } else {
        println!("{}", output_content);
    }

    Ok(())
}

/// 根据文件扩展名或指定的格式检测配置类型
fn detect_format(input: &PathBuf, format: &Option<String>) -> Result<String> {
    if let Some(f) = format {
        return Ok(f.clone());
    }

    let extension = input
        .extension()
        .and_then(|ext| ext.to_str())
        .ok_or_else(|| anyhow::anyhow!("Cannot determine file format from extension"))?;

    match extension.to_lowercase().as_str() {
        "yaml" | "yml" => Ok("yaml".to_string()),
        "toml" => Ok("toml".to_string()),
        "json" => Ok("json".to_string()),
        _ => bail!("Unsupported file extension: {}", extension),
    }
}
