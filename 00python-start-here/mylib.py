import gzip

COMPLEMENT = str.maketrans('ACGTRYMKWSBDHV', 'TGCAYRKMWSVHDB')

def anti(seq):
	"""Returns the reverse complement of a sequence"""
	anti = seq.translate(COMPLEMENT)[::-1]
	return anti

def getfp(filename):
	"""Returns a file pointer for reading based on file name"""
	if   filename.endswith('.gz'): return gzip.open(filename, 'rt')
	elif filename == '-':          return sys.stdin
	else:                          return open(filename)

def readfasta(filename):
	"""Simple fasta file iterator: yields defline, seq"""
	name = None
	seqs = []
	fp = getfp(filename)
	while True:
		line = fp.readline()
		if line == '': break
		line = line.rstrip()
		if line.startswith('>'):
			if len(seqs) > 0:
				seq = ''.join(seqs)
				yield name, seq
				name = line[1:]
				seqs = []
			else:
				name = line[1:]
		else:
			seqs.append(line)
	yield name, ''.join(seqs)
	fp.close()

