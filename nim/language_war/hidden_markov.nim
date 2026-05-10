## Generic Hidden Markov Model with construction from hashed probability
## tables and Viterbi decoding.

import tables, options
import language_war/collections

type
  HmmParseError* = object of CatchableError
    ## Raised when HMM construction fails (duplicate state/observation,
    ## missing probability, etc.).

  HiddenMarkov*[O, S] = object
    ## A Hidden Markov Model parameterised by observation type `O` and
    ## state type `S`.
    ##
    ## Both type parameters must be hashable (for `Table` lookup) and
    ## support `==` comparison.
    states: seq[S]
    stateIndices: Table[S, int]
    observations: seq[O]
    obsIndices: Table[O, int]
    initialProbabilities: seq[float]
    transitionMatrix: Vec2[float]
    emissionMatrix: Vec2[float]

# ---------------------------------------------------------------------------
# Construction
# ---------------------------------------------------------------------------

proc newHiddenMarkov*[O, S](
    states: seq[S],
    observations: seq[O],
    initialProbabilities: Table[S, float],
    transitionMatrix: Table[S, Table[S, float]],
    emissionMatrix: Table[S, Table[O, float]],
  ): HiddenMarkov[O, S] =
  ## Build an HMM from hashed probability maps.
  ##
  ## Raises `HmmParseError` on duplicate symbols or missing probabilities.
  ## Zero-probability transitions/emissions may be omitted.
  runnableExamples:
    type
      State = enum Fair, Loaded
      Obs = enum One, Two, Three, Four, Five, Six
    var initProbs = {Fair: 0.5, Loaded: 0.5}.toTable
    # ...
    # let hmm = newHiddenMarkov(...)
  discard

# ---------------------------------------------------------------------------
# Accessors
# ---------------------------------------------------------------------------

proc states*[O, S](hmm: HiddenMarkov[O, S]): seq[S] = hmm.states
proc observations*[O, S](hmm: HiddenMarkov[O, S]): seq[O] = hmm.observations

proc stateIndex*[O, S](hmm: HiddenMarkov[O, S], state: S): Option[int] =
  if hmm.stateIndices.hasKey(state):
    some hmm.stateIndices[state]
  else:
    none(int)

proc observationIndex*[O, S](hmm: HiddenMarkov[O, S], obs: O): Option[int] =
  if hmm.obsIndices.hasKey(obs):
    some hmm.obsIndices[obs]
  else:
    none(int)

proc stateAt*[O, S](hmm: HiddenMarkov[O, S], idx: int): Option[S] =
  if idx >= 0 and idx < hmm.states.len:
    some hmm.states[idx]
  else:
    none(S)

proc initialProbabilities*[O, S](hmm: HiddenMarkov[O, S]): seq[float] =
  hmm.initialProbabilities

proc transitionMatrix*[O, S](hmm: var HiddenMarkov[O, S]): var Vec2[float] =
  hmm.transitionMatrix

proc emissionMatrix*[O, S](hmm: var HiddenMarkov[O, S]): var Vec2[float] =
  hmm.emissionMatrix

proc transition*[O, S](hmm: var HiddenMarkov[O, S]): IndexedVec2[S, S, float] =
  initIndexedVec2(addr hmm.transitionMatrix, addr hmm.stateIndices,
                  addr hmm.stateIndices)

proc emissions*[O, S](hmm: var HiddenMarkov[O, S]): IndexedVec2[S, O, float] =
  initIndexedVec2(addr hmm.emissionMatrix, addr hmm.stateIndices,
                  addr hmm.obsIndices)

proc initialProbability*[O, S](hmm: HiddenMarkov[O, S], state: S): Option[float] =
  if hmm.stateIndices.hasKey(state):
    some hmm.initialProbabilities[hmm.stateIndices[state]]
  else:
    none(float)

proc transitionProbability*[O, S](hmm: var HiddenMarkov[O, S],
                                   fromState, toState: S): Option[float] =
  hmm.transition().get(fromState, toState)

proc emissionProbability*[O, S](hmm: HiddenMarkov[O, S],
                                 state: S, observation: O): float =
  let si = hmm.stateIndices.getOrDefault(state, -1)
  let oi = hmm.obsIndices.getOrDefault(observation, -1)
  if si >= 0 and oi >= 0:
    result = hmm.emissionMatrix.get(si, oi)
  else:
    result = 0.0

# ---------------------------------------------------------------------------
# Viterbi algorithm
# ---------------------------------------------------------------------------

proc calculate*[O, S](hmm: var HiddenMarkov[O, S],
                       observations: seq[O]): seq[S] =
  ## Viterbi decoding: returns the most probable state path given an
  ## observation sequence.
  let nStates = hmm.states.len
  let nObs = observations.len
  if nObs == 0:
    return @[]
  # -- placeholder: DP table + backtrace --
  discard
