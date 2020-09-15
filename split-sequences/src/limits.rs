use crate::chunks::Chunks;

pub struct Limits {
    max_sequences: Option<u64>,
    max_nucleotides: Option<u64>,
    max_file_size: Option<u64>,
}

impl Limits {
    pub fn new(
        max_sequences: Option<u64>,
        max_nucleotides: Option<u64>,
        max_file_size: Option<u64>,
    ) -> Self {
        Self {
            max_sequences,
            max_nucleotides,
            max_file_size,
        }
    }

    pub fn too_large(&self, chunk: &Chunks) -> bool {
        let seq_limit = self.max_sequences.map(|s| s > chunk.sequence_count());
        let nt_limit = self.max_nucleotides.map(|n| n > chunk.nucleotide_count());
        let fs_limit = self.max_file_size.map(|n| n >= chunk.file_size());
        seq_limit.or(nt_limit).or(fs_limit).unwrap_or(false)
    }
}
