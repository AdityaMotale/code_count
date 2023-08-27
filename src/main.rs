use clap::Parser;
use std::fs::{self};
use std::io::{self};
use std::path::Path;

#[derive(Parser)]
struct Cli {
    /// The path of the project
    path: String,
}

fn main() -> Result<(), io::Error> {
    let args = Cli::parse();

    let dir_path = Path::new(&args.path);

    let mut lines = 0;
    let mut characters = 0;

    // Path to the directory

    // Walk through the directory and its subdirectories
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;

        if entry.file_type()?.is_file() {
            let file_path = entry.path();
            let file_content = fs::read(&file_path)?;

            // Counting bytes as characters for non-UTF-8 files
            characters += file_content.len();

            // Counting lines by counting newline characters
            lines += file_content.iter().filter(|&&c| c == b'\n').count();
        }
    }

    // Print the results
    println!("Your Project contains {} lines of code", lines);
    println!("Your project contains {} characters", characters);

    Ok(())
}
