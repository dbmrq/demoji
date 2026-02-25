# Homebrew formula for demoji
# Install with: brew install dbmrq/tap/demoji

class Demoji < Formula
  desc "A fast CLI tool to remove or replace emoji characters from text files"
  homepage "https://github.com/dbmrq/demoji"
  url "https://github.com/dbmrq/demoji.git", tag: "v0.1.0"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", "--locked", "--root", prefix, "--path", "."
  end

  test do
    system "#{bin}/demoji", "--version"
  end
end
