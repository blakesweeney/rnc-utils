use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use std::io::{BufReader, BufWriter};

use anyhow::Result;

use fst::{Set, SetBuilder};

use memmap::Mmap;


pub fn load(filename: &Path) -> Result<Set<Mmap>> {
    let mmap = unsafe { Mmap::map(&File::open(filename)?)? };
    return Set::new(mmap).map_err(From::from);
}

pub fn build(filename: &Path, output: &Path) -> Result<()> {
    let file = BufReader::new(File::open(filename)?);
    let writer = BufWriter::new(File::create(output)?);
    let mut builder = SetBuilder::new(writer)?;
    for line in file.lines() {
        builder.insert(line?.trim())?;
    }
    builder.finish()?;
    return Ok(());
}
