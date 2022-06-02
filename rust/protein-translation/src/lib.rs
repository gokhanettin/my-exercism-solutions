use std::collections::HashMap;

const STOP: &'static str = "stop codon";

pub struct CodonsInfo<'a> {
    info: HashMap<&'a [u8], &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.info.get(codon.as_bytes()).and_then(|&s| Some(s))
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        rna.as_bytes()
            .chunks(3)
            .map(|codon| self.info.get(codon).and_then(|&s| Some(s)))
            .take_while(|&name| name != Some(STOP))
            .collect()
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo {
        info: pairs
            .into_iter()
            .map(|(codon, name)| (codon.as_bytes(), name))
            .collect(),
    }
}
