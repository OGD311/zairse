use std::fs::{read, File, read_dir};
use std::io::{Error, Write};
use std::path::PathBuf;
use jiff::Timestamp;

use crate::archive::{build_archive_entry, ArchiveFormat};

pub fn encode_archive(paths: Vec<PathBuf>, output: Option<PathBuf>) -> Result<(), Error> {
    let mut archive_entries: Vec<ArchiveFormat> = Vec::new();
    let mut pending_paths = paths;
    let output_folder = output.unwrap_or_else(|| PathBuf::from(format!("out{}", Timestamp::now()
        .strftime("%Y%m%d-%H%M"))));

    while let Some(path) = pending_paths.pop() {
        if path.is_dir() {
            let dir_read = read_dir(path.clone())?;

            let files = dir_read.collect::<Result<Vec<_>, Error>>()?;
            for file in files {
                pending_paths.push(file.path());
            }
            continue;
        }

        let mut archive_entry = build_archive_entry(path.as_path())?;

        let mut file_data = read(path.as_path())?;
        archive_entry.contents = Some(file_data);

        archive_entries.push(archive_entry);
    }

    let mut output =  File::create(format!("{}.zrs", output_folder.display()))?;

    let count = archive_entries.len() as u32;
    output.write_all(&count.to_le_bytes())?;

    for archive_entry in archive_entries {
        archive_entry.serialize(&mut output)?
    }

    Ok(())
}