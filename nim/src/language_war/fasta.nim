## FASTA record and iterator.  Supports plain, gzip-compressed, and stdin
## input sources.

import streams, gzipfiles, os
from strutils import startsWith, strip

type
  FastaRecord* = object
    ## A single FASTA record: a description line (sans '>') and a
    ## sanitised nucleotide sequence.
    description*: string
    sequence*: string

  FastaIterError* = object of CatchableError
    ## Raised when reading a FASTA file fails.

  FastaIter* = ref object
    ## Stateful iterator over FASTA records from a stream.
    ## Use one of the ``newFastaIter*`` constructors to obtain an instance,
    ## then iterate with ``for record in iter``.
    stream: Stream
    lineBuffer: string
    pendingHeader: Option[string]
    finished: bool

# ---------------------------------------------------------------------------
# FastaRecord
# ---------------------------------------------------------------------------

proc newFastaRecord*(description, sequence: string): FastaRecord =
  ## Creates a `FastaRecord`, keeping only ASCII alphabetic characters
  ## in the sequence.
  result.description = description
  for c in sequence:
    if c.isAlphaAscii:
      result.sequence.add(c)

proc description*(rec: FastaRecord): string {.inline.} =
  rec.description

proc sequence*(rec: FastaRecord): string {.inline.} =
  rec.sequence

proc reverseComplement*(rec: FastaRecord): string =
  ## Convenience: reverse complement of this record's sequence.
  from language_war/sequence import reverseComplement
  language_war/sequence.reverseComplement(rec.sequence)

proc `$`*(rec: FastaRecord): string =
  ## Formats the record in FASTA format (60-character wrapped lines).
  result.add ">" & rec.description & "\n"
  var i = 0
  while i < rec.sequence.len:
    let chunkLen = min(60, rec.sequence.len - i)
    result.add rec.sequence[i ..< i + chunkLen]
    result.add "\n"
    i += chunkLen

# ---------------------------------------------------------------------------
# FastaIter — constructors
# ---------------------------------------------------------------------------

proc newFastaIter*(stream: Stream): FastaIter =
  ## Low-level constructor.  Prefer the convenience constructors below.
  result = FastaIter(
    stream: stream,
    lineBuffer: newStringOfCap(96),
    pendingHeader: none(string),
    finished: false,
  )

proc newFastaIterFromFile*(filename: string): FastaIter =
  ## Opens a plain FASTA file for iteration.
  let s = newFileStream(filename, fmRead)
  if s.isNil:
    raise newException(FastaIterError, "cannot open FASTA file: " & filename)
  result = newFastaIter(s)

proc newFastaIterFromGzFile*(filename: string): FastaIter =
  ## Opens a gzip-compressed FASTA file (``.gz``) for iteration.
  let gzs = newGzFileStream(filename)
  if gzs.isNil:
    raise newException(FastaIterError, "cannot open gzipped FASTA file: " & filename)
  result = newFastaIter(gzs)

proc newFastaIterFromStdin*(): FastaIter =
  ## Iterates FASTA records from stdin.
  newFastaIter(newFileStream(stdin))

proc newFastaIterFromGzStdin*(): FastaIter =
  ## Iterates FASTA records from gzip-compressed stdin.
  let gzs = newGzFileStream(stdin)
  result = newFastaIter(gzs)

proc newFastaIterFromString*(data: string): FastaIter =
  ## Iterates FASTA records from an in-memory string.
  newFastaIter(newStringStream(data))

# ---------------------------------------------------------------------------
# FastaIter — iteration
# ---------------------------------------------------------------------------

proc close*(iter: FastaIter) =
  ## Release the underlying stream.
  if not iter.stream.isNil:
    iter.stream.close()
    iter.stream = nil

iterator items*(iter: FastaIter): FastaRecord =
  ## Yields successive `FastaRecord` objects.
  ##
  ## Lines starting with `;` are treated as comments and skipped.
  ## Non-alphabetic characters in sequence lines are silently stripped.
  ##
  ## *Note*: unlike the Rust implementation this iterator does not return
  ## ``Result`` — I/O errors are raised as `FastaIterError` exceptions.
  template err(msg: string) = raise newException(FastaIterError, msg)
  # -- placeholder: logic to be filled in --
  discard
