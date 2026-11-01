extern crate core;

mod archive;
mod encode;
mod decode;


use std::io::{Read, Write};
use std::path::{PathBuf};
use clap::{Parser, Subcommand};
use crate::decode::{decode_archive};
use crate::encode::{encode_archive};

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
           match encode_archive(input) {
               Err(err) => println!("failed to encode ZRS archive: {}", err),
               _ => { println!("Encoded")}
           }
        }

        Command::Decode { input } => {
            match decode_archive(input) {
                Err(err) => println!("failed to decode ZRS archive: {}", err),
                _ => { println!("Decoded")}
            }
        }
    }

}