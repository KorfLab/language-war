# Package

version       = "0.1.0"
author        = "Shanoa Ice"
description   = "KorfLab Language War libraries and binaries (Nim)"
license       = "MIT"
srcDir        = "src"

# Dependencies

requires "nim >= 2.2.10"

# Binaries — one per bioinformatics tool

installExt = @["nim"]
bin = @[
  "dust",
  "kmers",
  "genotype",
  "exons",
  "params",
  "occasionally_dishonest_casino",
]
