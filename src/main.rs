use std::fs::{self, File};
use std::io::{self, Read};
use std::path::Path;

fn main() -> Result<(), io::Error> {
    let mut lines = 0;
    let mut characters = 0;

    // Path to the directory
    let dir_path = Path::new("./../code_count/src");

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
    println!("Lines: {}", lines);
    println!("Characters: {}", characters);

    Ok(())
}
