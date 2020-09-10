use std::collections::HashMap;
use std::io::prelude::*;
use std::path::Path;

use itertools::Itertools;

use rusqlite::{params, Connection, OpenFlags, Row, Transaction, NO_PARAMS};
use serde::{Deserialize, Serialize};
use serde_query::{DeserializeQuery, Query};

use anyhow::{anyhow, Result};

use crate::utils;
use serde_json;

const FETCH: &str = r#"
SELECT
    idx, data_type, data
FROM indexed_data
WHERE
    idx BETWEEN ? AND ?
ORDER BY idx, data_type
"#;

const FETCH_METADATA: &str = "SELECT name FROM data_types";

const SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS data_types (
    name TEXT PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS indexed_data (
    idx INT,
    data_type TEXT NOT NULL,
    data TEXT NOT NULL,
    UNIQUE (idx, data_type)
);
"#;

const INSERT_DATA: &str = r#"
INSERT INTO indexed_data (idx, data_type, data) VALUES (?, ?, ?)
"#;

const INSERT_METADATA: &str = "INSERT OR IGNORE INTO data_types (name) VALUES (?)";

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

impl Entry {
    pub fn from_raw(data_type: &String, raw: String) -> Result<Self> {
        let entry: serde_json::Value = serde_json::from_str(&raw)?;
        let id = match &entry["id"] {
            serde_json::Value::Number(n) => match n.as_i64() {
                Some(n) => Ok(n),
                None => Err(anyhow!("Given id must be a 64 bit int")),
            },
            _ => Err(anyhow!("Entry does not have numeric id: {}", entry)),
        }?;

        Ok(Entry {
            id,
            data_type: data_type.clone(),
            raw_data: raw,
        })
    }

    pub fn from_row(row: &Row) -> rusqlite::Result<Self> {
        let id_idx = row.column_index("idx")?;
        let data_type_idx = row.column_index("data_type")?;
        let value_idx = row.column_index("data")?;
        Ok(Self {
            id: row.get_unwrap(id_idx),
            data_type: row.get_raw(data_type_idx).as_str()?.to_owned(),
            raw_data: row.get_raw(value_idx).as_str()?.to_owned(),
        })
    }

    pub fn json_data(&self) -> serde_json::Result<serde_json::Value> {
        serde_json::from_str(&self.raw_data)
    }
}

fn setup(conn: &Connection) -> Result<()> {
    conn.execute_batch(SCHEMA)?;
    Ok(())
}

fn batch_insert<'a>(
    txn: &mut Transaction<'a>,
    entries: impl Iterator<Item = Result<Entry>>,
) -> Result<()> {
    let mut statement = txn.prepare_cached(INSERT_DATA)?;
    for entry in entries {
        let entry = entry?;
        statement.execute(params![entry.id, entry.data_type, entry.raw_data])?;
    }
    Ok(())
}

fn file_insert<'a>(
    txn: &mut Transaction<'a>,
    data_type: &String,
    file: &mut Box<dyn std::io::BufRead>,
) -> Result<usize> {
    let mut count: usize = 0;
    let mut buf = String::new();
    let mut statement = txn.prepare_cached(INSERT_DATA)?;
    loop {
        match file.read_line(&mut buf)? {
            0 => break,
            _ => {
                let cleaned = buf.replace("\\\\", "\\");
                let data: DocId = serde_json::from_str::<Query<DocId>>(&cleaned)?.into();
                let id = data.id;
                statement.execute(params![id, data_type, cleaned])?;
                count += 1;
                buf.clear();
            }
        }
    }
    Ok(count)
}

pub fn index(data_type: &String, json_file: &Path, chunk_size: usize, output: &Path) -> Result<()> {
    let mut file = utils::buf_reader(&json_file)?;

    let mut conn = Connection::open_with_flags(
        output,
        OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
    )?;
    setup(&conn)?;
    conn.execute(INSERT_METADATA, params![data_type])?;

    let mut txn = conn.transaction()?;
    let count = file_insert(&mut txn, data_type, &mut file)?;
    println!("{}", count);
    txn.commit()?;

    Ok(())
}

pub fn index_files(filename: &Path, chunk_size: usize, output: &Path) -> Result<()> {
    let file = utils::buf_reader(&filename)?;
    for line in file.lines() {
        let path = line?.to_owned();
        let path = Path::new(&path);
        let data_type = path
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow!("Could not get data_type from file: {:?}", path))
            .map(|s| s.to_owned())?;
        index(&data_type, &path, chunk_size, &output)?;
    }
    Ok(())
}

fn empty_data(conn: &Connection) -> Result<HashMap<String, serde_json::Value>> {
    let mut data = HashMap::new();
    let mut stmt = conn.prepare_cached(FETCH_METADATA)?;
    let basic = serde_json::json!("{}");
    let names = stmt.query_and_then(NO_PARAMS, |row| row.get(0))?;
    for name in names {
        data.insert(name?, basic.clone());
    }
    Ok(data)
}

pub fn extract_range(path: &Path, min: i64, max: i64, output: &Path) -> Result<()> {
    let mut out = utils::buf_writer(output)?;
    let conn = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY)?;
    let mut stmt = conn.prepare_cached(FETCH)?;
    let empty = empty_data(&conn)?;
    let grouped = stmt
        .query(params![min, max])?
        .mapped(|r| Entry::from_row(r))
        .filter_map(Result::ok)
        .group_by(|e| e.id);

    let mut seen: i64 = 0;
    for (_, group) in &grouped {
        let mut data = empty.clone();
        for entry in group {
            let json = entry.json_data()?;
            data.insert(entry.data_type, json);
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
