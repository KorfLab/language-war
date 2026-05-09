# Package

version       = "0.1.0"
author        = "Shanoa Ice"
description   = "KorfLab Language War libraries and binaries (Nim)"
license       = "MIT"
installExt = @["nim"]
bin = @[
  "dust",
# "kmers",
# "genotype",
# "exons",
# "params"
]

# Dependencies

requires "nim >= 2.2.0"
requires "zippy ^= 0.10.19"
requires "nclap"
