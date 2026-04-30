## DUST low-complexity filtering (sliding-window Shannon entropy mask).

import language_war/fasta, language_war/sequence

proc mask*(seq: string, windowSize: int, entropyThreshold: float, softMask: bool): string =
  ## Returns a copy of `seq` with low-complexity windows masked.
  ##
  ## - ``windowSize``   — sliding window width (nucleotides).
  ## - ``entropyThreshold`` — windows whose Shannon entropy (bits) falls
  ##   below this value are masked.
  ## - ``softMask``     — if `true`, use lowercase letters; otherwise `N`.
  runnableExamples:
    let masked = mask("ACGTACGTACGTNNN", windowSize = 6, entropyThreshold = 1.4,
                       softMask = false)
  discard

proc dustMasked*(rec: FastaRecord, windowSize: int, entropyThreshold: float,
                 softMask: bool): FastaRecord =
  ## Returns a new `FastaRecord` with the sequence masked via `mask`.
  let maskedSeq = mask(rec.sequence, windowSize, entropyThreshold, softMask)
  result = newFastaRecord(rec.description, maskedSeq)
