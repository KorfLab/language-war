use crate::fasta::FastaRecord;
use crate::sequence::reverse_complement;
use rustc_hash::FxBuildHasher;
use std::collections::HashMap;

pub fn count_kmers(
    seqs: &[FastaRecord],
    k: usize,
    anti: bool,
) -> HashMap<String, usize, FxBuildHasher> {
    let estimated_unique_kmers = 4_usize.pow(k.try_into().unwrap_or(0));

    let mut counts = HashMap::with_capacity_and_hasher(estimated_unique_kmers, FxBuildHasher);

    for seq in seqs {
        let seq_len = seq.sequence.len();

        for i in 0..=seq_len - k {
            let kmer = &seq.sequence[i..i + k];
            *counts.entry(kmer.to_string()).or_insert(0) += 1;

            if anti {
                let rev_comp = reverse_complement(kmer);
                *counts.entry(rev_comp).or_insert(0) += 1;
            }
        }
    }

    counts
}
