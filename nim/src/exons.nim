## `exons` — Retrieve exon sequences from a GFF3 SQLite database (Nim version)
##
## Usage::
##
##   exons --db=<db_path>

import parseopt, os, db_sqlite

type
  CliArgs = object
    dbPath: string

proc parseArgs(): CliArgs =
  var p = initOptParser()
  while true:
    p.next()
    case p.kind
    of cmdEnd: break
    of cmdShortOption, cmdLongOption:
      case p.key
      of "db": result.dbPath = p.val
      else: discard
    of cmdArgument: discard

when isMainModule:
  let args = parseArgs()
  if args.dbPath.len == 0:
    quit "usage: exons --db=<db_path>", QuitFailure
  if not fileExists(args.dbPath):
    quit "aborting: no database " & args.dbPath, QuitFailure

  let db = open(args.dbPath, "", "", "")

  let exonQuery = "SELECT seqid, beg, end, strand FROM feature WHERE type = 'exon'"
  for row in db.fastRows(sql(exonQuery)):
    let seqid = row[0]
    let beg = parseInt(row[1])
    let endPos = parseInt(row[2])
    let strand = row[3]
    let offset = beg
    let length = endPos - beg + 1

    let seqQuery = sql(
      "SELECT substr(seq, ?, ?) FROM sequence WHERE seqid = ?")
    let seq = db.getValue(seqQuery, offset, length, seqid)
    echo seqid, "\t", beg, "\t", endPos, "\t", strand, "\t", seq

  db.close()
