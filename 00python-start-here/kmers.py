import argparse
import mylib

parser = argparse.ArgumentParser(description='Kmer frequency reporter')
parser.add_argument('fasta', type=str, help='name of fasta file')
parser.add_argument('k', type=int, help='kmer size')
parser.add_argument('--anti', action='store_true', help='count both strands')
arg = parser.parse_args()

counts = {}
for defline, seq in mylib.readfasta(arg.fasta):
	for i in range(len(seq) - arg.k + 1):
		kmer = seq[i:i+arg.k]
		if kmer not in counts: counts[kmer] = 0
		counts[kmer] += 1
		if arg.anti:
			kmer = mylib.anti(kmer)
			if kmer not in counts: counts[kmer] = 0
			counts[kmer] += 1

total = sum(counts.values())
#if arg.anti: total *= 2
for kmer, n in sorted(counts.items()):
	print(kmer, n, n/total, sep='\t')
