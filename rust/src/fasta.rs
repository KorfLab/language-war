use std::{
    error::Error,
    io::{BufReader, Lines, prelude::*},
};
use flate2::bufread::GzDecoder;

use thiserror::Error;

type FileLines = Lines<BufReader<std::fs::File>>;
type GzFileLines = Lines<BufReader<GzDecoder<BufReader<std::fs::File>>>>;
type StringLines = Lines<BufReader<std::io::Cursor<String>>>;

pub struct FastaRecord {
    pub description: String,
    pub sequence: String,
}

pub struct FastaIter<I: Iterator<Item = Result<String, E>>, E: Error> {
    lines: I,
    pending_header: Option<String>,
    finished: bool,
}

#[derive(Debug, Error)]
pub enum FastaIterError<E: Error> {
    LineIterError(E),
}

fn sanitize_sequence(seq: &str) -> String {
    seq.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect::<String>()
}

impl<I, E> FastaIter<I, E>
where
    I: Iterator<Item = Result<String, E>>,
    E: Error,
{
    fn new(lines: I) -> Self {
        Self {
            lines,
            pending_header: None,
            finished: false,
        }
    }
}

impl FastaIter<FileLines, std::io::Error> {
    pub fn from_file(path: &str) -> std::io::Result<Self> {
        let file = std::fs::File::open(path)?;
        let reader = BufReader::new(file);
        Ok(Self::new(reader.lines()))
    }
}

impl FastaIter<GzFileLines, std::io::Error> {
    pub fn from_gz_file(path: &str) -> std::io::Result<Self> {
        let file = std::fs::File::open(path)?;
        let buf_file = BufReader::new(file);
        let gz_decoder = GzDecoder::new(buf_file);
        let buf_gz_decoder = BufReader::new(gz_decoder);
        Ok(Self::new(buf_gz_decoder.lines()))
    }
}

impl FastaIter<Lines<std::io::StdinLock<'_>>, std::io::Error> {
    pub fn from_stdin() -> Self {
        let stdin = std::io::stdin();
        Self::new(stdin.lines())
    }
}

impl FastaIter<Lines<BufReader<GzDecoder<std::io::StdinLock<'_>>>>, std::io::Error> {
    pub fn from_gz_stdin() -> Self {
        let stdin = std::io::stdin();
        let gz_decoder = GzDecoder::new(stdin.lock());
        let buf_gz_decoder = BufReader::new(gz_decoder);
        Self::new(buf_gz_decoder.lines())
    }
}

impl FastaIter<StringLines, std::io::Error> {
    pub fn from_string(data: String) -> Self {
        let cursor = std::io::Cursor::new(data);
        Self::new(BufReader::new(cursor).lines())
    }
}

impl<I, E> Iterator for FastaIter<I, E>
where
    I: Iterator<Item = Result<String, E>>,
    E: Error,
{
    type Item = Result<FastaRecord, FastaIterError<E>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let mut seq_desc: Option<String> = None;
        let mut sequence = String::new();

        if let Some(header) = self.pending_header.take() {
            seq_desc = Some(header);
        }

        for line_result in &mut self.lines {
            match line_result {
                Ok(line) => {
                    if let Some(description) = line.strip_prefix('>') {
                        if seq_desc.is_none() {
                            seq_desc = Some(description.to_string());
                        } else {
                            self.pending_header = Some(description.to_string());
                            break;
                        }
                    } else if line.starts_with(';') {
                        continue; // Skip comment lines
                    } else {
                        sequence.push_str(&sanitize_sequence(&line));
                    }
                }
                Err(e) => return Some(Err(FastaIterError::LineIterError(e))),
            }
        }

        if seq_desc.is_none() && sequence.is_empty() {
            self.finished = true;
            return None;
        }

        Some(Ok(FastaRecord {
            description: seq_desc.unwrap_or_default(),
            sequence,
        }))
    }
}
