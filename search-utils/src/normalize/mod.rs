use anyhow::{
    Context,
    Result,
};

use std::path::Path;

pub mod ds;
pub mod utils;

use crate::normalize::ds::{
    entry::{
        Normalized,
        Raw,
    },
    so_tree,
};

pub fn write_file(input_file: &Path, so_term_tree: &Path, output_file: &Path) -> Result<()> {
    let so_tree = so_tree::load(&so_term_tree)?;
    let mut reader = rnc_utils::buf_reader(input_file)?;
    let mut writer = rnc_utils::buf_writer(output_file)?;
    let mut buf = String::new();
    loop {
        match reader.read_line(&mut buf)? {
            0 => break,
            _ => {
                let raw: Raw = serde_json::from_str(&buf)?;
                let norm = Normalized::new(&raw, &so_tree)
                    .with_context(|| format!("Normalizing: {:?}", &raw))?;
                serde_json::to_writer(&mut writer, &norm)?;
                writeln!(&mut writer)?;
                buf.clear();
            },
        }
    }

    Ok(())
}
