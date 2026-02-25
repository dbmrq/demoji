# Homebrew formula for demoji
# Install with: brew install demoji
# Or from tap: brew tap yourusername/demoji && brew install demoji

class Demoji < Formula
  desc "A fast CLI tool to remove or replace emoji characters from source code files"
  homepage "https://github.com/yourusername/demoji"
  license "MIT"
  
  # Source URL placeholder - replace with actual GitHub release URL when publishing
  # Format: https://github.com/yourusername/demoji/releases/download/v{version}/demoji-{version}-{target}.tar.gz
  url "https://github.com/yourusername/demoji/releases/download/v0.1.0/demoji-0.1.0-x86_64-apple-darwin.tar.gz"
  sha256 "PLACEHOLDER_SHA256_HASH"
  
  # Alternative: Build from source using Rust
  # Uncomment the following lines and comment out the url/sha256 above to build from source
  # url "https://github.com/yourusername/demoji.git", tag: "v0.1.0"
  # depends_on "rust" => :build
  
  depends_on "rust" => :build if build.with? "from-source"
  
  option "with-from-source", "Build from source instead of using pre-built binary"
  
  def install
    if build.with? "from-source"
      # Build from source using cargo
      system "cargo", "install", "--locked", "--root", prefix, "--path", "."
    else
      # Install pre-built binary
      bin.install "demoji"
    end
  end
  
  def caveats
    <<~EOS
      demoji has been installed successfully!
      
      Quick start:
        demoji --help                    # Show help
        demoji --dry-run src/            # Preview changes
        demoji src/                      # Remove emojis from source files
        demoji init                      # Create .demoji.toml config
        demoji watch src/                # Watch for file changes
      
      Configuration:
        Create a .demoji.toml file in your project root to customize behavior.
        Run 'demoji init' to generate a template.
    EOS
  end
  
  test do
    # Basic smoke test
    system "#{bin}/demoji", "--version"
    
    # Test with a temporary file
    (testpath/"test.rs").write("fn main() { println!(\"Hello 👋\"); }")
    output = shell_output("#{bin}/demoji --dry-run #{testpath}/test.rs")
    assert_match /emoji/, output.downcase
  end
end
