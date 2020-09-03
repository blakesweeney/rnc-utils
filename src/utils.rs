use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use anyhow::Result;

pub fn buf_reader(filename: &Path) -> Result<Box<dyn BufRead>> {
    if filename == Path::new("-") {
        let stdin = io::stdin();
        stdin.lock();
        return Ok(Box::new(BufReader::new(stdin)));
    }
    let file = File::open(filename)?;
    return Ok(Box::new(BufReader::new(file)));
}

pub fn buf_writer(filename: &Path) -> Result<Box<dyn Write>> {
    if filename == Path::new("-") {
        return Ok(Box::new(BufWriter::new(io::stdout())));
    }
    let file = File::create(filename)?;
    return Ok(Box::new(BufWriter::new(file)));
}
