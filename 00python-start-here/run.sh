FASTA=../data/ce1pct.fa.gz
DB=../data/ce.db
JSON=../data/hmm.json

python3 dust.py $FASTA > dust.out
python3 kmers.py $FASTA 5 > kmers.out
python3 kmers.py --anti $FASTA 5 > kmers_anti.out
python3 genotype.py 1e6 6 > genotype.out
python3 exons.py $DB > exons.out
python3 params.py $JSON > params.out
