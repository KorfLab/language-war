## `params` — Read a JSON parameter file and pretty-print it (Nim version)
##
## Usage::
##
##   params <json_file>

import os, json

when isMainModule:
  if paramCount() != 1:
    quit "usage: " & getAppFilename() & " <json_file>", QuitFailure

  let path = paramStr(1)
  if not fileExists(path):
    quit "file not found: " & path, QuitFailure

  let jsonStr = readFile(path)
  let node = parseJson(jsonStr)
  echo node.pretty
