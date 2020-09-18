use std::path::PathBuf;
extern crate log;
use structopt::StructOpt;

pub mod stream_lookup;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
enum Subcommand {
    StreamLookup {
        #[structopt(default_value = "0")]
        column: usize,

        #[structopt(parse(from_os_str))]
        missing_to: Option<PathBuf>,

        #[structopt(parse(from_os_str))]
        xml_directory: PathBuf,

        #[structopt(parse(from_os_str), default_value = "-")]
        raw: PathBuf,

        #[structopt(parse(from_os_str), default_value = "-")]
        output: PathBuf,
    },
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct Opt {
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: u32,

    #[structopt(subcommand)]
    command: Subcommand,
}

fn main() -> anyhow::Result<()> {
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
        Subcommand::StreamLookup {
            column,
            missing_to,
            xml_directory,
            raw,
            output,
        } => stream_lookup::write_references(&xml_directory, &raw, column, &missing_to, &output),
    }
}
