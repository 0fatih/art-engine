mod images;
mod utils;
mod generator;

use crate::utils::check_files;
use crate::generator::{Collection, generate_all_images, generate_all_metadata, set_images_dir};

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(name = "arg-engine")]
#[command(author = "0fatih")]
#[command(about = "An art engine for creating NFTs", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Image {
        operation: ImageOperation
    },
    Metadata {
        #[arg(required = true)]
        name: String,
        #[arg(required = true)]
        description: String,
        #[arg(required = true)]
        base_uri: String,
    }
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum ImageOperation {
    Generate
}

fn main() {
    // make sure all the directories required are exists
    check_files().unwrap();

    match Cli::parse().command {
        Commands::Image {operation} => {
            match operation {
                 ImageOperation::Generate => {
                    set_images_dir().unwrap();
                    let layers: Vec<&str> = vec!["Background", "Bottom lid", "Eye color", "Eyeball", "Goo", "Iris", "Shine", "Top lid"];
                    generate_all_images(100, layers).unwrap();
                },
            }
        },
        Commands::Metadata { name, base_uri, description } => {
            generate_all_metadata(Collection { name, description, base_uri}).unwrap();
        }
    }
}
