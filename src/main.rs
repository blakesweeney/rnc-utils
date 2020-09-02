use std::path::PathBuf;

extern crate log;

use anyhow::Result;
use structopt::StructOpt;

pub mod fst_utils;
pub mod sequences;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
enum SequenceCommands {
    /// Filter a psql JSON file of sequences to select only those that are in the given id file.
    Select {
        #[structopt(parse(from_os_str))]
        fst_file: PathBuf,

        #[structopt(parse(from_os_str))]
        raw: PathBuf,

        #[structopt(parse(from_os_str))]
        output: PathBuf,
    },

    /// Filter a psql JSON file of sequences to reject those that are in the given id file.
    Reject {
        #[structopt(parse(from_os_str))]
        fst_file: PathBuf,

        #[structopt(parse(from_os_str))]
        raw: PathBuf,

        #[structopt(parse(from_os_str))]
        output: PathBuf,
    },

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

fn handle_json_sequence(cmd: SequenceCommands) -> Result<()> {
    match cmd {
        SequenceCommands::Select {
            fst_file,
            raw,
            output,
        } => sequences::write_selected(&fst_file, &raw, &output, &sequences::Selection::InIdSet),
        SequenceCommands::Reject {
            fst_file,
            raw,
            output,
        } => sequences::write_selected(&fst_file, &raw, &output, &sequences::Selection::NotInIdSet),
        SequenceCommands::ToFasta { raw, output } => sequences::write_fasta(&raw, &output),
    }
}

fn handle_fst(cmd: FstCommands) -> Result<()> {
    match cmd {
        FstCommands::Build { filename, output } => fst_utils::build(&filename, &output),
    }
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    match opt {
        Opt::JsonSequence { command } => handle_json_sequence(command),
        Opt::FstSet { command } => handle_fst(command),
    }
}
