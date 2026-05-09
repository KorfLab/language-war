## FASTA record and iterator.  Supports plain, gzip-compressed, and stdin
## input sources.

import options
from sequence import reverseComplement
import std/[streams, ropes, strutils]

type
  FastaRecord* = object
    ## A single FASTA record: a description line (sans '>') and a
    ## sanitised nucleotide sequence.
    description*: string
    sequence*: string

  FastaIterError* = object of CatchableError ## Raised when reading a FASTA file fails.

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
  sequence.reverseComplement(rec.sequence)

proc `$`*(rec: FastaRecord): string =
  ## Formats the record in FASTA format (60-character wrapped lines).
  result.add(">" & rec.description & "\n")
  var i = 0
  while i < rec.sequence.len:
    let chunkLen = min(60, rec.sequence.len - i)
    result.add(rec.sequence[i ..< i + chunkLen])
    result.add("\n")
    i += chunkLen

# ---------------------------------------------------------------------------
# FastaIter — iteration
# ---------------------------------------------------------------------------

## Yields successive `FastaRecord` objects.
##
## Lines starting with `;` are treated as comments and skipped.
## Non-alphabetic characters in sequence lines are silently stripped.
##
## *Note*: unlike the Rust implementation this iterator does not return
## ``Result`` — I/O errors are raised as `FastaIterError` exceptions.
iterator fastaRecords*(stream: Stream): FastaRecord {.inline.} =
  template err(msg: string) =
    raise newException(FastaIterError, msg)

  var sequence: Rope = nil
  var pendingDesc: Option[string] = none(string)
  var lineBuffer = newStringOfCap(96)

  if not stream.atEnd:
    # -- read a line into lineBuffer --
    # -- if it's a header, store it in pendingDesc and continue --
    # -- if it's a comment, skip it --
    # -- otherwise, it's a sequence line: combine with pending header to yield a FastaRecord --
    # -- if we reach the end of the stream, yield any pending record and finish --
    while stream.readLine(lineBuffer):
      if lineBuffer.startsWith('>'):
        if pendingDesc.isSome:
          yield newFastaRecord(pendingDesc.get(), $sequence)
          sequence = nil
        lineBuffer.removePrefix('>')
        pendingDesc = some(lineBuffer.strip())
      elif lineBuffer.startsWith(';'):
        continue
      else:
        sequence.add(lineBuffer)

    # EOF: yield any pending record and finish
    if pendingDesc.isSome or sequence.isNil:
      yield newFastaRecord(pendingDesc.get(""), $sequence)
