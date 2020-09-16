use std::convert::TryFrom;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

use fnv::FnvHashSet;
use rnc_core::urs::UrsTaxid;

pub struct UrsTaxidContainer {
    set: FnvHashSet<UrsTaxid>,
}

impl UrsTaxidContainer {
    pub fn from_path(path: &Path) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut set: FnvHashSet<UrsTaxid> = FnvHashSet::default();
        let mut buf = String::new();
        loop {
            match reader.read_line(&mut buf)? {
                0 => break,
                _ => {
                    let to_parse = buf.trim_end();
                    let urs_taxid = UrsTaxid::try_from(to_parse)?;
                    set.insert(urs_taxid);
                    buf.clear();
                }
            }
        }
        Ok(Self { set })
    }

    pub fn contains(&self, urs: &UrsTaxid) -> bool {
        self.set.contains(&urs)
    }
}
