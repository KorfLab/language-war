use clap::Parser;
use korflab_language_war::sequence;

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
}

fn main() {
    let args = Args::parse();
}
