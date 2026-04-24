use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use clap::Parser;
use korflab_language_war::genotype;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of iterations to simulate
    iterations: f64,
    /// Depth of sequencing
    depth: usize,
    #[arg(long, short = 'e', default_value_t = 0.1)]
    /// Sequencing error rate (0.0 - 1.0)
    err_rate: f64,
    #[arg(long, default_value_t = 2)]
    /// Number of threads to use for simulation
    threads: usize,
    #[arg(long, short = 'o')]
    /// Output file for results
    output: Option<PathBuf>,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let mut output: Box<dyn Write> = match args.output {
        Some(path) => {
            Box::new(BufWriter::new(File::create(&path).unwrap_or_else(|_| {
                panic!("Failed to create output file {}", path.display())
            })))
        }
        None => Box::new(std::io::stdout().lock()),
    };

    let (homo_count, hetero_count) = genotype::count_genotypes_parallel(
        args.iterations as usize,
        args.depth,
        args.err_rate,
        args.threads,
    );

    writeln!(output, "Counts\tHom\tHet\tP(hom)")?;

    let sigs: BTreeSet<_> = homo_count.keys().chain(hetero_count.keys()).collect();

    for sig in sigs.iter().rev() {
        let hom = homo_count.get(*sig).copied().unwrap_or(0);
        let het = hetero_count.get(*sig).copied().unwrap_or(0);
        let total = hom + het;
        let p_hom = if total > 0 {
            hom as f64 / total as f64
        } else {
            0.0
        };
        writeln!(output, "{}\t{}\t{}\t{}", sig, hom, het, p_hom)?;
    }

    Ok(())
}
