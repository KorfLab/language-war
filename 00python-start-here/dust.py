import argparse
import math
import mylib

def entropy(a, c, g, t):
	h = 0
	total = a + c + g + t
	pa = a / total
	pc = c / total
	pg = g / total
	pt = t / total
	if pa > 0: h -= pa * math.log2(pa)
	if pc > 0: h -= pc * math.log2(pc)
	if pg > 0: h -= pg * math.log2(pg)
	if pt > 0: h -= pt * math.log2(pt)
	return h


parser = argparse.ArgumentParser(description='DNA entropy filter')
parser.add_argument('fasta', type=str, help='name of fasta file')
parser.add_argument('-s', '--size', type=int, default=20, metavar='<int>',
	help='window size [%(default)i]')
parser.add_argument('-e', '--entropy', type=float, default=1.4, 
	metavar='<float>', help='entropy threshold [%(default).3f]')
parser.add_argument('--lower', action='store_true', help='soft mask')
arg = parser.parse_args()

for defline, seq in mylib.readfasta(arg.fasta):
	mask = list(seq)
	a = seq[0:arg.size].count('A')
	c = seq[0:arg.size].count('C')
	g = seq[0:arg.size].count('G')
	t = seq[0:arg.size].count('T')
	if entropy(a, c, g, t) < arg.entropy:
		if arg.lower:
			for i in range(arg.size): mask[i] = seq[i].lower()
		else:
			for i in range(arg.size): mask[i] = 'N'
	
	for i in range(1, len(seq) - arg.size + 1):
		off = seq[i-1]
		on = seq[i+arg.size-1]
		if   off == 'A': a -= 1
		elif off == 'C': c -= 1
		elif off == 'G': g -= 1
		elif off == 'T': t -= 1
		
		if   on == 'A': a += 1
		elif on == 'C': c += 1
		elif on == 'G': g += 1
		elif on == 'T': t += 1
		
		if entropy(a, c, g, t) < arg.entropy:
			if arg.lower:
				for j in range(i, i+arg.size): mask[j] = seq[j].lower()
			else:
				for j in range(i, i+arg.size): mask[j] = 'N'
	
	seq = ''.join(mask)
	print('>', defline, sep='')
	for i in range(0, len(seq), 60):
		print(seq[i:i+60])
