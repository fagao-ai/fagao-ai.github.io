# APT 仓库和 GPG 密钥说明

本目录用于存储 Fagao AI 的 APT 仓库 GPG 公钥，用于验证软件包的真实性。

## 使用说明

### 1. 导入 GPG 密钥

使用 wget：
```bash
wget -qO - https://fagao-ai.github.io/apt/gpg.key | sudo apt-key add -
```

使用 curl：
```bash
curl -s https://fagao-ai.github.io/apt/gpg.key | sudo apt-key add -
```

### 2. 验证密钥是否已导入

```bash
apt-key list
```

### 3. 添加 APT 仓库（即将推出）

```bash
echo "deb [arch=amd64] https://fagao-ai.github.io/apt stable main" | sudo tee /etc/apt/sources.list.d/fagao-ai.list
```

### 4. 更新包列表

```bash
sudo apt update
```

### 5. 安装软件包

```bash
sudo apt install package-name
```

## 安全说明

- 始终验证下载的软件包签名
- 定期更新 GPG 密钥
- 如果密钥指纹不匹配，请勿安装软件包

## 生成 GPG 密钥

如果您需要为项目生成新的 GPG 密钥：

```bash
# 生成密钥对
gpg --full-generate-key

# 导出公钥
gpg --armor --export your-email@example.com > gpg.key

# 查看密钥指纹
gpg --fingerprint your-email@example.com
```

## 联系方式

如有问题，请通过以下方式联系我们：
- GitHub Issues: https://github.com/fagao-ai/issues
- Email: contact@fagao-ai.github.io
