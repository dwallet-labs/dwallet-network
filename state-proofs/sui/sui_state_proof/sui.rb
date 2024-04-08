class Sui < Formula
  desc "Next-generation smart contract platform powered by the Move programming language"
  homepage "https://sui.io"
  url "https://github.com/MystenLabs/sui/releases/tag/sui_v1.23.0_1712604813_ci"
  sha256 "44a1fbe38671c7223141d3c20135bfc5b89b9bc75830abc390240e70094ec5bf"
  license "Apache-2.0"

  livecheck do
    url :stable
    regex(/^testnet[._-]v?(\d+(?:\.\d+)+)$/i)
  end

  bottle do
    sha256 cellar: :any_skip_relocation, arm64_sonoma:   "53df5a74a959b842866df112070849d7364cf824e84396646b0c9fc0b0620a02"
    sha256 cellar: :any_skip_relocation, arm64_ventura:  "84e96c559893d01c697a171730bddac1ca7cd163db5bf9009f1f86a762fd268d"
    sha256 cellar: :any_skip_relocation, arm64_monterey: "2166273b32155fe3cd89cd6a3a5b768816c18752b98e7dacd8570302a06f0425"
    sha256 cellar: :any_skip_relocation, sonoma:         "b28c7cc55e84372a8af96ea4dde20612db6739b04950aae30a8f75b7a7e0a7ed"
    sha256 cellar: :any_skip_relocation, ventura:        "f6cef7bc2791b2a1108fc65e24f1946ccef0ee5f649ed60dd9a5c3eac37a8b55"
    sha256 cellar: :any_skip_relocation, monterey:       "9aa631ad1a3a0b4cf342b05b609bfa146bffbf6c084600acc8f033b879937773"
    sha256 cellar: :any_skip_relocation, x86_64_linux:   "53c821fc1dab7a51aa74d3ba878c0308d1ffc95d8dcee06fbadc69144075a595"
  end

  depends_on "cmake" => :build
  depends_on "libpq" => :build
  depends_on "rust" => :build

  on_linux do
    depends_on "llvm" => :build
  end

  def install
    system "cargo", "install", *std_cargo_args(path: "crates/sui")
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/sui --version")

    (testpath/"testing.keystore").write <<~EOS
      [
        "AOLe60VN7M+X7H3ZVEdfNt8Zzsj1mDJ7FlAhPFWSen41"
      ]
    EOS
    keystore_output = shell_output("#{bin}/sui keytool --keystore-path testing.keystore list")
  end
end
