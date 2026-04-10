language-war
============

Comparing various languages for building bioinformatics applications

## Intent

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
- `00python-start-here` pure python to inspire other solutions
- `c-klib` a C solution based partly on klib

## 00python3-start-here

Other languages should have programs with similar names and produce identical
output. There should be a `run.sh` that builds and runs the programs.

- `run.sh` use this to run all programs (then `rm *.out`)
- `mylib.py` contains the FASTA iterator
- `dust.py` masks low complexity sequence
- `kmers.py` counts kmers
- `genotype.py` simulates genotyping by sequencing
- `exons.py` reports exon sequences
- `params.py` reads a JSON


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
