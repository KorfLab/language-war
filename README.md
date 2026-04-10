language-war
============

Comparing various languages for building bioinformatics applications

## Task

Write the following in each language:

- FASTA iterator in a shared library
- dust filter: reads FASTA, outputs masked sequence
- kmer counter: reads FASTA, reports kmer frequencies
- genotype by sequencing simulator: reports probabilities of patterns
- sqlite interaction to produce mRNAs from a FASTA/GFF3 database
- JSON interaction to read a parameter file

For inspiration, see the `00python3-start-here` solution.

## Manifest

- `README.md` this document
- `data` files used for testing
	- `ce1pct.fa.gz` 1% of the C. elegans genome in FASTA
	- `ce1pct.gff3.gz` 1% of the C. elegans genome in GFF3 (genes, RNA-seq)
	- `ce.db` a sqlite database of the files above
	- `hmm.json` a mock HMM parameter file
- `src` each directory is named with a language and other tags
	- `00python-start-here` pure python
	- `perl-oldschool` for historical reaons
	- `c-klib-amal-jsmn` a C solution based on klib

## 00python3-start-here

To be filled in by Ian

## c-klib

To be filled in by Ian

The C implementation uses Klib at its core, which is a great library for
C-based bioinformatics work. It is used in htslib, minimap2, etc. The SQLITE
interaction uses the sqlite amalgamation header. The JSON parser is JSMN.

```
Makefile
README.md
lib/
	include/
		jsmn.h
		khash.h
		krng.h
		kseq.h
		sqlite3.h
programs/
	dust/
		Makefile
		main.c
	kmer/
		Makefile
		main.c
	genotype/
		Makefile
		main.c
	sql/
		Makefile
		main.c
```

## perl-oldschool

To be filled in by Ian

## next languages...
