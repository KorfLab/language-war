use clap::Parser;
use korflab_language_war::fasta::{FastaIter, FastaRecord};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to FASTA file
    fasta: String,
    #[arg(long, short = 's', default_value_t = 20)]
    /// Window size
    size: usize,
    #[arg(long, short = 'e', default_value_t = 1.4)]
    /// Entropy threshold
    entropy: f64,
    #[arg(long)]
    /// Soft mask instead of hard masking with N
    lower: bool,
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

    let fasta_records = fasta_iter
        .map(|r| r.expect("Failed to read FASTA record"))
        .collect::<Vec<FastaRecord>>();

    let masked_fasta_records = fasta_records
        .iter()
        .map(|r| r.dust_masked(args.size, args.entropy, args.lower))
        .collect::<Vec<FastaRecord>>();

    for masked_fasta_record in masked_fasta_records {
        print!("{}", masked_fasta_record);
    }
}
