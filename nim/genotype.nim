## `genotype` — Genotype-by-sequencing Monte-Carlo simulator (Nim version)
##
## Usage::
##
##   genotype <iterations> <depth> [-e|--err-rate=F] [--threads=N] [-o|--output=PATH]
##
## Simulates homozygous and heterozygous genotype observations and reports
## signature probabilities.

import parseopt, strutils, options, algorithm, tables
import std/strformat
import language_war/genotype

type CliArgs = object
  iterations: float
  depth: int
  errRate: float
  threads: int
  outputPath: Option[string]

proc parseArgs(): CliArgs =
  result = CliArgs(errRate: 0.1, threads: 2)
  var p = initOptParser(mode = LaxMode)
  while true:
    p.next()
    case p.kind
    of cmdEnd:
      break
    of cmdShortOption, cmdLongOption:
      case p.key
      of "e", "err-rate":
        result.errRate = parseFloat(p.val)
      of "threads":
        result.threads = parseInt(p.val)
      of "o", "output":
        result.outputPath = some(p.val)
      else:
        discard
    of cmdArgument:
      if result.iterations == 0.0:
        result.iterations = parseFloat(p.key)
      elif result.depth == 0:
        result.depth = parseInt(p.key)

when isMainModule:
  let args = parseArgs()
  if args.iterations == 0.0 or args.depth == 0:
    quit "usage: genotype <iterations> <depth> [--err-rate=F] [--threads=N] [-o PATH]",
      QuitFailure

  let (homoCounts, heteroCounts) =
    countGenotypesParallel(int(args.iterations), args.depth, args.errRate, args.threads)

  var outputStream: File
  if args.outputPath.isSome:
    outputStream = open(args.outputPath.get, fmWrite)
  else:
    outputStream = stdout

  var allSigs: seq[string]
  for k in homoCounts.keys:
    allSigs.add(k)
  for k in heteroCounts.keys:
    if k notin allSigs:
      allSigs.add(k)
  allSigs.sort()

  # Compute column widths for alignment
  var sigWidth = 3
  var homWidth = 3
  var hetWidth = 3
  const phomWidth = 8
  for sig in allSigs:
    sigWidth = max(sigWidth, sig.len)
    let hom = homoCounts.getOrDefault(sig, 0)
    let het = heteroCounts.getOrDefault(sig, 0)
    homWidth = max(homWidth, ($hom).len)
    hetWidth = max(hetWidth, ($het).len)

  let (sigH, homH, hetH, phomH) = ("Sig", "Hom", "Het", "P(hom)")
  outputStream.writeLine(alignLeft(sigH, sigWidth) & " " & align(homH, homWidth) & " " &
                         align(hetH, hetWidth) & " " & align(phomH, phomWidth))
  outputStream.writeLine("-".repeat(sigWidth) & " " & "-".repeat(homWidth) & " " &
                         "-".repeat(hetWidth) & " " & "-".repeat(phomWidth))

  for i in countdown(allSigs.high, 0):
    let sig = allSigs[i]
    let hom = homoCounts.getOrDefault(sig, 0)
    let het = heteroCounts.getOrDefault(sig, 0)
    let total = hom + het
    let pHom =
      if total > 0:
        float(hom) / float(total)
      else:
        0.0
    outputStream.writeLine(alignLeft(sig, sigWidth) & " " & align($hom, homWidth) & " " &
                           align($het, hetWidth) & " " & align(&"{pHom:.4f}", phomWidth))

  if args.outputPath.isSome:
    outputStream.close()
