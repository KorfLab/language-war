language-war
============

Comparing various languages for building bioinformatics applications

## Task

Write the following in each language:

- FASTA iterator in a shared library
- dust filter: reads FASTA, outputs masked sequence
- kmer counter: reads FASTA, reports kmer frequencies
- genotyping simulator: reports heterozygous probability column counts
- get exon sequences from a FASTA/GFF3 sqlite database
- read a parameter file in JSON

For inspiration, see the `00python3-start-here` solution.

Which languages?

- Modern high performance: D, Go, Rust, Zig
- Less common but maybe interested: Crystal, Mojo, Nim, V
- Probably not interested
	- JIT-based: C#, Java, Javascript, Julia, LuaJIT
	- Interpreted: Lua, PHP, Raku, Ruby

## Manifest

- `README.md` this document
- `data` files used for testing
	- `ce1pct.fa.gz` 1% of the C. elegans genome in FASTA
	- `ce1pct.gff3.gz` 1% of the C. elegans genome in GFF3 (for ref, not used)
	- `ce.db` a sqlite database of the files above
	- `hmm.json` a simple HMM parameter file
- `src` each directory is named with a language and other tags
	- `00python-start-here` pure python
	- `perl-oldschool` for historical perspective
	- `c-klib` a C solution based partly on klib

## 00python3-start-here

Other project should look sort of similar to this. There is no need to describe
them in great detail since they are supposed to look like the python version.

- `run.sh` use this to run all programs (then `rm *.out`)
- `mylib.py` the shared library for FASTA files
- `dust.py` nucleotide complexity filter
- `kmers.py` kmer frequencies
- `genotype.py` heterozygous probabilities
- `exons.py` exon sequences
- `params.py` hmm parameters


## c-klib

To be filled in by Ian

The C implementation uses Klib at its core, which is a great library for
C-based bioinformatics work. It is used in htslib, minimap2, etc. The SQLITE
interaction uses the sqlite amalgamation header. The JSON parser is JSMN.

The programs all have their own directory with a `Makefile` and a `main.c`.

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
	kmers/
	genotypes/
	exons/
	params/
```

## next languages...
