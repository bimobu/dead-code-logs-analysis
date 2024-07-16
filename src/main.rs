use clap::Parser;
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path to the directory where the log files are located
    #[arg(short, long)]
    directory_path: String,
}

fn main() {
    let Args { directory_path } = Args::parse();

    let output_path = format!("{}/dead_code_analysis.txt", &directory_path);

    let path = Path::new(&output_path);

    if path.exists() {
        fs::remove_file(path).expect("Could not delete the output file");
    }

    let mut unique_lines = HashSet::new();

    let paths = fs::read_dir(&directory_path).expect("could not read directory");

    for path in paths {
        let file = File::open(path.unwrap().path()).expect("Failed to open input file");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                if let Some(pos) = line.find("[Dead Code Analysis]") {
                    let content = line[pos + "[Dead Code Analysis]".len()..]
                        .trim()
                        .to_string();
                    unique_lines.insert(content);
                }
            }
        }
    }

    println!("{:#?}", unique_lines);

    let mut output = File::create(&output_path).expect("Failed to create output file");

    for line in unique_lines {
        writeln!(output, "[Dead Code Analysis] {}", line).expect("Failed to write to output file");
    }

    println!("Unique lines saved to {}", output_path);
}
