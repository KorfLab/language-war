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

    let sigs: BTreeSet<_> = homo_count.keys().chain(hetero_count.keys()).collect();

    // Compute column widths for alignment
    let mut sig_width = 3usize;
    let mut hom_width = 3usize;
    let mut het_width = 3usize;
    const PHOM_WIDTH: usize = 8;

    for sig in &sigs {
        sig_width = sig_width.max(sig.len());
        let hom = homo_count.get(*sig).copied().unwrap_or(0);
        let het = hetero_count.get(*sig).copied().unwrap_or(0);
        hom_width = hom_width.max(hom.to_string().len());
        het_width = het_width.max(het.to_string().len());
    }

    writeln!(output,
        "{:<sig_width$} {:>hom_width$} {:>het_width$} {:>PHOM_WIDTH$}",
        "Sig", "Hom", "Het", "P(hom)"
    )?;
    writeln!(output,
        "{} {} {} {}",
        "-".repeat(sig_width),
        "-".repeat(hom_width),
        "-".repeat(het_width),
        "-".repeat(PHOM_WIDTH)
    )?;

    for sig in sigs.iter().rev() {
        let hom = homo_count.get(*sig).copied().unwrap_or(0);
        let het = hetero_count.get(*sig).copied().unwrap_or(0);
        let total = hom + het;
        let p_hom = if total > 0 {
            hom as f64 / total as f64
        } else {
            0.0
        };
        writeln!(output,
            "{:<sig_width$} {:>hom_width$} {:>het_width$} {:>PHOM_WIDTH$.4}",
            sig, hom, het, p_hom
        )?;
    }

    Ok(())
}
