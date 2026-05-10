## k-mer counting over a collection of FASTA records.

import tables, math
import language_war/fasta, language_war/sequence

proc countKmers*(seqs: openArray[FastaRecord], k: int, anti: bool): CountTable[string] =
  ## Returns a `CountTable` of all `k`-mers across every record in `seqs`.
  ##
  ## When ``anti`` is `true`, the reverse complement of each k-mer is also
  ## counted (stranded / double-strand counting).
  let estimatedUnique = 4 ^ k
    # rough estimate of unique k-mers, divided by 2 for anti counting
  result = initCountTable[string](estimatedUnique)

  for record in seqs:
    let seq = record.sequence
    for i in 0 .. seq.len - k:
      let kmer = seq[i ..< i + k]
      result.inc(kmer)
      if anti:
        let antiKmer = reverseComplement(kmer)
        result.inc(antiKmer)
  discard
