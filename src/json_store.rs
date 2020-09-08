use std::collections::HashMap;
use std::io::prelude::*;
use std::path::Path;

use itertools::Itertools;

use rusqlite::{params, Connection, OpenFlags, Row, Transaction, NO_PARAMS};
use serde::{Deserialize, Serialize};

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

CREATE INDEX IF NOT EXISTS ix_data__data_type ON indexed_data(data_type);
CREATE INDEX IF NOT EXISTS ix_data__index ON indexed_data(idx);
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

fn batch_insert<'a, 'b>(
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

pub fn index(data_type: &String, json_file: &Path, chunk_size: usize, output: &Path) -> Result<()> {
    let file = utils::buf_reader(&json_file)?;
    let chunks = file
        .lines()
        .into_iter()
        .filter_map(Result::ok)
        .map(|l| l.replace("\\\\", "\\"))
        .map(|l| Entry::from_raw(&data_type, l))
        .chunks(chunk_size);

    let mut conn = Connection::open_with_flags(
        output,
        OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
    )?;
    setup(&conn)?;
    conn.execute(INSERT_METADATA, params![data_type])?;
    for chunk in chunks.into_iter() {
        let mut txn = conn.transaction()?;
        batch_insert(&mut txn, chunk.into_iter())?;
        txn.commit()?;
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
        return Err(anyhow!("Only found {} of expected {} items", seen, expected));
    }

    Ok(())
}
