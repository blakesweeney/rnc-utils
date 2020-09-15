use std::convert::TryFrom;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

use fnv::{FnvHashMap, FnvHashSet};
use rnc_core::urs::{Urs, UrsTaxid};

pub struct UrsTaxidMapping {
    mapping: FnvHashMap<u64, FnvHashSet<UrsTaxid>>,
}

impl UrsTaxidMapping {
    pub fn from_path(path: &Path) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut mapping: FnvHashMap<u64, FnvHashSet<UrsTaxid>> = FnvHashMap::default();
        let mut buf = String::new();
        loop {
            match reader.read_line(&mut buf)? {
                0 => break,
                _ => {
                    let urs_taxid = UrsTaxid::try_from(&buf)?;
                    let urs: Urs = urs_taxid.to_urs();
                    mapping
                        .entry(urs.into())
                        .and_modify(|v| {
                            v.insert(urs_taxid);
                        })
                        .or_insert(FnvHashSet::default());
                }
            }
        }
        Ok(Self { mapping })
    }

    pub fn get(&self, urs: &Urs) -> Option<&FnvHashSet<UrsTaxid>> {
        self.mapping.get(&urs.into())
    }
}
