## `occasionally_dishonest_casino` — Hidden Markov Model demo (Nim version)
##
## Simulates the "occasionally dishonest casino" problem: a dealer switches
## between a fair die and a loaded die, then the Viterbi algorithm recovers
## the most probable state path.
##
## Usage::
##
##   occasionally_dishonest_casino [--num-observations=N]

import parseopt, strutils, tables, random
import language_war / hidden_markov

type
  State = enum
    Fair, Loaded

  Observation = enum
    One = 1, Two, Three, Four, Five, Six

type
  CliArgs = object
    numObservations: int

proc parseArgs(): CliArgs =
  result = CliArgs(numObservations: 20)
  var p = initOptParser()
  while true:
    p.next()
    case p.kind
    of cmdEnd: break
    of cmdShortOption, cmdLongOption:
      case p.key
      of "num-observations": result.numObservations = parseInt(p.val)
      else: discard
    of cmdArgument: discard

when isMainModule:
  let args = parseArgs()

  # -- build HMM --
  let states = @[Fair, Loaded]
  let observationSpace = @[One, Two, Three, Four, Five, Six]

  var initProbs = {Fair: 0.5, Loaded: 0.5}.toTable

  var transMatrix = {
    Fair:   {Fair: 0.8, Loaded: 0.2}.toTable,
    Loaded: {Fair: 0.5, Loaded: 0.5}.toTable,
  }.toTable

  var emissMatrix = {
    Fair: {
      One: 1.0/6.0, Two: 1.0/6.0, Three: 1.0/6.0,
      Four: 1.0/6.0, Five: 1.0/6.0, Six: 1.0/6.0,
    }.toTable,
    Loaded: {
      One: 0.1, Two: 0.1, Three: 0.1,
      Four: 0.1, Five: 0.1, Six: 0.5,
    }.toTable,
  }.toTable

  var hmm = newHiddenMarkov[Observation, State](
    states, observationSpace, initProbs, transMatrix, emissMatrix)

  # -- simulate observations --
  var rng = initRand()
  var observations: seq[Observation]
  var trueStates: seq[State]
  var currentState = if rng.rand(1.0) < 0.5: Fair else: Loaded

  for _ in 1 .. args.numObservations:
    trueStates.add(currentState)
    # sample emission ...
    # sample transition ...
    discard  # -- placeholder: simulation logic --
    observations.add(One)  # stub

  # -- decode --
  let bestPath = hmm.calculate(observations)

  # -- report --
  proc obsToStr(o: Observation): string = $o
  proc stateToStr(s: State): string = if s == Fair: "F" else: "L"

  echo "Observations:  ", observations.map(obsToStr).join(" ")
  echo "True states:   ", trueStates.map(stateToStr).join(" ")
  echo "Viterbi path:  ", bestPath.map(stateToStr).join(" ")
