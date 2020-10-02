use std::{
    collections::HashMap,
    fs::File,
    io::Write,
    path::{
        Path,
        PathBuf,
    },
};

use serde::{
    Deserialize,
    Serialize,
};

use anyhow::anyhow;
use fallible_iterator::FallibleIterator;
use walkdir::WalkDir;

use rnc_core::{
    europe_pmc::XmlIterator,
    publications::external_reference::{
        ConversionError,
        ExternalReference,
    },
};

#[derive(Debug, Serialize, Deserialize)]
struct RawEntry {
    accession: String,
    reference_id: String,
}

impl RawEntry {
    pub fn reference_id(&self) -> Result<ExternalReference, ConversionError> {
        self.reference_id.parse::<ExternalReference>()
    }
}

type Mapping = HashMap<ExternalReference, Vec<RawEntry>>;

fn load_mapping(raw: &Path) -> anyhow::Result<Mapping> {
    let mut mapping = HashMap::new();
    let file = rnc_utils::reader(&raw)?;
    let mut reader = csv::Reader::from_reader(file);
    for result in reader.deserialize() {
        let entry: RawEntry = result?;
        let ref_id = entry.reference_id()?;
        let current = mapping.entry(ref_id).or_insert(Vec::new());
        current.push(entry);
    }

    Ok(mapping)
}

fn fetch_files(raw: &Path) -> anyhow::Result<Vec<PathBuf>> {
    if !raw.exists() {
        return Err(anyhow!("Missing file(s) to process at {:?}", &raw));
    }
    if raw.is_file() {
        return Ok(vec![PathBuf::from(raw)]);
    }
    if raw.is_dir() {
        let dirs = WalkDir::new(raw)
            .follow_links(true)
            .into_iter()
            .filter_entry(|e| e.file_type().is_file())
            .filter_map(Result::ok)
            .map(|e| PathBuf::from(e.path()))
            .collect::<Vec<PathBuf>>();
        return Ok(dirs);
    }

    Err(anyhow!("Cannot handle {:?}", &raw))
}

fn write_lookup(
    filename: &Path,
    mapping: &mut Mapping,
    writer: &mut csv::Writer<Box<dyn Write>>,
) -> anyhow::Result<()> {
    let mut iter = XmlIterator::from_file(&filename)?;
    while let Some(reference) = iter.next()? {
        for pub_id in reference.external_ids() {
            log::trace!("Looking up match references using {:?}", &pub_id);
            match mapping.remove(&pub_id) {
                None => (),
                Some(entries) => {
                    for raw in entries {
                        log::trace!("Found {:?} as a match", &raw);
                        writer.write_record(&[
                            reference.md5(),
                            raw.accession,
                            reference.authors(),
                            reference.location(),
                            reference.title().to_string(),
                            reference.pmid().map(|s| s.to_string()).unwrap_or(String::new()),
                            reference.doi().map(|s| s.to_string()).unwrap_or(String::new()),
                        ])?;
                    }
                },
            }
        }
    }

    Ok(())
}

pub fn write_references(
    xml_directory: &Path,
    raw: &Path,
    column: usize,
    missing_to: &Option<PathBuf>,
    output: &Path,
) -> anyhow::Result<()> {
    let xml_files = fetch_files(&xml_directory)?;
    if xml_files.len() == 0 {
        return Err(anyhow!("No xml files found in {:?}", &xml_directory));
    }

    let mut writer =
        csv::WriterBuilder::new().has_headers(false).from_writer(rnc_utils::writer(&output)?);

    let mut mapping = load_mapping(&raw)?;
    if mapping.is_empty() {
        return Err(anyhow!("Nothing to lookup in {:?}", &raw));
    }

    for xml_file in xml_files {
        log::debug!("Looking up publications in {:?}", &xml_file);
        write_lookup(&xml_file, &mut mapping, &mut writer)?;
        if mapping.is_empty() {
            log::debug!("Ending early, nothing left to map");
            return Ok(());
        }
    }

    match missing_to {
        None => (),
        Some(path) => {
            log::info!("Writing {} missing data to {:?}", mapping.len(), &path);
            let missing = File::create(&path)?;
            let mut missing = csv::Writer::from_writer(missing);
            for (key, records) in &mapping {
                log::warn!("Did not find publication information for {:?}", &key);
                for record in records {
                    missing.serialize(&record)?;
                }
            }
        },
    }

    Ok(())
}
