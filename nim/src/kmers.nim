## `kmers` — k-mer counter (Nim version)
##
## Usage::
##
##   kmers <fasta> <k> [--anti]
##
## Counts k-mers across all FASTA records, optionally counting both strands.
## Output format: ``kmer\\tcount\\tfrequency``

import parseopt, os, strutils
import language_war / [fasta, kmer]

type
  CliArgs = object
    fastaPath: string
    k: int
    anti: bool

proc parseArgs(): CliArgs =
  var p = initOptParser()
  while true:
    p.next()
    case p.kind
    of cmdEnd: break
    of cmdShortOption, cmdLongOption:
      case p.key
      of "anti": result.anti = true
      else: discard
    of cmdArgument:
      if result.fastaPath.len == 0:
        result.fastaPath = p.key
      elif result.k == 0:
        result.k = parseInt(p.key)

when isMainModule:
  let args = parseArgs()
  if args.fastaPath.len == 0 or args.k == 0:
    quit "usage: kmers <fasta> <k> [--anti]", QuitFailure

  let iter = if args.fastaPath == "-":
      newFastaIterFromStdin()
    elif args.fastaPath.endsWith(".gz"):
      newFastaIterFromGzFile(args.fastaPath)
    else:
      newFastaIterFromFile(args.fastaPath)

  var records: seq[FastaRecord]
  for rec in iter:
    records.add(rec)
  iter.close()

  let kmerCounts = countKmers(records, args.k, args.anti)
  var totalKmers = 0
  for c in kmerCounts.values:
    totalKmers += c
  var sortedKeys = toSeq(kmerCounts.keys)
  sortedKeys.sort()

  for kmer in sortedKeys:
    let n = kmerCounts[kmer]
    echo kmer, "\t", n, "\t", float(n) / float(totalKmers)
