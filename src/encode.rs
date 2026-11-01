use std::fs::{read, File};
use std::io::{Error, Write};
use std::path::PathBuf;
use crate::archive::{build_archive_entry, ArchiveFormat};

pub fn encode_archive(paths: Vec<PathBuf>) -> Result<(), Error> {
    let mut archive_entries: Vec<ArchiveFormat> = Vec::new();

    for path in paths {

        let mut archive_entry =
            match build_archive_entry(path.as_path()) {
                Ok(archive_entry) => archive_entry,
                Err(error) => {
                    println!("Error: {}", error);
                    return Err(error);
                }
            };

        let mut file_data: Vec<u8> = Vec::new();
        match read(path.as_path()) {
            Ok(data) => file_data.extend(data),
            Err(error) => {
                return Err(error);
            }
        };

        archive_entry.contents = Some(file_data);

        archive_entries.push(archive_entry);
    }

    let mut output = match File::create("out.zrs") {
        Ok(file) => file,
        Err(error) => {
            return Err(error);
        }
    };

    let count = archive_entries.len() as u32;
    output.write_all(&count.to_le_bytes())?;

    for archive_entry in archive_entries {
        if let Err(error) = archive_entry.serialize(&mut output) {
            return Err(error);
        }
    }

    Ok(())
}