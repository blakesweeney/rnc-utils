use std::convert::TryFrom;
use std::path::PathBuf;
use std::error::Error;

extern crate log;

use bio::io::fasta;

use structopt::StructOpt;

use rnc_core::json_sequence::{each_sequence, Sequence};
use rnc_core::urs::Urs;

pub mod mapping;

use crate::mapping::UrsTaxidMapping;

/// This is a command to process a list of active urs_taxids and urs sequences and produce a
/// fasta file of the active urs taxids. The sequence file only needs to contain an entry for each
/// urs and the urs_taxid file may contain duplicates.
#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct Opt {
    /// A file where each line is a urs_taxid, which are all active xrefs that need to be output.
    /// This may contain duplicates.
    #[structopt(parse(from_os_str))]
    xref_urs_taxids: PathBuf,

    /// A file where each line is json sequence and the id of each entry is a URS, '-' means stdin.
    #[structopt(parse(from_os_str))]
    filename: PathBuf,

    /// File to output to, '-' means stdout.
    #[structopt(parse(from_os_str))]
    output: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    let input = rnc_utils::buf_reader(&opt.filename)?;
    let output = rnc_utils::buf_writer(&opt.output)?;
    let mut writer = fasta::Writer::new(output);

    let urs_mapping = UrsTaxidMapping::from_path(&opt.xref_urs_taxids)?;
    each_sequence(input, |sequence: Sequence| {
        let urs = Urs::try_from(sequence.id)?;
        match urs_mapping.get(&urs) {
            None => Ok(()),
            Some(urs_taxids) => {
                let seq = sequence.sequence.as_bytes();
                for urs_taxid in urs_taxids {
                    let record = fasta::Record::with_attrs(
                        &urs_taxid.to_string(),
                        sequence.description,
                        seq
                    );
                    writer.write_record(&record)?;
                }
                Ok(())
            }
        }
    })
}
