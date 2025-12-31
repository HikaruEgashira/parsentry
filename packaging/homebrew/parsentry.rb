# Homebrew Formula for Parsentry
# SHA256 hashes are automatically updated by the release workflow.
#
# Setup a tap repository:
#   gh repo create homebrew-parsentry --public
#   cp this file to Formula/parsentry.rb in the tap repository
#
# Users can then install with:
#   brew tap HikaruEgashira/parsentry
#   brew install parsentry

class Parsentry < Formula
  desc "PAR-based security scanner using LLMs and static code analysis"
  homepage "https://github.com/HikaruEgashira/parsentry"
  version "0.14.0"
  license "AGPL-3.0-only"

  on_macos do
    on_arm do
      url "https://github.com/HikaruEgashira/parsentry/releases/download/v#{version}/parsentry-aarch64-apple-darwin.tar.gz"
      sha256 "PLACEHOLDER_AARCH64_APPLE_DARWIN"
    end
  end

  on_linux do
    on_intel do
      url "https://github.com/HikaruEgashira/parsentry/releases/download/v#{version}/parsentry-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "PLACEHOLDER_X86_64_LINUX_GNU"
    end
  end

  def install
    bin.install "parsentry"
  end

  test do
    assert_match "parsentry", shell_output("#{bin}/parsentry --version")
  end
end
