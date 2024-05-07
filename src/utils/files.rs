use std::{ffi::OsStr, fs::{self, File}, io::{BufRead, BufReader, BufWriter, Write}, path::{Path, PathBuf}};

use globset::{Glob, GlobSet, GlobSetBuilder};
use lopdf::Document;
use walkdir::WalkDir;

use crate::utils::error::{Result, Error};

pub fn bundle_to_file(files: Vec<PathBuf>, dst_file: &Path) -> Result<()> {
    let mut writer = BufWriter::new(File::create(dst_file)?);

    for file in files {
        if !file.is_file() {
            return Err(Error::IsNotFile(file.to_string_lossy().to_string()));
        }
        let reader = get_reader(&file)?;


        for line in reader.lines() {
            let line = line?;
            writeln!(writer, "{}", line)?;
        }
        writeln!(writer, "\n\n")?;
    }
    writer.flush()?;

    Ok(())
}

pub fn bundle_to_pdf(files: Vec<PathBuf>, dst_file: &Path) -> Result<()> {
    for file in files {
        if !file.is_file() {
            return Err(Error::IsNotFile(file.to_string_lossy().to_string()));
        }
        let mut doc = Document::load(file.as_path()).unwrap();
        doc.save(dst_file)?;
    }

    Ok(())
}

fn get_reader(file: &Path) -> Result<BufReader<File>> {
    let Ok(file) = File::open(file) else {
        return Err(Error::FileNotFound(file.to_string_lossy().to_string()))
    };

    Ok(BufReader::new(file))
}

pub fn load_from_toml<T>(file: impl AsRef<Path>) -> Result<T> 
where
    T: serde::de::DeserializeOwned
{
    let content = read_to_string(file.as_ref())
        .map_err(|_| Error::ReadFileToStringErro)?;

    Ok(
        toml::from_str(&content)
            .map_err(|_| Error::ConvertStrFromTomlError)?
    )
}

pub fn read_to_string(file: &Path) -> Result<String> {
    if !file.is_file() {
        return Err(Error::FileNotFound(file.display().to_string()));
    }
    let content = fs::read_to_string(file)?;

    Ok(content)
}

pub fn list_files(
    dir: &Path,
    include_globs: Option<&[&str]>,
    exclude_globs: Option<&[&str]>,
) -> Result<Vec<PathBuf>> {
    let base_dir_exclude = base_dir_exclude_globs()?;

    let depth = include_globs
        .map(|globs| globs.iter().any(|&glob| glob.contains("**")))
        .map(|v| if v { 100 } else { 1 })
        .unwrap_or(1);
    
    let include_globs = include_globs.map(get_glob_set).transpose()?;
    let exclude_globs = exclude_globs.map(get_glob_set).transpose()?;

    let walk_dir_it = WalkDir::new(dir)
        .max_depth(depth)
        .into_iter()
        .filter_entry(|e|
            if e.file_type().is_dir() {
                !base_dir_exclude.is_match(e.path())
            } else {
                if let Some(exclude_globs) = exclude_globs.as_ref() {
                    if exclude_globs.is_match(e.path()) {
                        return false;
                    }
                }

                match include_globs.as_ref() {
                    Some(globs) => globs.is_match(e.path()),
                    None => true,
                }
            }
        )
        .filter_map(|e| e.ok().filter(|e| e.file_type().is_file()));

    let paths = walk_dir_it.map(|e| e.into_path());

    Ok(paths.collect())
}

fn base_dir_exclude_globs() -> Result<GlobSet> {
    get_glob_set(&["**/.git", "**/target"])
}

pub fn get_glob_set(globs: &[&str]) -> Result<GlobSet> {
    let mut builder = GlobSetBuilder::new();
    for glob in globs {
        builder.add(Glob::new(glob)?);
    }
    Ok(builder.build()?)
}

pub fn load_from_json<T>(file: impl AsRef<Path>) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let val = serde_json::from_reader(get_reader(file.as_ref())?)?;
    Ok(val)
}

pub fn save_to_json<T>(file: impl AsRef<Path>, data: &T) -> Result<()>
where
    T: serde::Serialize,
{
    let file = file.as_ref();

    let file = File::create(file)?;
    serde_json::to_writer_pretty(file, data)?;

    Ok(())
}

pub trait XFile {
    fn x_file_name(&self) -> &str;
}

impl XFile for Path {
    fn x_file_name(&self) -> &str {
        self.file_name().and_then(OsStr::to_str).unwrap_or("")
    }
}