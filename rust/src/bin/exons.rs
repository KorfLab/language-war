use rusqlite::named_params;
use std::{fs, path::PathBuf, process::exit};

use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
/// Retrieve exon sequence from SQLite DB
struct Args {
    #[arg(long)]
    /// Path to GFF3 SQLite DB
    db: PathBuf,
}

fn main() {
    let args = Args::parse();

    let sqlite_conn = match fs::exists(&args.db) {
        Err(err) => {
            eprintln!("Error checking DB file: {err}");
            exit(1);
        }
        Ok(false) => {
            eprintln!("DB file does not exist: {}", args.db.display());
            exit(1);
        }
        Ok(true) => rusqlite::Connection::open(&args.db).unwrap_or_else(|err| {
            eprintln!("Error opening DB file: {err}");
            exit(1);
        }),
    };

    let exon_query = "SELECT seqid, beg, end, strand FROM feature WHERE type = 'exon'".to_string();

    let mut exon_stmt = sqlite_conn.prepare(&exon_query).unwrap_or_else(|err| {
        eprintln!("Error preparing exon query: {err}");
        exit(1);
    });

    let exon_rows = exon_stmt
        .query_map([], |exon_row| {
            let exon_info: (String, isize, isize, String) =
                exon_row.try_into().unwrap_or_else(|err| {
                    eprintln!("Error reading exon row: {err}");
                    exit(1);
                });
            Ok(exon_info)
        })
        .unwrap_or_else(|err| {
            eprintln!("Error executing exon query: {err}");
            exit(1);
        });

    for mapped_exon_row in exon_rows {
        let (seqid, start, end, strand) = mapped_exon_row.unwrap_or_else(|err| {
            eprintln!("Error reading mapped exon row: {err}");
            exit(1);
        });

        let offset = start;
        let length = end - start + 1;

        let seq_query =
            "SELECT substr(seq, :offset, :length) from sequence WHERE seqid = :seqid".to_string();

        sqlite_conn
            .query_row(
                &seq_query,
                named_params! { ":offset": offset, ":length": length, ":seqid": seqid },
                |row| {
                    let seq: String = row.get(0).unwrap_or_else(|err| {
                        eprintln!("Error reading sequence row: {err}");
                        exit(1);
                    });

                    println!("{}\t{}\t{}\t{}\t{}", seqid, start, end, strand, seq);
                    Ok(())
                },
            )
            .unwrap_or_else(|err| {
                eprintln!("Error executing sequence query: {err}");
                exit(1);
            });
    }
}
