#[derive(Debug, PartialEq)]
pub struct Dna(String);

#[derive(Debug, PartialEq)]
pub struct Rna(String);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        if let Some(index) = dna.bytes().position(|b| !b"GCTA".contains(&b)) {
            Err(index)
        } else {
            Ok(Dna(dna.to_string()))
        }
    }

    pub fn into_rna(self) -> Rna {
        let Dna(ref dna) = self;

        Rna(dna
            .chars()
            .map(|c| match c {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                x => x,
            })
            .collect())
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        if let Some(index) = rna.bytes().position(|b| !b"CGAU".contains(&b)) {
            Err(index)
        } else {
            Ok(Rna(rna.to_string()))
        }
    }
}
