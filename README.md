language-war
============

Comparing various languages for building bioinformatics applications

## Intent

Using `00python3-start-here` as a template for other languages, write the
following solutions:

- FASTA iterator in a shared library
- dust filter: reads FASTA, outputs masked sequence
- kmer counter: reads FASTA, reports kmer frequencies
- genotyping simulator: reports genotype probabilities given nt counts
- get exon sequences from a FASTA/GFF3 sqlite database
- read a parameter file in JSON

Which languages?

- Classic systems-level: C
- Modern systems-level: Go, Rust, Zig
- Less common but maybe interested: Crystal, D, Mojo, Nim, V
- Probably not interested
	- Compiled: C++, Free Pascal
	- JIT-based: C#, F#, Java, Javascript, Julia, LuaJIT, Scala
	- Interpreted: Lua, PHP, Raku, Ruby

## Manifest

- `README.md` this document
- `data` files used for testing
	- `ce1pct.fa.gz` 1% of the C. elegans genome in FASTA
	- `ce1pct.gff3.gz` 1% of the C. elegans genome in GFF3 (for ref, not used)
	- `ce.db` a sqlite database of the files above
	- `hmm.json` a simple HMM parameter file
- `00python-start-here` pure Python to inspire other solutions
- `c-klib` a C solution based partly on klib

## 00python3-start-here

Status: acceptable, requires version >= 3.10

Other languages should have programs with similar names and produce nearly
identical output. There should be a `run.sh` that builds and runs the programs.

- `run.sh` use this to run all programs (then `rm *.out` later)
- `mylib.py` contains the FASTA iterator and `anti()` function
- `dust.py` masks low complexity sequence, optionally lowercase
- `kmers.py` counts kmers, optionally double-stranded
- `genotype.py` simulates genotyping by sequencing, threaded
- `exons.py` reports exon sequences
- `params.py` reads a JSON and spits it back out reformatted


## c-klib

Status: `dust` complete

The C implementation uses Klib at its core, which is a great library for
C-based bioinformatics work. It is used in htslib, minimap2, etc. The sqlite
interaction uses the sqlite amalgamation. The JSON parser is JSMN.

The programs all have their own directory with a `Makefile` and a `main.c`.

```
Makefile
lib/
	include/
		jsmn.h
		khash.h krng.h kseq.h kvec.h
		mylib.h
		sqlite3.h
	src/
		mylib.c
		sqlite3.c
dust/
	Makefile
	main.c
kmers/
genotypes/
exons/
params/
```

## next languages...
