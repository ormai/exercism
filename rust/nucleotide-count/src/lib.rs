use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !"GATC".contains(nucleotide) {
        return Err(nucleotide);
    }

    let mut count = 0;
    for c in dna.chars() {
        if !"GATC".contains(c) {
            return Err(c);
        }
        if c == nucleotide {
            count += 1;
        }
    }
    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    Ok(HashMap::from([
        ('G', count('G', dna)?),
        ('A', count('A', dna)?),
        ('T', count('T', dna)?),
        ('C', count('C', dna)?),
    ]))
}
