#! /bin/bash
set -e

FASTA="../data/ce1pct.fa.gz"
DB="../data/ce.db"
JSON="../data/hmm.json"

NIM_FLAGS="-d:release"

echo "=== Building Nim binaries ==="
nimble build "$NIM_FLAGS"

echo ""
echo "=== Running Nim tools ==="
./bin/dust --size=20 --entropy=1.4 "$FASTA" > dust.out
./bin/kmers "$FASTA" 5 > kmers.out
./bin/kmers --anti "$FASTA" 5 > kmers_anti.out
./bin/genotype 1e6 8 --threads=8 > genotype.out
./bin/exons --db="$DB" > exons.out
#./bin/params "$JSON" > params.out

echo ""
echo "=== Done ==="
echo "Output files: dust.out kmers.out kmers_anti.out genotype.out exons.out params.out"
