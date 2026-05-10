## DUST low-complexity filtering (sliding-window Shannon entropy mask).

import strutils
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

  var countTable: array[0..127, int]  # ASCII count table
  countTable[cast[int]('a')] = 0
  countTable[cast[int]('c')] = 0
  countTable[cast[int]('g')] = 0
  countTable[cast[int]('t')] = 0

  var maskedSeq = seq;

  for i in 0..<windowSize:
    let base = seq[i].toLowerAscii
    inc countTable[cast[int](base)]
  
  var a = countTable[cast[int]('a')]
  var c = countTable[cast[int]('c')]
  var g = countTable[cast[int]('g')]
  var t = countTable[cast[int]('t')]

  if entropy(a, c, g, t) < entropyThreshold:
    for j in 0..<windowSize:
      maskedSeq[j] = if softMask: maskedSeq[j].toLowerAscii else: 'N'

  for i in 1..(seq.len - windowSize):
    let prevBase = seq[i - 1].toLowerAscii
    let nextBase = seq[i + windowSize - 1].toLowerAscii
    dec countTable[cast[int](prevBase)]
    inc countTable[cast[int](nextBase)]
    a = countTable[cast[int]('a')]
    c = countTable[cast[int]('c')]
    g = countTable[cast[int]('g')]
    t = countTable[cast[int]('t')]
    if entropy(a, c, g, t) < entropyThreshold:
      for j in i..<(i + windowSize):
        maskedSeq[j] = if softMask: maskedSeq[j].toLowerAscii else: 'N'

  result = maskedSeq

proc mask*(rec: FastaRecord, windowSize: int, entropyThreshold: float,
                 softMask: bool): FastaRecord =
  ## Returns a new `FastaRecord` with the sequence masked via `mask`.
  let maskedSeq = mask(rec.sequence, windowSize, entropyThreshold, softMask)
  result = newFastaRecord(rec.description, maskedSeq)
