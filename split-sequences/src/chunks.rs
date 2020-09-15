use bio::io::fasta;

use crate::limits::Limits;

pub struct Chunks {
    sequences: u64,
    nucleotides: u64,
    file_size: u64,
    index: u64,
}

impl Chunks {
    pub fn new() -> Self {
        Self {
            sequences: 0,
            nucleotides: 0,
            file_size: 0,
            index: 0,
        }
    }

    pub fn sequence_count(&self) -> u64 {
        self.sequences
    }

    pub fn nucleotide_count(&self) -> u64 {
        self.nucleotides
    }

    pub fn file_size(&self) -> u64 {
        self.file_size
    }

    pub fn key(&self) -> u64 {
        self.index
    }

    pub fn add_record(&mut self, record: &fasta::Record, limit: &Limits) -> () {
        let nucleotides: u64 = record.seq().len() as u64;
        let mut file_size = 1;
        file_size += record.id().as_bytes().len();
        file_size += record.desc().map(|d| d.as_bytes().len()).unwrap_or(0);
        file_size += record.seq().len();

        self.sequences += 1;
        self.nucleotides += nucleotides;
        self.file_size += file_size as u64;

        if limit.too_large(&self) {
            self.sequences = 1;
            self.nucleotides = nucleotides;
            self.file_size = file_size as u64;
            self.index += 1;
        }
    }

}
