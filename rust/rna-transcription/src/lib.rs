#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    strand: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    strand: String,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        for (i, c) in dna.chars().enumerate() {
            if !"ACGT".contains(c) {
                return Err(i);
            }
        }
        Ok(Self {
            strand: String::from(dna),
        })
    }

    pub fn into_rna(self) -> Rna {
        Rna {
            strand: self
                .strand
                .chars()
                .map(|c| match c {
                    'G' => 'C',
                    'C' => 'G',
                    'T' => 'A',
                    'A' => 'U',
                    _ => panic!(""),
                })
                .collect(),
        }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        for (i, c) in rna.chars().enumerate() {
            if !"ACGU".contains(c) {
                return Err(i);
            }
        }
        Ok(Self {
            strand: String::from(rna),
        })
    }
}
