FASTA=../data/ce1pct.fa.gz
DB=../data/ce.db
JSON=../data/hmm.json

make

dust/dust $FASTA > dust.out
kmers/kmers $FASTA 5 > kmers.out
#genotype/genotype 1e6 6 > genotype.out
#exons/exons $DB > exons.out
#params/params $JSON > params.out
