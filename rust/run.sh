#! /bin/bash
set -e

FASTA=../data/ce1pct.fa.gz
DB=../data/ce.db
JSON=../data/hmm.json

#cargo run --bin dust --release -- $FASTA > dust.out
#cargo run --bin kmers --release -- $FASTA 5 > kmers.out
#cargo run --bin genotype --release -- 1e6 6 > genotype.out
cargo run --bin exons --release -- --db $DB > exons.out
#cargo run --bin params --release -- $JSON > params.out
