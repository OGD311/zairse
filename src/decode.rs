use std::fs::{create_dir, File};
use std::io::{BufReader, Error, Read, Write};
use std::path::PathBuf;
use crate::archive::ArchiveFormat;

pub fn decode_archive(paths: Vec<PathBuf>) -> Result<(), Error> {
    for path in paths {
        let input = match File::open(path.as_path()) {
            Ok(file) => file,
            Err(error) => {
                return Err(error);
            }
        };

        let mut reader = BufReader::new(input);

        let mut archive_entries: Vec<ArchiveFormat> = Vec::new();


        let mut count_bytes = [0u8; 4];
        reader.read_exact(&mut count_bytes)?;
        let entry_count = u32::from_le_bytes(count_bytes);

        for _ in 0..entry_count {
            let entry = ArchiveFormat::deserialize(&mut reader).unwrap();
            archive_entries.push(entry);
        }

        let mut output_folder = match create_dir("out") {
            Ok(folder) => folder,
            Err(error) => {
                return Err(error);
            }
        };

        for file_data in archive_entries {
            let mut output = match File::create("out/".to_string() + &*file_data.filename) {
                Ok(file) => file,
                Err(error) => {
                    return Err(error);
                }
            };

            println!("{}", file_data.filename);
            output.write(file_data.contents.unwrap().as_slice())?;
        }
    }

    Ok(())
}

