use std::io::prelude::*;
use std::path::Path;

use serde::{Deserialize, Serialize};

use anyhow::Result;
use fst::Set;
use memmap::Mmap;

use bio::io::fasta;

use lazy_static::lazy_static;
use regex::Regex;

use crate::fst_utils;
use crate::utils;

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
        fasta::Record::with_attrs(
            &entry.id,
            entry.description.as_ref().map(String::as_ref),
            &entry.sequence.as_bytes(),
        )
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
    let input = utils::buf_reader(sequences)?;
    let known = fst_utils::load(ids)?;
    let mut writer = utils::buf_writer(output)?;
    for line in input.lines() {
        let line = line?.replace("\\\\", "\\");
        let entry: Sequence = serde_json::from_str(&line)?;
        if selection.selected(&known, &entry.id) {
            serde_json::to_writer(&mut writer, &entry)?;
        }
    }
    Ok(())
}

pub fn write_fasta(sequences: &Path, output: &Path) -> Result<()> {
    let input = utils::buf_reader(sequences)?;
    let out = utils::buf_writer(output)?;
    let mut writer = fasta::Writer::new(out);
    for line in input.lines() {
        let line = line?.replace("\\\\", "\\");
        let entry: Sequence = serde_json::from_str(&line)?;
        writer.write_record(&entry.into())?;
    }
    Ok(())
}

pub fn write_easel(sequences: &Path, output: &Path) -> Result<()> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[ACGTN]+$").unwrap();
    }
    let input = utils::buf_reader(sequences)?;
    let mut writer = utils::buf_writer(output)?;
    for line in input.lines() {
        let line = line?.replace("\\\\", "\\");
        let entry: Sequence = serde_json::from_str(&line)?;
        if RE.is_match(&entry.sequence) {
            serde_json::to_writer(&mut writer, &entry)?;
        }
    }
    Ok(())
}
