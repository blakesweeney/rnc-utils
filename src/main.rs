use std::path::PathBuf;

extern crate lazy_static;
extern crate log;

use anyhow::Result;
use structopt::StructOpt;

pub mod fst_utils;
pub mod json_store;
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
enum JsonStoreCommands {
    /// Index a JSON document.
    Index {
        #[structopt(short, long, default_value = "1000000")]
        chunk_size: usize,

        /// Type of data being indexed, eg, secondary_structure, hits, etc
        data_type: String,

        #[structopt(parse(from_os_str))]
        /// Filename of the raw json file, '-' means stdin.
        filename: PathBuf,

        #[structopt(parse(from_os_str))]
        /// Filename to store the index in.
        output: PathBuf,
    },

    /// Extract all data between min, max (inclusive) in a store.
    ExtractRange {
        #[structopt(parse(from_os_str))]
        /// The name of the file the index database is in.
        filename: PathBuf,

        /// Min index to access.
        min: i64,

        /// Max index to access, inclusive.
        max: i64,

        #[structopt(parse(from_os_str), default_value = "-")]
        /// Output filename, '-' is stdout.
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

    /// Commands dealing with indexing JSON documents
    JsonStore {
        #[structopt(subcommand)]
        command: JsonStoreCommands,
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
            } => {
                sequences::write_selected(&fst_file, &raw, &output, &sequences::Selection::InIdSet)
            }
            SequenceCommands::RejectKnown {
                fst_file,
                raw,
                output,
            } => sequences::write_selected(
                &fst_file,
                &raw,
                &output,
                &sequences::Selection::NotInIdSet,
            ),
            SequenceCommands::SelectEasel { raw, output } => sequences::write_easel(&raw, &output),
            SequenceCommands::ToFasta { raw, output } => sequences::write_fasta(&raw, &output),
        },
        Opt::FstSet { command } => match command {
            FstCommands::Build { filename, output } => fst_utils::build(&filename, &output),
        },
        Opt::JsonStore { command } => match command {
            JsonStoreCommands::Index {
                chunk_size,
                data_type,
                filename,
                output,
            } => json_store::index(&data_type, &filename, chunk_size, &output),
            JsonStoreCommands::ExtractRange {
                filename,
                min,
                max,
                output,
            } => json_store::extract_range(&filename, min, max, &output),
        },
    }
}
