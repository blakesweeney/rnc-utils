use std::{
    collections::HashMap,
    io::BufRead,
    path::Path,
    str,
};

use serde_query::{
    DeserializeQuery,
    Query,
};

use serde_json::{
    Deserializer,
    Value,
};

use anyhow::{
    Context,
    Result,
};

use sled;

#[derive(DeserializeQuery)]
struct DocId {
    #[query(".id")]
    id: String,
}

pub struct Spec<'a> {
    path: &'a Path,
    allow_missing: bool,
    commit_size: usize,
    threads: usize,
}

impl<'a> Spec<'a> {
    pub fn new(path: &Path) -> Spec {
        Spec {
            path: path,
            allow_missing: false,
            commit_size: 1_000_000usize,
            threads: 4,
        }
    }

    pub fn set_allow_missing(&mut self, allow_missing: bool) -> () {
        self.allow_missing = allow_missing;
    }

    pub fn set_commit_size(&mut self, commit_size: usize) -> () {
        self.commit_size = commit_size;
    }
}

fn concatenate_merge(_key: &[u8], old_value: Option<&[u8]>, new_bytes: &[u8]) -> Option<Vec<u8>> {
    let mut ret = old_value.map(|ov| ov.to_vec()).unwrap_or_else(|| vec![]);
    ret.extend_from_slice(new_bytes);
    Some(ret)
}

pub fn index(spec: &Spec, data_type: &str, filename: &Path) -> anyhow::Result<()> {
    let reader = rnc_utils::buf_reader(&filename)?;
    let db: sled::Db = sled::open(spec.path)?;
    let store: sled::Tree = db.open_tree(data_type)?;
    store.set_merge_operator(concatenate_merge);
    for line in reader.lines() {
        let line = line?;
        let id: DocId = serde_json::from_str::<Query<DocId>>(&line)?.into();
        store.merge(id.id.as_bytes(), line.as_bytes())?;
    }

    Ok(())
}

pub fn lookup(spec: &Spec, key_file: &Path, output: &Path) -> anyhow::Result<()> {
    let db: sled::Db = sled::open(spec.path)?;
    let names = db.tree_names();
    let mut trees: Vec<(String, sled::Tree)> = Vec::with_capacity(names.len());
    for name in names {
        let human = String::from_utf8(name.to_vec())?;
        trees.push((human, db.open_tree(name)?));
    }

    let mut writer = rnc_utils::buf_writer(&output)?;
    let mut keys = rnc_utils::buf_reader(&key_file)?;
    let mut buf = String::new();

    loop {
        match keys.read_line(&mut buf)? {
            0 => break,
            _ => {
                let mut data = HashMap::new();
                let trimmed = buf.trim_end();
                data.insert("id", serde_json::Value::String(trimmed.to_string()));
                let mut seen = false;
                let key = trimmed.as_bytes();
                for (name, tree) in &trees {
                    let default = serde_json::Value::Array(Vec::new());
                    let to_update = data.entry(&name).or_insert(default).as_array_mut().unwrap();
                    match tree.get(key)? {
                        None => (),
                        Some(v) => {
                            seen = true;
                            let text = str::from_utf8(&v)?;
                            let values = Deserializer::from_str(text).into_iter::<Value>();
                            for value in values {
                                to_update.push(value?);
                            }
                        },
                    }
                }

                match (seen, spec.allow_missing) {
                    (true, _) => {
                        data.remove("__sled__default");
                    },
                    (false, true) => {
                        log::warn!("No data found for key {}", &buf);
                        continue;
                    },
                    (false, false) => {
                        return Err(anyhow::anyhow!("No data found for key {}", &buf));
                    },
                }

                serde_json::to_writer(&mut writer, &data)?;
                writeln!(&mut writer)?;
                buf.clear();
            },
        }
    }

    Ok(())
}