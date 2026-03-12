class DocTransformer < Formula
  desc "Transform markdown documentation into AI-optimized searchable structures"
  homepage "https://github.com/lprior-repo/centralized-docs"
  url "https://github.com/lprior-repo/centralized-docs/archive/refs/tags/v6.1.0.tar.gz"
  sha256 "da7427fb7501971585953033149ca0e2a494da88ed7dfa87a650f3cdc8776015"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args(path: "doc_transformer")
  end

  test do
    assert_match "Transform documentation", shell_output("#{bin}/doc_transformer --help")
  end
end
