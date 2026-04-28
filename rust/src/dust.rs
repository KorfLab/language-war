use crate::{fasta::FastaRecord, sequence};

pub fn mask(seq: &str, window_size: usize, entropy_threshold: f64, soft_mask: bool) -> String {
    let bytes = seq.as_bytes();
    let mut masked_seq = bytes.to_vec();
    let mut a;
    let mut c;
    let mut g;
    let mut t;

    // Initialize the first window
    let mut count_table = [0; 128];
    for &byte in &bytes[0..window_size] {
        count_table[byte as usize] += 1;
    }

    a = count_table['A' as usize];
    c = count_table['C' as usize];
    g = count_table['G' as usize];
    t = count_table['T' as usize];

    if sequence::entropy(a, c, g, t) < entropy_threshold {
        if soft_mask {
            for nucleotide in masked_seq.iter_mut().take(window_size) {
                *nucleotide = nucleotide.to_ascii_lowercase();
            }
        } else {
            for nucleotide in masked_seq.iter_mut().take(window_size) {
                *nucleotide = b'N';
            }
        }
    }

    // Slide the window through the rest of the sequence
    for i in 1..(bytes.len() - window_size + 1) {
        let off = bytes[i - 1] as usize;
        let on = bytes[i + window_size - 1] as usize;

        count_table[off] -= 1;
        count_table[on] += 1;

        a = count_table['A' as usize];
        c = count_table['C' as usize];
        g = count_table['G' as usize];
        t = count_table['T' as usize];

        if sequence::entropy(a, c, g, t) < entropy_threshold {
            if soft_mask {
                for nucleotide in masked_seq.iter_mut().skip(i).take(window_size) {
                    *nucleotide = nucleotide.to_ascii_lowercase();
                }
            } else {
                for nucleotide in masked_seq.iter_mut().skip(i).take(window_size) {
                    *nucleotide = b'N';
                }
            }
        }
    }

    String::from_utf8(masked_seq).expect("Invalid UTF-8")
}

impl FastaRecord {
    pub fn dust_masked(&self, window_size: usize, entropy_threshold: f64, soft_mask: bool) -> Self {
        let sequence = mask(self.sequence(), window_size, entropy_threshold, soft_mask);
        FastaRecord::new(self.description().to_string(), sequence)
    }
}
