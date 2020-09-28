use std::{
    error::Error,
    io,
    io::Write,
};

use std::{
    io::BufRead,
    path::{
        Path,
        PathBuf,
    },
    process::Output,
};

use tempfile::{
    tempdir,
    NamedTempFile,
};

use serde_json::json;

trait Jsonl {
    fn jsonl(&self) -> Result<Vec<serde_json::Value>, Box<dyn Error>>;
}

impl Jsonl for Output {
    fn jsonl(&self) -> Result<Vec<serde_json::Value>, Box<dyn Error>> {
        let data = String::from_utf8_lossy(&self.stdout);
        let mut result = Vec::new();
        for line in data.lines() {
            result.push(serde_json::from_str(&line)?);
        }
        Ok(result)
    }
}

impl Jsonl for NamedTempFile {
    fn jsonl(&self) -> Result<Vec<serde_json::Value>, Box<dyn Error>> {
        let buf = io::BufReader::new(self);
        let mut data = Vec::new();
        for line in buf.lines() {
            let line = line?;
            println!("{}", &line);
            data.push(serde_json::from_str(&line)?);
        }
        Ok(data)
    }
}

fn index(data_type: &str, data_path: &Path, db_path: &Path) -> io::Result<Output> {
    test_bin::get_test_bin("kv").arg("index").arg(data_type).arg(data_path).arg(db_path).output()
}

fn lookup(id_file: &Path, db_path: &Path, output: &Path) -> io::Result<Output> {
    test_bin::get_test_bin("kv").arg("lookup").arg(db_path).arg(id_file).arg(output).output()
}

fn temp_file_with(lines: Vec<&str>) -> io::Result<NamedTempFile> {
    let mut temp = NamedTempFile::new()?;
    for line in lines {
        writeln!(&mut temp, "{}", &line)?;
    }
    Ok(temp)
}

#[test]
fn simple_indexing_test() -> Result<(), Box<dyn Error>> {
    let data_file = temp_file_with(vec![
        r#"{"id": "1", "value": "2"}"#,
        r#"{"id": "2", "value": "3"}"#,
        r#"{"id": "3", "value": "4"}"#,
        r#"{"id": "4", "value": "5"}"#,
    ])?;
    let out_file = tempdir()?;
    let result = index("example", data_file.path(), out_file.path())?;
    assert_eq!(result.status.success(), true);
    assert_eq!(String::from_utf8_lossy(&result.stdout), "");
    assert_eq!(String::from_utf8_lossy(&result.stderr), "");

    Ok(())
}

#[test]
fn indexing_and_grouping_test() -> Result<(), Box<dyn Error>> {
    let data_file = temp_file_with(vec![
        r#"{"id": "1", "value": "2"}"#,
        r#"{"id": "2", "value": "3"}"#,
        r#"{"id": "3", "value": "4"}"#,
        r#"{"id": "4", "value": "5"}"#,
        r#"{"id": "a", "value": "5"}"#,
        r#"{"id": "a", "value": "6"}"#,
    ])?;
    let out_file = tempdir()?;
    let result = index("example", data_file.path(), out_file.path())?;
    assert_eq!(result.status.success(), true);
    assert_eq!(String::from_utf8_lossy(&result.stdout), "");
    assert_eq!(String::from_utf8_lossy(&result.stderr), "");

    Ok(())
}

#[test]
fn simple_indexing_and_querying() -> Result<(), Box<dyn Error>> {
    let data_file = temp_file_with(vec![
        r#"{"id": "1", "value": "2"}"#,
        r#"{"id": "2", "value": "3"}"#,
        r#"{"id": "3", "value": "4"}"#,
        r#"{"id": "4", "value": "5"}"#,
    ])?;
    let id_file = temp_file_with(vec!["1", "3", "4"])?;
    let db_dir = tempdir()?;

    let res = index("example", data_file.path(), db_dir.path())?;
    assert_eq!(res.status.success(), true);

    let output = PathBuf::from("-");
    let query = lookup(id_file.path(), db_dir.path(), &output)?;

    assert_eq!(String::from_utf8_lossy(&query.stderr), "");
    assert_eq!(
        query.jsonl()?,
        vec![
            json!({"id": "1", "example": [{"id": "1", "value": "2"}]}),
            json!({"id": "3", "example": [{"id": "3", "value": "4"}]}),
            json!({"id": "4", "example": [{"id": "4", "value": "5"}]}),
        ]
    );
    assert_eq!(query.status.success(), true);

    Ok(())
}

#[test]
fn lookup_grouped_test() -> Result<(), Box<dyn Error>> {
    let data_file = temp_file_with(vec![
        r#"{"id": "1", "value": "2"}"#,
        r#"{"id": "2", "value": "3"}"#,
        r#"{"id": "3", "value": "4"}"#,
        r#"{"id": "4", "value": "5"}"#,
        r#"{"id": "a", "value": "5"}"#,
        r#"{"id": "a", "value": "6", "second": "2"}"#,
    ])?;
    let db_dir = tempdir()?;
    let result = index("example", data_file.path(), db_dir.path())?;
    assert_eq!(result.status.success(), true);

    let id_file = temp_file_with(vec!["a"])?;
    let output = PathBuf::from("-");
    let query = lookup(id_file.path(), db_dir.path(), &output)?;
    assert_eq!(String::from_utf8_lossy(&query.stderr), "");
    assert_eq!(
        query.jsonl()?,
        vec![json!(
            {"id": "a", "example": [
                {"id": "a", "value": "5"},
                {"id": "a", "value": "6", "second": "2"},
            ]
        })]
    );
    assert_eq!(query.status.success(), true);

    Ok(())
}
