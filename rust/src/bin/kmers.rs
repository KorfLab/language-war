use clap::Parser;
use korflab_language_war::fasta::{FastaIter, FastaRecord};
use korflab_language_war::kmer::count_kmers;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to FASTA file
    fasta: String,
    /// k-mer size
    k: usize,
    #[arg(long)]
    /// count both strands
    anti: bool,
}

fn main() {
    let args = Args::parse();

    let fasta_iter = if args.fasta == "-" {
        FastaIter::from_stdin()
    } else if args.fasta.ends_with(".gz") {
        FastaIter::from_gz_file(args.fasta.as_ref()).expect("Failed to read gzipped FASTA file")
    } else {
        FastaIter::from_file(args.fasta.as_ref()).expect("Failed to read FASTA file")
    };

    let fasta_records: Vec<FastaRecord> = fasta_iter
        .map(|res| res.expect("Failed to parse FASTA record"))
        .collect();

    let kmer_counts = count_kmers(&fasta_records, args.k, args.anti);

    let total_kmers: usize = kmer_counts.values().sum();
    // we just use an owned slice of references, since we only need to sort by
    // k-mer string and don't need to modify the counts
    let mut kmer_pairs: Box<[(&String, &usize)]> = kmer_counts.iter().collect();
    kmer_pairs.sort_unstable();

    for (kmer, n) in kmer_pairs {
        println!("{}\t{}\t{}", kmer, n, *n as f64 / total_kmers as f64);
    }
}
