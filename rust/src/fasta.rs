use flate2::bufread::GzDecoder;
use std::{
    io::{BufReader, prelude::*},
};
use thiserror::Error;

pub struct FastaRecord {
    pub description: String,
    pub sequence: String,
}

fn complement_base(base: char) -> char {
    match base {
        'A' => 'T',
        'C' => 'G',
        'G' => 'C',
        'T' => 'A',
        'R' => 'Y',
        'Y' => 'R',
        'M' => 'K',
        'K' => 'M',
        'W' => 'W',
        'S' => 'S',
        'B' => 'V',
        'D' => 'H',
        'H' => 'D',
        'V' => 'B',
        _ => base,
    }
}

impl FastaRecord {
    pub fn reverse_complement(&self) -> String {
        self.sequence.chars().rev().map(complement_base).collect()
    }
}

pub struct FastaIter {
    buf_read: Box<dyn BufRead>,
    line_buffer: String,
    pending_header: Option<String>,
    finished: bool,
}

#[derive(Error, Debug)]
pub enum FastaIterError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

fn append_sanitized_sequence(dest: &mut String, src: &str) {
    dest.extend(src.chars().filter(|c| c.is_ascii_alphabetic()))
}

impl FastaIter {
    fn new(buf_read: Box<dyn BufRead>) -> Self {
        Self {
            buf_read,
            // lines in FASTA file typically is 60-80 chars long
            line_buffer: String::with_capacity(96),
            pending_header: None,
            finished: false,
        }
    }
}

impl FastaIter {
    pub fn from_file(path: &str) -> std::io::Result<Self> {
        let file = std::fs::File::open(path)?;
        let reader = BufReader::new(file);
        Ok(Self::new(Box::new(reader)))
    }

    pub fn from_gz_file(path: &str) -> std::io::Result<Self> {
        let file = std::fs::File::open(path)?;
        let buf_file = BufReader::new(file);
        let gz_decoder = GzDecoder::new(buf_file);
        let buf_gz_decoder = BufReader::new(gz_decoder);
        Ok(Self::new(Box::new(buf_gz_decoder)))
    }
    
    pub fn from_stdin() -> Self {
        let stdin = std::io::stdin();
        Self::new(Box::new(stdin.lock()))
    }
    
    pub fn from_gz_stdin() -> Self {
        let stdin = std::io::stdin();
        let gz_decoder = GzDecoder::new(stdin.lock());
        let buf_gz_decoder = BufReader::new(gz_decoder);
        Self::new(Box::new(buf_gz_decoder))
    }
    
    pub fn from_string(data: String) -> Self {
        let cursor = std::io::Cursor::new(data);
        Self::new(Box::new(cursor))
    }
}

impl Iterator for FastaIter {
    type Item = Result<FastaRecord, FastaIterError>;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let mut seq_desc: Option<String> = None;
        let mut sequence = String::with_capacity(1024);

        if let Some(header) = self.pending_header.take() {
            seq_desc = Some(header);
        }

        loop {
            self.line_buffer.clear();
            match self.buf_read.read_line(&mut self.line_buffer) {
                Ok(0) => {
                    self.finished = true;
                    break;
                }
                Ok(_) => {
                    let line = self.line_buffer.trim_end();
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
                        append_sanitized_sequence(&mut sequence, line);
                    }
                }
                Err(e) => {
                    self.finished = true;
                    return Some(Err(FastaIterError::Io(e)));
                }
            }
        }

        if seq_desc.is_none() && sequence.is_empty() {
            return None;
        }
        
        if self.line_buffer.capacity() > 1024 {
            self.line_buffer.shrink_to(96);
        }

        Some(Ok(FastaRecord {
            description: seq_desc.unwrap_or_default(),
            sequence,
        }))
    }
}
