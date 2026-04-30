## Sequence manipulation utilities: nucleotide complement, reverse complement,
## and Shannon entropy of a 4-nt composition vector.

import math

proc complementBase*(base: char): char =
  ## Returns the complementary nucleotide for a single IUPAC base.
  ## Unrecognized characters are returned unchanged.
  case base
  of 'A': 'T'
  of 'C': 'G'
  of 'G': 'C'
  of 'T': 'A'
  of 'R': 'Y'
  of 'Y': 'R'
  of 'M': 'K'
  of 'K': 'M'
  of 'W': 'W'
  of 'S': 'S'
  of 'B': 'V'
  of 'D': 'H'
  of 'H': 'D'
  of 'V': 'B'
  else: base

proc reverseComplement*(sequence: string): string =
  ## Returns the reverse complement of a nucleotide sequence.
  for i in countdown(sequence.high, 0):
    result.add complementBase(sequence[i])

proc entropy*(a, c, g, t: int): float =
  ## Shannon entropy (bits) of a 4-nucleotide composition.
  ## Returns 0.0 when the total count is zero.
  let total = float(a + c + g + t)
  if total == 0.0:
    return 0.0
  var ent = 0.0
  for count in @[a, c, g, t]:
    if count > 0:
      let p = float(count) / total
      ent -= p * log2(p)
  result = ent
