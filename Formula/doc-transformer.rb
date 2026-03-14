class Ctd < Formula
  desc "Transform markdown documentation into AI-optimized searchable structures"
  homepage "https://github.com/lprior-repo/centralized-docs"
  url "https://github.com/lprior-repo/centralized-docs/archive/refs/tags/v0.6.1.tar.gz"
  sha256 "TODO"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args(path: "centralized-docs", bin: "ctd")
  end

  test do
    assert_match "Transform documentation", shell_output("#{bin}/ctd --help")
  end
end
