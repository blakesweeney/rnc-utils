use std::collections::HashMap;
use std::fs::create_dir_all;
use std::fs::File;
use std::path::PathBuf;
use std::cell::RefCell;

use anyhow::Result;
use structopt::StructOpt;

/// This is a program to
#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct Opt {
    /// Suffix for output filenames.
    #[structopt(short, long, default_value = "csv")]
    suffix: String,

    #[structopt(short, long, default_value = ",")]
    delimiter: u8,

    #[structopt(short, long)]
    has_headers: bool,

    /// The CSV file to extract data from. If the filename is '-' then the stdin will be read.
    #[structopt(parse(from_os_str))]
    raw: PathBuf,

    /// Index of the column to use for finding the name of the file to write to. This is 0 based,
    /// so the first
    column_index: usize,

    /// Directory to output into. If it does not exist, it will be created.
    #[structopt(parse(from_os_str))]
    output: PathBuf,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    create_dir_all(&opt.output)?;
    let input = rnc_utils::buf_reader(&opt.raw)?;
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(opt.delimiter)
        .has_headers(opt.has_headers)
        .from_reader(input);

    let mut mapping: HashMap<String, RefCell<csv::Writer<File>>> = HashMap::new();
    for record in reader.records() {
        let record = record?;
        let value = &record[opt.column_index];
        if !mapping.contains_key(value) {
            let mut path = PathBuf::from(&opt.output);
            path.push(value);
            path.set_extension(&opt.suffix);
            let writer = csv::Writer::from_writer(File::create(path)?);
            mapping.insert(value.to_string(), RefCell::new(writer));
        }
        let mut writer = mapping[value].borrow_mut();
        writer.write_record(&record)?;
    }

    Ok(())
}
