## Simple 2-D matrix (`Vec2[T]`) with optional keyed access (`IndexedVec2`).

import tables

type
  Vec2*[T] = object
    ## Row-major matrix backed by a flat ``seq[T]``.
    data: seq[T]
    cols: int

  IndexedVec2*[R, C, T] = object
    ## Read-only keyed view over a `Vec2[T]`.
    ## Uses `rowIndices` and `colIndices` hash tables to translate
    ## key lookups into matrix coordinates.
    data: ptr Vec2[T]
    rowIndices: ptr Table[R, int]
    colIndices: ptr Table[C, int]

# ---------------------------------------------------------------------------
# Vec2[T] constructors & accessors
# ---------------------------------------------------------------------------

proc initVec2*[T](rows, cols: int, default: T): Vec2[T] =
  ## Create a matrix filled with `default`.
  result.data = newSeq[T](rows * cols)
  for i in 0 ..< result.data.len:
    result.data[i] = default
  result.cols = cols

proc initVec2Default*[T](rows, cols: int): Vec2[T] =
  ## Create a matrix filled with `T.default` (zero value).
  result.data = newSeq[T](rows * cols)
  result.cols = cols

proc fromFlat*[T](data: seq[T], rows, cols: int): Vec2[T] =
  ## Create a matrix from a flat sequence.
  assert data.len == rows * cols, "Dimension mismatch"
  result.data = data
  result.cols = cols

proc get*[T](m: Vec2[T], row, col: int): T {.inline.} =
  m.data[row * m.cols + col]

proc getPtr*[T](m: var Vec2[T], row, col: int): ptr T {.inline.} =
  ## Mutable access via pointer (Nim idiom for in-place mutation).
  addr m.data[row * m.cols + col]

proc set*[T](m: var Vec2[T], row, col: int, value: T) {.inline.} =
  m.data[row * m.cols + col] = value

proc cols*[T](m: Vec2[T]): int {.inline.} = m.cols

proc rows*[T](m: Vec2[T]): int {.inline.} =
  if m.cols == 0: 0 else: m.data.len div m.cols

proc row*[T](m: Vec2[T], r: int): seq[T] =
  ## Return a copy of row `r`.
  let start = r * m.cols
  m.data[start ..< start + m.cols]

# ---------------------------------------------------------------------------
# IndexedVec2
# ---------------------------------------------------------------------------

proc initIndexedVec2*[R, C, T](
    data: ptr Vec2[T],
    rowIndices: ptr Table[R, int],
    colIndices: ptr Table[C, int],
  ): IndexedVec2[R, C, T] =
  IndexedVec2[R, C, T](data: data, rowIndices: rowIndices, colIndices: colIndices)

proc get*[R, C, T](v: IndexedVec2[R, C, T], rowKey: R, colKey: C): Option[T] =
  ## Keyed lookup; returns `none(T)` when either key is missing.
  let row = v.rowIndices[].getOrDefault(rowKey, -1)
  let col = v.colIndices[].getOrDefault(colKey, -1)
  if row < 0 or col < 0:
    return none(T)
  some(v.data[].get(row, col))

proc rowIndex*[R, C, T](v: IndexedVec2[R, C, T], key: R): Option[int] =
  if v.rowIndices[].hasKey(key):
    some(v.rowIndices[][key])
  else:
    none(int)

proc colIndex*[R, C, T](v: IndexedVec2[R, C, T], key: C): Option[int] =
  if v.colIndices[].hasKey(key):
    some(v.colIndices[][key])
  else:
    none(int)
