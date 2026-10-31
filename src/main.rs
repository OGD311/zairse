extern crate core;

mod archive;
use archive::{ArchiveFormat, build_archive_entry};

use std::fs::{read, File, create_dir};
use std::io::{BufReader, Error, Read, Write};
use std::path::{PathBuf};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "zrs", about = "Encode and decode ZRS archives")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Encode {
        input: Vec<PathBuf>,
    },
    Decode {
        input: Vec<PathBuf>,
    },
}
fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Encode { input } => {
           match encode(input) {
               Err(err) => println!("failed to encode ZRS archive: {}", err),
               _ => { println!("Encoded")}
           }
        }

        Command::Decode { input } => {
            match decode(input) {
                Err(err) => println!("failed to decode ZRS archive: {}", err),
                _ => { println!("Decoded")}
            }
        }
    }

}

fn encode(paths: Vec<PathBuf>) -> Result<(), Error> {
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

    let mut output = match std::fs::File::create("out.zrs") {
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

fn decode(paths: Vec<PathBuf>) -> Result<(), Error> {
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

