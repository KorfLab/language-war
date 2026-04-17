pub fn complement_base(base: char) -> char {
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

pub fn reverse_complement(sequence: &str) -> String {
    sequence.chars().rev().map(complement_base).collect()
}
