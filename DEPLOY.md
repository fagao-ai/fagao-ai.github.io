# 🚀 部署指南

本文档说明如何部署和维护 Fagao AI 的 GitHub Pages 网站。

## 📋 前置要求

- GitHub 账户
- fagao-ai 组织的访问权限
- Git 基础知识

## 🔧 本地开发

### 1. 克隆仓库

```bash
git clone https://github.com/fagao-ai/fagao-ai.github.io.git
cd fagao-ai.github.io
```

### 2. 本地预览

你可以使用任何 HTTP 服务器来预览网站：

#### 使用 Python（推荐）

```bash
# Python 3
python3 -m http.server 8000

# Python 2
python -m SimpleHTTPServer 8000
```

#### 使用 Node.js

```bash
# 安装 http-server
npm install -g http-server

# 启动服务器
http-server -p 8000
```

#### 使用 PHP

```bash
php -S localhost:8000
```

然后在浏览器中访问 `http://localhost:8000`

### 3. 修改网站

- **主页内容**: 编辑 `index.html`
- **样式**: 编辑 `styles.css`
- **GPG 密钥**: 编辑 `apt/gpg.key`

## 📦 更新 GPG 密钥

### 1. 生成新的 GPG 密钥（如果需要）

```bash
# 生成密钥对
gpg --full-generate-key

# 选择密钥类型、密钥长度和有效期
# 输入你的姓名和邮箱
# 设置密码短语

# 导出公钥
gpg --armor --export your-email@example.com > apt/gpg.key

# 查看密钥指纹
gpg --fingerprint your-email@example.com
```

### 2. 更新网站上的密钥信息

在 `index.html` 中找到 GPG 部分，更新以下内容：

1. **Key ID**: 将 `待添加` 替换为实际的密钥 ID
2. **密钥指纹**: 将 `密钥指纹将在添加后显示在这里` 替换为实际指纹

### 3. 测试密钥

```bash
# 下载并导入密钥
curl -s https://fagao-ai.github.io/apt/gpg.key | gpg --import -

# 验证密钥
gpg --list-keys
```

## 🌐 部署到 GitHub Pages

### 方式一：直接推送到 main 分支

```bash
# 添加所有更改
git add .

# 提交更改
git commit -m "Update website content"

# 推送到 GitHub
git push origin main
```

GitHub Pages 会自动部署，通常在几分钟后生效。

### 方式二：使用 GitHub Actions（可选）

如果你想使用 CI/CD 流程，可以创建 `.github/workflows/deploy.yml`：

```yaml
name: Deploy to GitHub Pages

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Deploy to GitHub Pages
        uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: .
```

## 🔍 验证部署

### 1. 检查部署状态

访问 GitHub 仓库的 Settings > Pages，查看部署状态。

### 2. 访问网站

打开浏览器访问：https://fagao-ai.github.io

### 3. 测试 GPG 密钥

```bash
# 测试密钥是否可访问
curl -I https://fagao-ai.github.io/apt/gpg.key

# 下载并验证密钥
wget https://fagao-ai.github.io/apt/gpg.key
gpg --import gpg.key
```

## 📊 监控和维护

### 1. 查看 GitHub Pages 构建日志

访问仓库的 Actions 标签页查看构建历史和日志。

### 2. 自定义域名（可选）

如果你想使用自定义域名：

1. 将 `CNAME.template` 重命名为 `CNAME`
2. 在 `CNAME` 文件中添加你的域名
3. 在域名注册商处配置 DNS 记录

### 3. HTTPS 证书

GitHub Pages 会自动为自定义域名提供 HTTPS 证书。

## 🐛 常见问题

### 问题一：页面没有更新

**解决方案**：
- 清除浏览器缓存（Ctrl+Shift+Delete 或 Cmd+Shift+Delete）
- 等待 5-10 分钟让 GitHub Pages 完成部署
- 检查 GitHub Actions 构建日志

### 问题二：GPG 密钥无法访问

**解决方案**：
- 确认 `apt/gpg.key` 文件存在于仓库中
- 检查文件权限
- 使用 `curl -I` 测试链接是否可访问

### 问题三：样式显示异常

**解决方案**：
- 确认 `styles.css` 文件存在
- 检查浏览器控制台是否有错误
- 清除浏览器缓存

## 📝 维护清单

- [ ] 定期更新 GPG 密钥
- [ ] 检查网站链接是否正常
- [ ] 更新项目信息
- [ ] 审查和优化性能
- [ ] 备份重要数据

## 🔗 相关链接

- [GitHub Pages 文档](https://docs.github.com/en/pages)
- [GPG 文档](https://www.gnupg.org/documentation/)
- [Jekyll 文档](https://jekyllrb.com/docs/)

## 📮 获取帮助

如有问题，请：
1. 查看 [GitHub Issues](https://github.com/fagao-ai/issues)
2. 联系维护者
3. 查阅相关文档

---

祝部署顺利！🎉
