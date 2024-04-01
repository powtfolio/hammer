use clap::{Parser, Subcommand};
use colored::Colorize;
use serde::Serialize;
use std::{error::Error, fs, path::Path};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    // Path to create new workspace in
    #[arg(short, long)]
    path: Option<String>,

    // Solana token standard to generate
    // Reference: https://developers.metaplex.com/core/create-asset
    #[arg(short, long)]
    standard: Option<TokenStandard>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    // Initialize a workspace
    New { name: String },
}

#[derive(clap::ValueEnum, Clone, Default, Debug, Serialize)]
enum TokenStandard {
    #[default]
    Fungible,
}

fn print_header() {
    println!("{}", "  _                                         ".green());
    println!("{}", " | |                                        ".green());
    println!("{}", " | |                                        ".green());
    println!("{}", " | |__   __ _ _ __ ___  _ __ ___   ___ _ __ ".green());
    println!(
        "{}",
        " | '_ \\ / _` | '_ ` _ \\| '_ ` _ \\ / _ \\ '__| ".green()
    );
    println!("{}", " | | | | (_| | | | | | | | | | | |  __/ |   ".green());
    println!(
        "{}",
        " |_| |_|\\__,_|_| |_| |_|_| |_| |_|\\___|_|   ".green()
    );
    println!();
    println!(
        "    {} {} {} ",
        "Flatten the".green(),
        "shit".bold().green(),
        "out of your assets".green()
    );
    println!();
}

fn main() -> Result<(), Box<dyn Error>> {
    // display ASCII art
    print_header();

    let args = Cli::parse();

    match &args.command {
        Commands::New { name } => {
            println!("FUNGIBLE STANDARD");
            println!("PATH: {:?}", args.path);
            println!("NAME: {:?}", name);

            let path_string = match args.path {
                Some(p) => p,
                None => ".".to_string(),
            };

            fs::create_dir_all(Path::new(&format!(
                "{}/{}/assets",
                path_string,
                name.to_lowercase()
            )))?;

            //
            // Some(path_string) => println!("{}", path_string),
            // // Some(path_string) => fs::create_dir_all(Path::new(path_string))?,
            // None => fs::create_dir_all(Path::new("./fungible"))?,
        }
    }

    Ok(())
}

// Some(path_string) => {
//     println!("{} {} at {}", "Created".bold().green(), name, path_string)
// }
// None => {
//     print_header();
//     println!(
//         "{} no path supplied. Using current directory",
//         "Warning".yellow()
//     );
//     println!(
//         "{} `{}` in the current directory",
//         "Created".bold().green(),
//         name
//     )
// }
