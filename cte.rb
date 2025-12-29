# Homebrew Formula for cte
# 保存路径: $(brew --repository)/Library/Taps/fagao-ai/homebrew-tap/Formula/cte.rb

class Cte < Formula
  desc "Config to ENV converter - Convert YAML/TOML/JSON configs to environment variables"
  homepage "https://github.com/fagao-ai/cte"
  url "https://github.com/fagao-ai/cte/archive/refs/tags/v0.1.0.tar.gz"
  sha256 ""  # 在发布时需要替换为实际的 SHA256
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    (testpath/"config.yaml").write <<~EOS
      app:
        name: "test"
        port: 8080
    EOS

    output = shell_output("#{bin}/cte -i config.yaml --prefix TEST_")
    assert_match "TEST_APP_NAME=test", output
    assert_match "TEST_APP_PORT=8080", output
  end
end
