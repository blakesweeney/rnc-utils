use std::convert::TryFrom;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

use fnv::FnvHashSet;

use crate::urs::{Urs, UrsTaxid};

pub trait UrsStore {
    fn add(&mut self, urs: &Urs);
    fn contains(&self, urs: &Urs) -> bool;

    fn add_urs_taxid(&mut self, urs: &UrsTaxid) {
        return self.add(&urs.into());
    }

    fn contains_parent_urs(&self, urs: &UrsTaxid) -> bool {
        self.contains(&urs.into())
    }
}

pub struct BasicStore {
    set: FnvHashSet<u64>,
}

impl UrsStore for BasicStore {
    fn add(&mut self, urs: &Urs) {
        self.set.insert(urs.into());
    }

    fn contains(&self, urs: &Urs) -> bool {
        self.set.contains(&urs.into())
    }
}

impl BasicStore {
    pub fn from_urs_file(path: &Path) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        let mut set: FnvHashSet<u64> = FnvHashSet::default();
        let mut buf = String::new();
        loop {
            match reader.read_line(&mut buf)? {
                0 => break,
                _ => {
                    let urs: Urs = Urs::try_from(buf.trim_end())?;
                    set.insert(urs.into());
                    buf.clear();
                }
            }
        }

        Ok(Self { set })
    }
}
