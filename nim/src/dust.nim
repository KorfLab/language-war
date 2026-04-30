## `dust` — DUST low-complexity filter (Nim version)
##
## Usage::
##
##   dust [--size=N] [--entropy=F] [--lower] <fasta>
##
## Reads a FASTA file, masks low-complexity windows using a sliding
## Shannon-entropy filter, and writes the masked sequence to stdout.
##
## .. code-block:: console
##
##    nim c -d:release -o:dust src/dust.nim
##    ./dust --size=20 --entropy=1.4 --lower ce1pct.fa.gz

import parseopt, os, strutils
import language_war / [fasta, dust]

# -- CLI argument parsing --
type
  CliArgs = object
    fastaPath: string
    windowSize: int
    entropyThreshold: float
    softMask: bool

proc parseArgs(): CliArgs =
  result = CliArgs(windowSize: 20, entropyThreshold: 1.4, softMask: false)
  var p = initOptParser()
  while true:
    p.next()
    case p.kind
    of cmdEnd: break
    of cmdShortOption, cmdLongOption:
      case p.key
      of "size", "s": result.windowSize = parseInt(p.val)
      of "entropy", "e": result.entropyThreshold = parseFloat(p.val)
      of "lower": result.softMask = true
      else: discard
    of cmdArgument:
      result.fastaPath = p.key

# -- main --
when isMainModule:
  let args = parseArgs()
  if args.fastaPath.len == 0:
    quit "usage: dust [--size=N] [--entropy=F] [--lower] <fasta>", QuitFailure

  let iter = if args.fastaPath == "-":
      newFastaIterFromStdin()
    elif args.fastaPath.endsWith(".gz"):
      newFastaIterFromGzFile(args.fastaPath)
    else:
      newFastaIterFromFile(args.fastaPath)

  for rec in iter:
    let masked = rec.dustMasked(args.windowSize, args.entropyThreshold,
                                args.softMask)
    stdout.write $masked
  iter.close()
