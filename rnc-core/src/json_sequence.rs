use std::error::Error;
use std::io::prelude::*;

use bio::io::fasta;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Sequence<'a> {
    pub id: &'a str,
    pub description: Option<String>,
    pub sequence: &'a str,
}

impl<'a> From<Sequence<'a>> for fasta::Record {
    fn from(entry: Sequence<'a>) -> fasta::Record {
        fasta::Record::with_attrs(
            entry.id,
            entry.description.as_ref().map(|s| s.as_str()),
            entry.sequence.as_bytes(),
        )
    }
}

pub fn each_sequence(
    mut reader: Box<dyn BufRead>,
    mut f: impl FnMut(Sequence) -> Result<(), Box<dyn Error>>,
) -> Result<(), Box<dyn Error>> {
    let mut buf = String::new();
    loop {
        match reader.read_line(&mut buf)? {
            0 => break,
            _ => {
                let cleaned = buf.replace("\\\\", "\\");
                let data: Sequence = serde_json::from_str(&cleaned)?;
                f(data)?;
                buf.clear();
            }
        }
    }

    Ok(())
}
