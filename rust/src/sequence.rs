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

pub fn entropy(a: isize, c: isize, g: isize, t: isize) -> f64 {
    let total = (a + c + g + t) as f64;
    if total == 0.0 {
        return 0.0;
    }

    let mut ent = 0.0;
    for &count in &[a, c, g, t] {
        if count > 0 {
            let p = count as f64 / total;
            ent -= p * p.log2();
        }
    }
    ent
}
