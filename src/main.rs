extern crate core;

mod archive;
mod encode;
mod decode;


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
        #[arg(required = true)]
        input: Vec<PathBuf>,

        #[arg(short, long)]
        out: Option<PathBuf>,
    },
    Decode {
        #[arg(required = true)]
        input: Vec<PathBuf>,

        #[arg(short, long)]
        out: Option<PathBuf>,
    },
}
fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Encode { input, out } => {
           match encode_archive(input, out) {
               Err(err) => println!("failed to encode ZRS archive: {}", err),
               _ => { println!("Encoded")}
           }
        }

        Command::Decode { input, out } => {
            match decode_archive(input) {
                Err(err) => println!("failed to decode ZRS archive: {}", err),
                _ => { println!("Decoded")}
            }
        }
    }

}