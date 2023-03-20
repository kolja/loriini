class loriini < Formula
  desc "A commandline color picker"
  homepage "https://github.com/kolja/loriini"
  url "https://github.com/kolja/loriini/releases/download/v0.1.2/loriini-x86_64-apple-darwin.tar.gz"
  sha256 "326ade6f4a3749bfb0a2ac71c22041ac1bf2ccfd0592adf9664dbe4a7c11a4a1"
  license "MIT"

  def install
    system "tar", "-xzf", "loriini-x86_64-apple-darwin.tar.gz"
    bin.install "loriini"
  end

  test do
    system "#{bin}/mybinary", "--version"
  end
end
