#! /bin/bash
set -e

FASTA="../data/ce1pct.fa.gz"
DB="../data/ce.db"
JSON="../data/hmm.json"

NIM_FLAGS="-d:release --outdir:."

echo "=== Building Nim binaries ==="
nim c $NIM_FLAGS src/dust.nim
nim c $NIM_FLAGS src/kmers.nim
nim c $NIM_FLAGS src/genotype.nim
nim c $NIM_FLAGS src/exons.nim
nim c $NIM_FLAGS src/params.nim

echo ""
echo "=== Running Nim tools ==="
./dust --size=20 --entropy=1.4 "$FASTA" > dust.out
./kmers "$FASTA" 5 > kmers.out
./kmers --anti "$FASTA" 5 > kmers_anti.out
./genotype 1e6 8 --threads 8 > genotype.out
./exons --db="$DB" > exons.out
./params "$JSON" > params.out

echo ""
echo "=== Done ==="
echo "Output files: dust.out kmers.out kmers_anti.out genotype.out exons.out params.out"
