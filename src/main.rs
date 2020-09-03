use std::path::PathBuf;

extern crate lazy_static;
extern crate log;

use anyhow::Result;
use structopt::StructOpt;

pub mod fst_utils;
pub mod sequences;
pub mod utils;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
enum SequenceCommands {
    /// Filter a psql JSON file of sequences to select only those that are in the given id file.
    SelectKnown {
        #[structopt(parse(from_os_str))]
        fst_file: PathBuf,

        #[structopt(parse(from_os_str))]
        raw: PathBuf,

        #[structopt(parse(from_os_str))]
        output: PathBuf,
    },

    /// Filter a psql JSON file of sequences to reject those that are in the given id file.
    RejectKnown {
        #[structopt(parse(from_os_str))]
        fst_file: PathBuf,

        #[structopt(parse(from_os_str))]
        raw: PathBuf,

        #[structopt(parse(from_os_str))]
        output: PathBuf,
    },

    /// Filter all sequences to only those that are valid for infernal/easel.
    SelectEasel {
        #[structopt(parse(from_os_str))]
        raw: PathBuf,

        #[structopt(parse(from_os_str))]
        output: PathBuf,
    },

    /// Convert a JSON sequence file and turn it into a standard fasta file.
    ToFasta {
        #[structopt(parse(from_os_str))]
        raw: PathBuf,

        #[structopt(parse(from_os_str))]
        output: PathBuf,
    },
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
enum FstCommands {
    /// Build a FST set, this can be replaced with the fst crate if need be.
    Build {
        #[structopt(parse(from_os_str))]
        filename: PathBuf,

        #[structopt(parse(from_os_str))]
        output: PathBuf,
    },
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
enum Opt {
    /// Some commands to deal with filtering sequences in JSON format from the database.
    JsonSequence {
        #[structopt(subcommand)]
        command: SequenceCommands,
    },

    /// Commands dealing with building up fst sets.
    FstSet {
        #[structopt(subcommand)]
        command: FstCommands,
    },
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    match opt {
        Opt::JsonSequence { command } => match command {
            SequenceCommands::SelectKnown {
                fst_file,
                raw,
                output,
            } => sequences::write_selected(&fst_file, &raw, &output, &sequences::Selection::InIdSet),
            SequenceCommands::RejectKnown {
                fst_file,
                raw,
                output,
            } => sequences::write_selected(&fst_file, &raw, &output, &sequences::Selection::NotInIdSet),
            SequenceCommands::SelectEasel { raw, output } => sequences::write_easel(&raw, &output),
            SequenceCommands::ToFasta { raw, output } => sequences::write_fasta(&raw, &output),
        },
        Opt::FstSet { command } => match command {
            FstCommands::Build { filename, output } => fst_utils::build(&filename, &output),
        },
    }
}
