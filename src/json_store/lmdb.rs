use std::collections::HashMap;
use std::str;
use std::fs;
use std::io::prelude::*;
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_query::{DeserializeQuery, Query};

use lmdb;
use lmdb::Cursor;
use lmdb::Transaction;

use anyhow::{anyhow, Result};

use crate::utils;
use serde_json;

const DB_SIZE: usize = 50_000_000_000;
const MAX_DBS: u32 = 20;

#[derive(Serialize, Deserialize, Debug)]
struct Entry {
    id: i64,
    data_type: String,
    raw_data: String,
}

#[derive(DeserializeQuery)]
struct DocId {
    #[query(".id")]
    id: i64,
}

pub fn index(data_type: &String, json_file: &Path, output: &Path) -> Result<()> {
    let mut file = utils::buf_reader(&json_file)?;

    fs::create_dir_all(&output)?;
    let env = {
        let mut builder = lmdb::Environment::new();
        builder.set_max_dbs(MAX_DBS);
        builder.set_map_size(DB_SIZE);
        builder.open(&output)
    }?;

    let index = env.create_db(Some("_data_types"), lmdb::DatabaseFlags::empty())?;
    let mut txn = env.begin_rw_txn()?;
    let empty_json = serde_json::to_string(&serde_json::Map::new())?;
    txn.put(
        index,
        &data_type,
        &empty_json,
        lmdb::WriteFlags::NO_OVERWRITE,
    )?;
    txn.commit()?;

    let index = env.create_db(Some(data_type), lmdb::DatabaseFlags::empty())?;
    let mut txn = env.begin_rw_txn()?;

    let mut buf = String::new();
    loop {
        match file.read_line(&mut buf)? {
            0 => break,
            _ => {
                let cleaned = buf.replace("\\\\", "\\");
                let data: DocId = serde_json::from_str::<Query<DocId>>(&cleaned)?.into();
                let id = data.id;
                txn.put(index, &id.to_be_bytes(), &cleaned, lmdb::WriteFlags::APPEND)?;
                buf.clear();
            }
        }
    }
    txn.commit()?;

    Ok(())
}

pub fn index_files(filename: &Path, output: &Path) -> Result<()> {
    let file = utils::buf_reader(&filename)?;
    for line in file.lines() {
        let path = line?.to_owned();
        let path = Path::new(&path);
        let data_type = path
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow!("Could not get data_type from file: {:?}", path))
            .map(|s| s.to_owned())?;
        index(&data_type, &path, &output)?;
    }
    Ok(())
}

fn read_only_env(path: &Path) -> lmdb::Result<lmdb::Environment> {
    let mut builder = lmdb::Environment::new();
    builder.set_max_dbs(MAX_DBS);
    builder.set_map_size(DB_SIZE);
    let mut flags = lmdb::EnvironmentFlags::empty();
    flags.toggle(lmdb::EnvironmentFlags::NO_LOCK);
    flags.toggle(lmdb::EnvironmentFlags::READ_ONLY);
    builder.set_flags(flags);
    builder.open(&path)
}

pub fn extract_range(path: &Path, min: u64, max: u64, output: &Path) -> Result<()> {
    let mut out = utils::buf_writer(output)?;
    let env = read_only_env(&path)?;

    let mut dbs = HashMap::new();
    let data_types_index = env.open_db(Some("_data_types"))?;
    let txn = env.begin_ro_txn()?;
    let mut cursor = txn.open_ro_cursor(data_types_index)?;
    let mut empty = HashMap::new();
    for pair in cursor.iter() {
        let (raw_key, raw_value) = pair?;
        let db_name = str::from_utf8(raw_key)?;
        let text = str::from_utf8(raw_value)?;
        let json: HashMap<String, serde_json::value::Value> = serde_json::from_str(&text)?;
        empty.extend(json);
        dbs.insert(db_name, env.open_db(Some(db_name))?);
    }
    drop(cursor);
    drop(txn);

    let mut seen: u64 = 0;
    let txn = env.begin_ro_txn()?;
    for index in min..=max {
        let mut data = empty.clone();
        for (_, db) in dbs.iter() {
            let raw = match txn.get(*db, &index.to_be_bytes()) {
                | Err(lmdb::Error::NotFound) => continue,
                | Ok(r) => Ok(r),
                | Err(e) => Err(e),
            }?;

            let text = str::from_utf8(raw)?;
            let json: HashMap<String, serde_json::value::Value> = serde_json::from_str(&text)?;
            data.extend(json);
        }
        serde_json::to_writer(&mut out, &data)?;
        writeln!(&mut out)?;
        seen += 1;
    }

    let expected = (max - min) + 1;
    if seen != expected {
        return Err(anyhow!(
            "Only found {} of expected {} items",
            seen,
            expected
        ));
    }

    Ok(())
}

pub fn extract_from_file(path: &Path, filename: &Path, output: &Path) -> Result<()> {
    let mut out = utils::buf_writer(output)?;
    let env = read_only_env(&path)?;

    let mut dbs = HashMap::new();
    let data_types_index = env.open_db(Some("_data_types"))?;
    let txn = env.begin_ro_txn()?;
    let mut cursor = txn.open_ro_cursor(data_types_index)?;
    let mut empty = HashMap::new();
    for pair in cursor.iter() {
        let (raw_key, raw_value) = pair?;
        let db_name = str::from_utf8(raw_key)?;
        let text = str::from_utf8(raw_value)?;
        let json: HashMap<String, serde_json::value::Value> = serde_json::from_str(&text)?;
        empty.extend(json);
        dbs.insert(db_name, env.open_db(Some(db_name))?);
    }
    drop(cursor);
    drop(txn);

    let mut file = utils::buf_reader(&filename)?;
    let txn = env.begin_ro_txn()?;
    let mut buf = String::new();
    loop {
        let mut data = empty.clone();
        match file.read_line(&mut buf)? {
            0 => break,
            _ => {
                let key = buf.as_bytes();
                for (_, db) in dbs.iter() {
                    let raw = match txn.get(*db, &key) {
                        | Err(lmdb::Error::NotFound) => continue,
                        | Ok(r) => Ok(r),
                        | Err(e) => Err(e),
                    }?;

                    let text = str::from_utf8(raw)?;
                    let json: HashMap<String, serde_json::value::Value> = serde_json::from_str(&text)?;
                    data.extend(json);
                }

                serde_json::to_writer(&mut out, &data)?;
                writeln!(&mut out)?;
            }
        }
    }

    Ok(())
}
