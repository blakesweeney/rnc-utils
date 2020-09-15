use std::fs;
use std::io;
use std::path::PathBuf;

extern crate log;

use bio::io::fasta;

use itertools::Itertools;

use anyhow::Result;
use structopt::StructOpt;

pub mod chunks;
pub mod limits;
pub mod utils;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct Opt {
    #[structopt(short, long)]
    max_nucleotides: Option<u64>,

    #[structopt(short, long)]
    max_sequences: Option<u64>,

    #[structopt(short, long)]
    max_file_size: Option<u64>,

    #[structopt(short, long, default_value = "sequence-chunk")]
    filename: String,

    #[structopt(parse(from_os_str))]
    raw: PathBuf,

    #[structopt(parse(from_os_str))]
    output: PathBuf,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let limits = limits::Limits::new(opt.max_sequences, opt.max_nucleotides, opt.max_file_size);

    fs::create_dir_all(&opt.output)?;
    let input = rnc_utils::buf_reader(&opt.raw)?;
    let reader = fasta::Reader::new(input);
    let mut chunk = chunks::Chunks::new();
    let records = reader.records().filter_map(Result::ok).group_by(|r| {
        chunk.add_record(&r, &limits);
        chunk.key()
    });

    for (key, records) in records.into_iter() {
        let filename = utils::filename(key, &opt.output, &opt.filename);
        let writer = fs::File::create(filename)?;
        let writer = io::BufWriter::new(writer);
        let mut writer = fasta::Writer::new(writer);
        for record in records {
            writer.write_record(&record)?;
        }
    }

    Ok(())
}
