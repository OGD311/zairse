extern crate core;

mod archive;
use archive::{ArchiveFormat, build_archive_entry};

use std::env::args;
use std::fs::{read, metadata, write, Metadata, File, create_dir};
use std::io::{BufReader, Error, Read, Write};
use std::path::Path;
use std::process::exit;

fn main() {
    let args = args().collect::<Vec<String>>();
    println!("{:?}", args);

    if args.len() < 2 {
        exit(1);
    }

    if args[1].contains(".zrs") {
        match decode(args) {
            Ok(_) => {
                println!("Decoded archive");
                exit(0);
            }
            Err(e) => {
                println!("Error: {}", e);
                exit(1);
            }
        }
    } else {
        match encode(args) {
            Ok(_) => {
                println!("Encoded archive");
                exit(0);
            },
            Err(e) => {
                println!("Error: {}", e);
                exit(1);
            }
        };
    }
}

fn encode(args: Vec<String>) -> Result<(), Error> {
    let mut archive_entries: Vec<ArchiveFormat> = Vec::new();

    for arg in args.iter().skip(1) {

        let mut archive_entry =
            match build_archive_entry(Path::new(&arg)) {
                Ok(archive_entry) => archive_entry,
                Err(error) => {
                    println!("Error: {}", error);
                    return Err(error);
                }
            };

        let mut file_data: Vec<u8> = Vec::new();
        match read(&arg) {
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

fn decode(args: Vec<String>) -> Result<(), Error> {
    let mut input = match std::fs::File::open("out.zrs") {
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


    Ok(())
}

