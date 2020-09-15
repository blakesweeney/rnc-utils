use std::path::{Path, PathBuf};

pub fn filename(key: u64, directory: &Path, basename: &String) -> PathBuf {
    let mut path = PathBuf::from(directory);
    path.set_file_name(format!("{}-{}", basename, key));
    path.set_extension("fasta");
    path
}
