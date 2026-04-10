import argparse
import os
import sqlite3
import sys

parser = argparse.ArgumentParser(description='Retrieve exon sequences from db')
parser.add_argument('db', type=str, help='database file')
arg = parser.parse_args()

if not os.path.exists(arg.db): sys.exit(f'aborting: no database {arg.db}')
con = sqlite3.connect(arg.db)
cur = con.cursor()

exon_query = 'SELECT seqid, beg, end, strand from feature WHERE type = "exon"'
for seqid, beg, end, strand in cur.execute(exon_query).fetchall():
	offset = beg
	length = end - beg + 1
	seq_query = f'SELECT substr(seq, {offset}, {length}) from sequence WHERE seqid = "{seqid}"'
	seq = cur.execute(seq_query).fetchone()[0]
	print(seqid, beg, end, strand, seq, sep='\t')

