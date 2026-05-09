## Genotype-by-sequencing Monte-Carlo simulator.

import tables
import algorithm
import std/typedthreads
import std/random
import std/strformat

## Core (single-threaded)
proc countGenotypes*(
    iterations: int, depth: int, errRate: float
): tuple[homoCounts: CountTable[string], heteroCounts: CountTable[string]] =
  ## Simulate `iterations` homozygous and heterozygous genotype calls each,
  ## at the given sequencing `depth` and `errRate`.
  ##
  ## Each genotype is represented as a signature string of the form
  ## ``"a.c.g.t"`` where the four integers are the sorted (descending)
  ## nucleotide observation counts.
  var homoCounts = initCountTable[string]()
  var heteroCounts = initCountTable[string]()

  var rng = initRand()

  for _ in 1 .. iterations:
    # 0 = A, 1 = C, 2 = G, 3 = T
    var homObs = [0, 0, 0, 0]
    for _ in 1 .. depth:
      if rng.rand(1.0) < errRate:
        # Error: observe a different nucleotide
        let nucleotide = rng.rand(1 .. 3)
        homObs[nucleotide] += 1
      else:
        # No error: observe the correct nucleotide (A)
        homObs[0] += 1
    homObs.sort(SortOrder.Descending)
    let sig = &"{homObs[0]}.{homObs[1]}.{homObs[2]}.{homObs[3]}"
    homoCounts.inc(sig)

  for _ in 1 .. iterations:
    var hetObs = [0, 0, 0, 0]
    case rng.rand(0 .. 1)
    # from mom: 'A'
    of 0:
      for _ in 1 .. depth:
        if rng.rand(1.0) < errRate:
          # Error: observe a different nucleotide
          let nucleotide = rng.rand(1 .. 3)
          hetObs[nucleotide] += 1
        else:
          # No error: observe the correct nucleotide (A)
          hetObs[0] += 1
    # from dad: 'T'
    of 1:
      for _ in 1 .. depth:
        if rng.rand(1.0) < errRate:
          # Error: observe a different nucleotide
          let nucleotide = rng.rand(0 .. 2)
          hetObs[nucleotide] += 1
        else:
          # No error: observe the correct nucleotide (T)
          hetObs[3] += 1
    else:
      assert false, "unreachable"

    hetObs.sort(SortOrder.Descending)
    let sig = &"{hetObs[0]}.{hetObs[1]}.{hetObs[2]}.{hetObs[3]}"
    heteroCounts.inc(sig)

  result = (homoCounts, heteroCounts)

type GenotypeThreadArgs = object
  iterations: int
  depth: int
  errRate: float
  homoCountsChannel: ptr Channel[CountTable[string]]
  heteroCountsChannel: ptr Channel[CountTable[string]]

## Thread wrapper for `countGenotypes`. Runs the function and sends results back
proc countGenotypesThreadWrapper(args: GenotypeThreadArgs) {.thread.} =
  let (homoCounts, heteroCounts) =
    countGenotypes(args.iterations, args.depth, args.errRate)
  args.homoCountsChannel[].send(homoCounts)
  args.heteroCountsChannel[].send(heteroCounts)

## Core (parallel)
proc countGenotypesParallel*(
    iterations: int, depth: int, errRate: float, numThreads: int
): tuple[homoCounts: CountTable[string], heteroCounts: CountTable[string]] {.
    raises: [ResourceExhaustedError]
.} =
  ## Threaded version of `countGenotypes`.
  ## Splits `iterations` evenly (remainder on the last thread), runs each
  ## chunk on a separate thread, then merges results.
  let perThread = iterations div numThreads
  let remainder = iterations mod numThreads
  var homoCounts = initCountTable[string]()
  var heteroCounts = initCountTable[string]()

  let homoChannel = cast[ptr Channel[CountTable[string]]](allocShared0(
    sizeof(Channel[CountTable[string]])
  ))
  let heteroChannel = cast[ptr Channel[CountTable[string]]](allocShared0(
    sizeof(Channel[CountTable[string]])
  ))

  homoChannel[].open()
  heteroChannel[].open()

  var threads: seq[Thread[GenotypeThreadArgs]] = @[]

  for i in 1 .. numThreads:
    let threadIterations =
      if i == numThreads:
        perThread + remainder
      else:
        perThread
    let args = GenotypeThreadArgs(
      iterations: threadIterations,
      depth: depth,
      errRate: errRate,
      homoCountsChannel: homoChannel,
      heteroCountsChannel: heteroChannel,
    )
    var thread: Thread[GenotypeThreadArgs]
    thread.createThread(countGenotypesThreadWrapper, args)
    threads.add(thread)

  threads.joinThreads()

  for _ in 1 .. numThreads:
    let threadHomoCounts = homoChannel[].recv()
    let threadHeteroCounts = heteroChannel[].recv()

    homoCounts.merge(threadHomoCounts)
    heteroCounts.merge(threadHeteroCounts)

  # cleanup
  homoChannel[].close()
  heteroChannel[].close()
  deallocShared(homoChannel)
  deallocShared(heteroChannel)

  result = (homoCounts, heteroCounts)
