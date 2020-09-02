use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

use serde::{Deserialize, Serialize};

use anyhow::Result;
use fst::Set;
use memmap::Mmap;

use bio::io::fasta;

use crate::fst_utils;

#[derive(Serialize, Deserialize, Debug)]
struct Sequence {
    id: String,
    description: Option<String>,
    sequence: String,
}

pub enum Selection {
    InIdSet,
    NotInIdSet,
}

impl From<Sequence> for fasta::Record {
    fn from(entry: Sequence) -> fasta::Record {
        return fasta::Record::with_attrs(
            &entry.id,
            entry.description.as_ref().map(String::as_ref),
            &entry.sequence.as_bytes(),
        );
    }
}

impl Selection {
    pub fn selected(&self, set: &Set<Mmap>, id: &String) -> bool {
        match self {
            Self::InIdSet => set.contains(id),
            Self::NotInIdSet => !set.contains(id),
        }
    }
}

pub fn write_selected(
    ids: &Path,
    sequences: &Path,
    output: &Path,
    selection: &Selection,
) -> Result<()> {
    let file = File::open(sequences)?;
    let file = BufReader::new(file);
    let known = fst_utils::load(ids)?;
    let writer = File::create(output)?;
    for line in file.lines() {
        let line = line?.replace("\\\\", "\\");
        let entry: Sequence = serde_json::from_str(&line)?;
        if selection.selected(&known, &entry.id) {
            serde_json::to_writer(&writer, &entry)?;
        }
    }
    Ok(())
}

pub fn write_fasta(sequences: &Path, output: &Path) -> Result<()> {
    let file = File::open(sequences)?;
    let file = BufReader::new(file);
    let mut writer = fasta::Writer::new(File::create(output)?);
    for line in file.lines() {
        let line = line?.replace("\\\\", "\\");
        let entry: Sequence = serde_json::from_str(&line)?;
        writer.write_record(&entry.into())?;
    }
    Ok(())
}
