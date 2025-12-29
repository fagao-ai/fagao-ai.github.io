# 发布快速检查清单

## 准备阶段

- [ ] 确保所有测试通过：`cargo test`
- [ ] 更新 `Cargo.toml` 中的版本号
- [ ] 更新 CHANGELOG.md（记录变更内容）
- [ ] 本地测试构建：`cargo build --release`

## 发布步骤

1. **提交版本变更**
   ```bash
   git add .
   git commit -m "Bump version to vX.X.X"
   ```

2. **创建标签**
   ```bash
   git tag -a vX.X.X -m "Release vX.X.X"
   git push origin main
   git push origin vX.X.X
   ```

3. **等待 GitHub Actions**
   - 访问 https://github.com/fagao-ai/cte/actions
   - 等待所有工作流完成（约 5-10 分钟）

## 验证阶段

- [ ] 检查 GitHub Release 是否创建成功
- [ ] 下载并测试二进制文件
- [ ] 测试 Homebrew 安装：`brew upgrade cte`
- [ ] 测试安装脚本：`curl -fsSL https://raw.githubusercontent.com/fagao-ai/cte/main/scripts/install.sh | sudo sh`
- [ ] 更新文档（如需要）

## 发布后

- [ ] 在社交媒体/社区发布更新公告
- [ ] 关闭相关的 GitHub Issues
- [ ] 记录发布日志

## 快速命令

```bash
# 完整发布流程（假设版本是 0.2.0）
vim Cargo.toml  # 修改版本号
cargo test      # 运行测试
cargo build --release  # 本地构建

git add .
git commit -m "Bump version to 0.2.0"
git tag -a v0.2.0 -m "Release v0.2.0"
git push origin main
git push origin v0.2.0

# 等待 Actions 完成后验证
gh release view v0.2.0  # 查看 Release
brew upgrade cte  # 测试 Homebrew
```

## 回滚流程（如果需要）

如果发布出现问题：

```bash
# 删除 GitHub Release（通过网页或 gh CLI）
gh release delete v0.X.X -y

# 删除标签
git tag -d v0.X.X
git push origin :refs/tags/v0.X.X

# 创建新版本或修复问题后重新发布
```
