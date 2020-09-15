use std::path::PathBuf;
use std::error::Error;

extern crate log;

use bio::io::fasta;

use structopt::StructOpt;

use rnc_core::json_sequence::{Sequence, each_sequence};
use rnc_core::nhmmer::valid_sequence;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct Opt {
    /// This will select only sequence which have known easel/infernal characters. This limits things
    /// to matching ACGUN.
    #[structopt(short, long)]
    only_valid_easel: bool,

    /// The name of the file to read from, using '-' means stdin.
    #[structopt(parse(from_os_str))]
    raw: PathBuf,

    /// The name of the file to write to, using - means stdout.
    #[structopt(parse(from_os_str))]
    output: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    let input = rnc_utils::buf_reader(&opt.raw)?;
    let output = rnc_utils::buf_writer(&opt.output)?;
    let mut writer = fasta::Writer::new(output);
    each_sequence(input, |sequence: Sequence| {
        if !opt.only_valid_easel || valid_sequence(&sequence.sequence) {
            writer.write_record(&sequence.into())?;
        }
        Ok(())
    })
}
