## k-mer counting over a collection of FASTA records.

import tables
import language_war/fasta, language_war/sequence

proc countKmers*(seqs: openArray[FastaRecord], k: int, anti: bool):
    CountTable[string] =
  ## Returns a `CountTable` of all `k`-mers across every record in `seqs`.
  ##
  ## When ``anti`` is `true`, the reverse complement of each k-mer is also
  ## counted (stranded / double-strand counting).
  let estimatedUnique = 4 ^ k
  result = initCountTable[string](rightSize(estimatedUnique))
  discard
