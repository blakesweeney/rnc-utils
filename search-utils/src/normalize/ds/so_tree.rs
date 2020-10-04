use std::{
    collections::HashMap,
    fs::File,
    io::{
        BufRead,
        BufReader,
    },
    path::Path,
};

use serde::{
    Deserialize,
    Serialize,
};

use anyhow::Result;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SoTreeEntry(String, String);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SoTree(Vec<SoTreeEntry>);

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct SoLine(String, SoTree);

pub fn load(filename: &Path) -> Result<HashMap<String, SoTree>> {
    let mut map: HashMap<String, SoTree> = HashMap::new();
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let entry: SoLine = serde_json::from_str(&line)?;
        map.insert(entry.0, entry.1);
    }

    Ok(map)
}
