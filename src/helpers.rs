use colored::Colorize;
use serde::Serialize;
use std::io::{BufWriter, Write};
use std::{error::Error, fs};

pub fn print_header() {
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

pub fn metadata_to_json_file<M: Serialize>(
    path: String,
    metadata: &M,
) -> Result<(), Box<dyn Error>> {
    let file = fs::File::create(path)?;

    let mut writer = BufWriter::new(file);

    serde_json::to_writer(&mut writer, &metadata)?;
    writer.flush()?;
    Ok(())
}
