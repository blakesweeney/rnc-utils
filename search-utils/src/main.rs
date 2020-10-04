use std::path::PathBuf;
use structopt::StructOpt;

// #[macro_use]
// extern crate soa_derive;

use anyhow::Result;

pub mod normalize;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
enum Subcommand {
    /// Process a stream of of data from a kv lookup for search data and turn it into a
    /// stream of JSON data suitable for the pipeline to process.
    Normalize {
        #[structopt(parse(from_os_str))]
        /// Filename of the data extracted data from `kv` of search data to normalize. '-'
        /// means stdin.
        input_file: PathBuf,

        #[structopt(parse(from_os_str))]
        /// Filename of the SO term tree metadata.
        so_term_tree: PathBuf,

        #[structopt(parse(from_os_str))]
        /// Where to write normalized JSON data to, '-' means stdout.
        output_file: PathBuf,
    },
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct Opt {
    /// Set the logging option, more is more verbose.
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: u32,

    #[structopt(subcommand)]
    command: Subcommand,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let level = match opt.verbose {
        0 => simplelog::LevelFilter::Warn,
        1 => simplelog::LevelFilter::Info,
        2 => simplelog::LevelFilter::Debug,
        _ => simplelog::LevelFilter::Trace,
    };
    simplelog::TermLogger::init(
        level,
        simplelog::Config::default(),
        simplelog::TerminalMode::Stderr,
    )
    .unwrap_or_else(|_| eprintln!("Failed to create logger, ignore"));

    match opt.command {
        Subcommand::Normalize {
            input_file,
            so_term_tree,
            output_file,
        } => normalize::write_file(&input_file, &so_term_tree, &output_file)?,
    }

    Ok(())
}
