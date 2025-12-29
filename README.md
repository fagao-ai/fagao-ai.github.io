# Fagao AI GitHub Pages

欢迎访问 Fagao AI 的官方网站！这是我们的 GitHub Pages 主页，用于展示我们的开源项目和相关资源。

## 🌐 访问网站

- **官网地址**: https://fagao-ai.github.io
- **组织地址**: https://github.com/fagao-ai

## 📦 项目展示

我们在主页上展示了以下内容：

1. **项目介绍** - 我们开发的开源工具和应用
2. **GPG 密钥** - 用于验证软件包的公钥
3. **更新日志** - 项目的最新动态

## 🔐 GPG 密钥使用

我们的 GPG 公钥托管在 `/apt/gpg.key`，可以用于验证软件包的真实性。

### 导入密钥

```bash
# 使用 wget
wget -qO - https://fagao-ai.github.io/apt/gpg.key | sudo apt-key add -

# 使用 curl
curl -s https://fagao-ai.github.io/apt/gpg.key | sudo apt-key add -
```

详细说明请访问 [APT 仓库说明](./apt/README.md)。

## 🚀 当前项目

### CTE - 配置转换工具

一个强大的命令行工具，支持在多种配置文件格式之间进行转换：

- JSON ↔ YAML
- JSON ↔ TOML
- YAML ↔ TOML
- JSON/YAML/TOML → 环境变量

**仓库地址**: https://github.com/fagao-ai/cte

## 🛠️ 技术栈

- **前端**: HTML5, CSS3, JavaScript (ES6+)
- **托管**: GitHub Pages
- **设计**: 响应式设计，支持各种设备

## 📝 贡献指南

欢迎为我们的项目做出贡献！

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 📮 联系我们

- **GitHub**: [@fagao-ai](https://github.com/fagao-ai)
- **Email**: contact@fagao-ai.github.io

## 🙏 致谢

感谢所有为 Fagao AI 项目做出贡献的开发者！

---

⭐ 如果觉得我们的项目有用，请给我们一个 Star！
