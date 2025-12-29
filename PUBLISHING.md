# 发布指南

本文档介绍如何发布新版本的 cte。

## 前置设置

### 1. 创建 Homebrew Tap 仓库

```bash
# 在 GitHub 上创建一个名为 homebrew-tap 的仓库
# 然后克隆它
git clone https://github.com/fagao-ai/homebrew-tap.git
cd homebrew-tap

# 创建 Formula 目录
mkdir -p Formula

# 复制 cte.rb 到 Formula 目录
# （项目根目录下的 cte.rb 文件）

git add Formula/cte.rb
git commit -m "Add cte formula"
git push origin main
```

### 2. 设置 GitHub Secrets

在你的 cte 仓库中设置以下 Secrets：

**Settings → Secrets and variables → Actions → New repository secret**

- `HOMEBREW_TAP_TOKEN`: 用于推送更新到 homebrew-tap 的 Personal Access Token
  - 需要有 `repo` 权限

### 3. 启用 GitHub Pages

为 APT 仓库启用 GitHub Pages：

**Settings → Pages → Source**
- 选择 `gh-pages` 分支
- 选择 `/ (root)` 目录

## 发布流程

### 自动发布（推荐）

1. **更新版本号**

   编辑 `Cargo.toml`:
   ```toml
   [package]
   name = "cte"
   version = "0.2.0"  # 新版本号
   ```

2. **创建并推送 Tag**

   ```bash
   git add Cargo.toml
   git commit -m "Bump version to 0.2.0"

   git tag -a v0.2.0 -m "Release v0.2.0"
   git push origin main
   git push origin v0.2.0
   ```

3. **等待 GitHub Actions 完成**

   GitHub Actions 会自动：
   - ✅ 构建多平台二进制文件（Linux/macOS, amd64/arm64）
   - ✅ 创建 GitHub Release
   - ✅ 更新 Homebrew Formula
   - ✅ 构建 Debian 包

4. **验证发布**

   - 访问 https://github.com/fagao-ai/cte/releases
   - 下载并测试二进制文件
   - 测试 Homebrew 安装：`brew upgrade cte`

### 手动发布

如果自动发布失败，可以手动执行以下步骤：

#### 1. 构建二进制文件

```bash
# Linux AMD64
cargo build --release --target x86_64-unknown-linux-gnu
cd target/x86_64-unknown-linux-gnu/release
tar czf cte-linux-amd64.tar.gz cte

# macOS AMD64
cargo build --release --target x86_64-apple-darwin
cd target/x86_64-apple-darwin/release
tar czf cte-darwin-amd64.tar.gz cte

# macOS ARM64
cargo build --release --target aarch64-apple-darwin
cd target/aarch64-apple-darwin/release
tar czf cte-darwin-arm64.tar.gz cte
```

#### 2. 创建 GitHub Release

访问 https://github.com/fagao-ai/cte/releases/new:
- 选择 tag
- 上传构建的二进制文件
- 编写 Release Notes

#### 3. 更新 Homebrew Formula

编辑 homebrew-tap 中的 `Formula/cte.rb`:
```ruby
class Cte < Formula
  desc "Config to ENV converter"
  homepage "https://github.com/fagao-ai/cte"
  url "https://github.com/fagao-ai/cte/archive/refs/tags/v0.2.0.tar.gz"
  sha256 "<计算新的 SHA256>"
  # ...
end
```

计算 SHA256:
```bash
curl -L https://github.com/fagao-ai/cte/archive/refs/tags/v0.2.0.tar.gz | shasum -a 256
```

#### 4. 构建 Debian 包

```bash
cargo install cargo-deb
cargo deb
```

上传 `.deb` 文件到 Release 或 APT 仓库。

## 发布检查清单

发布前确保：

- [ ] 更新 `Cargo.toml` 中的版本号
- [ ] 更新 CHANGELOG.md（如果有）
- [ ] 所有测试通过：`cargo test`
- [ ] 在多个平台上测试构建
- [ ] 更新文档中的版本号（如需要）
- [ ] 检查依赖版本是否需要更新

发布后验证：

- [ ] GitHub Release 创建成功
- [ ] 二进制文件可以下载和运行
- [ ] Homebrew 可以更新安装
- [ ] Debian 包可以安装
- [ ] 安装脚本正常工作

## 故障排除

### GitHub Actions 失败

1. 检查日志：Actions → 选择运行的工作流 → 查看具体失败步骤
2. 常见问题：
   - 权限问题：检查 `HOMEBREW_TAP_TOKEN` 是否有正确的权限
   - 构建失败：检查 Rust 版本和依赖兼容性
   - 签名失败：检查 GPG 密钥配置

### Homebrew 更新失败

```bash
# 清理 Homebrew 缓存
brew cleanup

# 强制更新
brew reinstall cte
```

### APT 仓库问题

确保 GitHub Pages 正常启用，并且文件在 `gh-pages` 分支中。

## 版本号规范

遵循 [语义化版本](https://semver.org/lang/zh-CN/)：

- **主版本号 (MAJOR)**：不兼容的 API 变更
- **次版本号 (MINOR)**：向后兼容的功能性新增
- **修订号 (PATCH)**：向后兼容的问题修正

示例：
- `v1.0.0` → `v1.0.1`：Bug 修复
- `v1.0.0` → `v1.1.0`：新功能
- `v1.0.0` → `v2.0.0`：破坏性变更
