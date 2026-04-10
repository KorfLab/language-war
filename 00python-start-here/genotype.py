import argparse
import math
import random
import sys

parser = argparse.ArgumentParser()
parser.add_argument('iterations', type=float, metavar='<float>',
	help='simulations to perform (e.g. 1e6')
parser.add_argument('depth', type=int, metavar='<int>',
	help='sequencing depth (e.g. 10)')
parser.add_argument('--output', '-o', metavar='<file>',
	help='output file [default is stdout]')
parser.add_argument('--err_rate', '-e', type=float, default=0.1, 
	metavar='<float>', help='sequencing error rate [%(default).2f]')
parser.add_argument('--seed', type=int, metavar='<int>',
	help='use a specific random seed [default is random]')
arg = parser.parse_args()
if arg.seed: random.seed(arg.seed)


hom_count = {}
for _ in range(int(arg.iterations)):
	# hom: mom is A (differences are sequencing error)
	hom = {'A':0, 'C':0, 'G':0, 'T':0}
	for i in range(arg.depth):
		if random.random() < arg.err_rate:
			nt = random.choice('CGT')
			hom[nt] += 1
		else:
			hom['A'] += 1
	vals = list(hom.values())
	vals.sort(reverse=True)
	sig = f'{vals[0]}.{vals[1]}.{vals[2]}.{vals[3]}'
	if sig not in hom_count: hom_count[sig] = 0
	hom_count[sig] += 1

het_count = {}
for _ in range(int(arg.iterations)):
	# het: mom is A, dad is T
	het = {'A':0, 'C':0, 'G':0, 'T':0}
	for i in range(arg.depth):
		if random.random() < 0.5:
			nt = 'A' # mom
			if random.random() < arg.err_rate: nt = random.choice('CGT')
		else:
			nt = 'T' # dad
			if random.random() < arg.err_rate: nt = random.choice('ACG')
		het[nt] += 1
	vals = list(het.values())
	vals.sort(reverse=True)
	sig = f'{vals[0]}.{vals[1]}.{vals[2]}.{vals[3]}'
	if sig not in het_count: het_count[sig] = 0
	het_count[sig] += 1

if arg.output: fp = open(arg.output, 'w')
else:          fp = sys.stdout

print('Counts', 'Hom', 'Het', 'P(hom)', sep='\t', file=fp)
for sig in sorted(list(hom_count or het_count), reverse=True):
	hom = hom_count[sig] if sig in hom_count else 0
	het = het_count[sig] if sig in het_count else 0
	print(sig, hom, het, hom / (hom+het), sep='\t', file=fp)
