use std::{fs, path::{Path, PathBuf}};

use tracing::info;

use crate::embeddings::file::File;

use super::error::{ Error, Result };

pub fn load_files_from_dir(dir: PathBuf, ending: &str, prefix: &PathBuf) -> Result<Vec<File>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            let mut sub_files = load_files_from_dir(path, ending, prefix)?;
            files.append(&mut sub_files);
        } else if path.is_file() && path.extension().unwrap() == ending {
            info!("Path: {:?}", path);
            let contents = fs::read_to_string(&path)?;
            let path = Path::new(&path).strip_prefix(prefix)?.to_owned();
            let key = path.to_str().ok_or(Error::NotAvaliableError)?;
            let mut file = File::new(key.to_string(), contents);
            file.parse();
            files.push(file)
        }
    }

    Ok(files)
}