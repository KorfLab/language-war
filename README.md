# language-war

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

## Discussion

### Python

Pros:

- It is the most popular language
- Most students already know it

Cons:

- Python is slow and I want this codebase to be fast
- This codebase isn't designed for "students", but for me
- I don't like programming in Python

### C

Pros:

- C is the "mother language"
- C is very fast
- C is old-school and I'm a curmudgeon
- C has some good bioinformatics libraries already
- Some of my favorite programs are written in C
- Most of my programming heros program in C

Cons:

- C has memory safety issues (which aren't a big deal for this project)

### Rust

Pros:

- It's a memory-safe systems language
- It's replacing C

Cons:

- There is much to learn
- Not much previous bioinformatics to lean on

### Go

Pros:

- It's pretty fast
- It's easy to develop in

Cons:

- It's not quite as fast as C
- The garbage collector adds weight

### Zig

Pros:

- Seems like "a better C"
- There is much to learn

Cons:

- A little niche
- Not much previous bioinformatics to lean on


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

Status:

- [x] `dust`
- [x] `kmers`
- [ ] `genotype` working, but not threaded and output not pretty
- [ ] `exons`
- [ ] `params`


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

## Rust

Status: major feature complete. FASTA iterator could use SIMD acceleration for
parsing. Extra HMM implementation is mostly complete.

- [x] `fasta` iterator
- [x] `exons` extraction from SQLite 3
- [x] `params` de/serialize (with pretty printing)
  - **Remarks:** as a strongly-typed language, Rust inherently have a different
    model to do data de/serialization. It is possible to not use a schema and
    instead work with raw values, but it is extremely-error prone. This should
    have been implemented in the library. However, I did not have a well-defined
    schema for HMM params yet, so the current implementation is in the binary
    with a best-effort guessed schema from test data.
- [x] kmer counter
- [x] genotyping simulator
- [x] dust filter

Extras:

- [ ] Hidden Markov Model
  - [x] Structural Modeling
  - [x] Viterbi Algorithm
  - [ ] Parse Strctural JSON Parameters
  - [ ] Parse Non-Structural JSON Parameters

The Rust implementation ties together several crates, as Rust ecosystem does
not seem to have bioinformatic libraries just yet. The SQLite interaction uses
the awesome `rusqlite` crate. The JSON parser is `serde` and `serde-json`.

The library resides under the root of `src/` directory. In the `bin/`
subdirectory, each file gets compiled into a separate binary (in a separate
crate). Cargo is the official way to build Rust programs and manage deps.

```tree
rust/
  Cargo.lock
  Cargo.toml
  run.sh
  src/
    bin/
      dust.rs
      exons.rs
      genotypes.rs
      kmers.rs
      occassionally_dishonest_casino.rs
      params.rs
    collections/
      vec2.rs
    collections.rs
    dust.rs
    fasta.rs
    genotype.rs
    hidden_markov.rs
    kmer.rs
    lib.rs
    sequence.rs
```

## Nim

Status: done

- [x] `fasta` iterator
- [x] `exons` extraction from SQLite 3
- [x] `params` de/serialize (with pretty printing)
- [x] kmer counter
- [x] genotyping simulator
- [x] dust filter

The Nim implementation also uses several packages. Notably, Nim does not have
good streaming gz decompressor support, so I choose to use `zippy` and
decompress in-memory. This is not good for very large fasta files.

Additionally, Nim's default threading is quite difficult to use. Maleglobia is
a good runtime which provides a easy interface, but if you need Channels between
threads, they become tricky because of Nim's reference counting.

The library resides under the `language_war` directory. Binaries are in the root
directory, and specified to be built in `language_war.nimble`. Preferred build
system is Nim's own `nimble`, but `atlas` also should work.

```tree
nim/
  language_war.nimble
  nimble.lock
  run.sh
  xmake.lua
  language_war/
    collections.nim
    dust.nim
    fasta.nim
    genotype.nim
    hidden_markov.nim
    kmer.nim
    sequence.nim
  dust.nim
  exons.nim
  genotype.nim
  kmers.nim
  params.nim
```

## next languages...
