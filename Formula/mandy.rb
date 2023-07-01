class Mandy < Formula
  desc "A hypersonic static-site generator written in Rust."
  homepage "https://github.com/angeldollface/mandy"
  url "https://github.com/angeldollface/mandy/releases/latest/download/mandy-ventura-x86_64-0.1.0.tar.gz"
  sha256 "e04db523c81eaa43668274c480f391f080b7990661768711ede50321c4aaf4d6"
  version "0.1.0"

  def install
    bin.install "mandy"
  end
end