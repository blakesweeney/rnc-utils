use std::path::PathBuf;

use structopt::StructOpt;

use anyhow::{
    Context,
    Result,
};

use rnc_core::{
    containers::urs_taxid::UrsTaxidMapping,
    urs::Urs,
};

/// This is a tool to process a file of JSON objects and expand their URS entry to
/// urs_taxid entries. Each object must contain a 'urs' field which contains the URS to
/// expand. This will then produce an object with an 'id' field for each urs_taxid with
/// the same urs.
#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct Opt {
    /// A file where each line is a urs_taxid, which are all urs_taxids that need to be
    /// output. Duplicate will be treated as single entry.
    #[structopt(parse(from_os_str))]
    active_file: PathBuf,

    /// A file ('-' means stdin) where each line is a valid json object, which contains a
    /// key 'urs' that is a URS that exists in RNAcentral. If the UPI is not in the
    /// active_file the object will not be written and a warning logged.
    #[structopt(parse(from_os_str))]
    filename: PathBuf,

    /// File to output to, '-' means stdout.
    #[structopt(parse(from_os_str))]
    output: PathBuf,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let mut input = rnc_utils::buf_reader(&opt.filename)?;
    let mut output = rnc_utils::buf_writer(&opt.output)?;

    let container = UrsTaxidMapping::from_urs_file(&opt.active_file)?;
    let mut buf = String::new();
    loop {
        match input.read_line(&mut buf)? {
            0 => break,
            _ => {
                let line = buf.replace("\\\\", "\\");
                let mut json: serde_json::Value = serde_json::from_str(&line)
                    .with_context(|| format!("Cannot parse JSON object {}", &line))?;

                if let Some(m) = json.as_object_mut() {
                    if let Some(serde_json::Value::String(raw_urs)) = m.get("urs") {
                        let urs: Urs = raw_urs
                            .parse()
                            .with_context(|| format!("Failed to parse URS id {}", &raw_urs))?;
                        let urs_taxids = container.urs_taxids(&urs);
                        if urs_taxids.len() == 0 {
                            log::warn!("No active URS_taxid found for: {}", &raw_urs);
                        }

                        for urs_taxid in urs_taxids {
                            m.insert(
                                String::from("id"),
                                serde_json::Value::String(urs_taxid.to_string()),
                            );
                            serde_json::to_writer(&mut output, &m)?;
                            writeln!(&mut output)?;
                        }
                    }
                }

                buf.clear();
            },
        }
    }

    Ok(())
}
