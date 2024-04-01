use clap::{Parser, Subcommand};
use colored::Colorize;
use std::{error::Error, fs, path::Path};

mod helpers;
mod tui;
mod types;

const CURRENT_DIRECTORY: &str = ".";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    // Configure current workspace metadata
    Config,
    // Initialize a new workspace
    New {
        name: String,

        // hotdog
        #[arg(help = "where to create workspace", short, long, default_value = CURRENT_DIRECTORY)]
        path: String,

        #[arg(help = "token standard to generate for according to Metaplex", value_enum, short, long, default_value_t = types::TokenStandard::default())]
        standard: types::TokenStandard,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    // display ASCII art
    helpers::print_header();

    let args = Cli::parse();

    match &args.command {
        Commands::Config => {
            tui::config_app()?;
        }

        Commands::New {
            name,
            path,
            standard,
        } => {
            let full_path = format!("{}/{}", path, name.to_lowercase());

            // create new workspace
            fs::create_dir_all(Path::new(&format!("{}/assets", full_path)))?;
            fs::create_dir_all(Path::new(&format!("{}/layers", full_path)))?;

            if let CURRENT_DIRECTORY = path.as_ref() {
                println!(
                    "{}: No path supplied. Using current directory",
                    "Warning".yellow()
                );
            }

            println!("{} {} at {}", "Created".bold().green(), name, path);

            match standard {
                types::TokenStandard::Fungible => {
                    let metadata = types::FungibleMetadata {
                        name: "My Coin".to_string(),
                        symbol: "MC".to_string(),
                        description: "This is my coin.".to_string(),
                        image: "https://linktoimage.com".to_string(),
                    };

                    helpers::metadata_to_json_file(
                        format!("{}/{}", full_path, "metadata.json"),
                        &metadata,
                    )?;
                }
                types::TokenStandard::SemiFungible => {
                    todo!()
                }
            };
        }
    }

    Ok(())
}
