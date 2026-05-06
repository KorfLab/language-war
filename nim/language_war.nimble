# Package

version       = "0.1.0"
author        = "Shanoa Ice"
description   = "KorfLab Language War libraries and binaries (Nim)"
license       = "MIT"
installExt = @["nim"]
srcDir        = "src"
bin = @[
  "dust",
  "kmers",
  "genotype",
  "exons",
  "params"
]

# Dependencies

requires "nim >= 2.2.10"
