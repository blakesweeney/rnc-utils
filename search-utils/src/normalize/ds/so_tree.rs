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

// {"so_rna_type": "SO:0000261", "so_term_tree": [["SO:0000655", "ncRNA"], ["SO:0002247", "sncRNA"], ["SO:0000253", "tRNA"], ["SO:0000261", "glycyl_tRNA"]]}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct SoLine {
    so_rna_type: String,
    so_term_tree: SoTree,
}

pub fn load(filename: &Path) -> Result<HashMap<String, SoTree>> {
    let mut map: HashMap<String, SoTree> = HashMap::new();
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let entry: SoLine = serde_json::from_str(&line)?;
        map.insert(entry.so_rna_type, entry.so_term_tree);
    }

    Ok(map)
}
