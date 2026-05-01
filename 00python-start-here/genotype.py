import argparse
import collections
import concurrent.futures
import math
import random
import sys


def count_genotypes(arg):
	hom_count = {}
	for _ in range(int(arg.iterations/arg.threads)):
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
	for _ in range(int(arg.iterations/arg.threads)):
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
	return hom_count, het_count


if __name__ == '__main__':
	
	# cli
	parser = argparse.ArgumentParser()
	parser.add_argument('iterations', type=float, metavar='<float>',
		help='simulations to perform (e.g. 1e6')
	parser.add_argument('depth', type=int, metavar='<int>',
		help='sequencing depth (e.g. 10)')
	parser.add_argument('--threads', type=int, metavar='<int>', default=2,
		help='parallel threads [%(default)i]')
	parser.add_argument('--output', '-o', metavar='<file>',
		help='output file [default is stdout]')
	parser.add_argument('--err_rate', '-e', type=float, default=0.1, 
		metavar='<float>', help='sequencing error rate [%(default).2f]')
	arg = parser.parse_args()

	# run jobs
	with concurrent.futures.ProcessPoolExecutor() as executor:
		results = executor.map(count_genotypes, [arg]*arg.threads)

	# combine results
	comb_hom = collections.Counter()
	comb_het = collections.Counter()
	for hom, het in results:
		comb_hom.update(hom)
		comb_het.update(het)
	
	# output
	if arg.output: fp = open(arg.output, 'w')
	else:          fp = sys.stdout
	print('Counts', 'Hom', 'Het', 'P(hom)', sep='\t', file=fp)
	for sig in sorted(list(comb_hom | comb_het), reverse=True):
		hom = comb_hom[sig] if sig in comb_hom else 0
		het = comb_het[sig] if sig in comb_het else 0
		print(sig, hom, het, hom / (hom+het), sep='\t', file=fp)
	