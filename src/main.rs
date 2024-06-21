use clap::Parser;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path to the file to read the input from
    #[arg(short, long)]
    input_path: String,
    /// The path to the file to write the output to
    #[arg(short, long)]
    output_path: String,
}

fn main() {
    let Args {
        input_path,
        output_path,
    } = Args::parse();

    let file = File::open(input_path).expect("Failed to open input file");
    let reader = BufReader::new(file);

    let mut unique_lines = HashSet::new();

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

    println!("{:#?}", unique_lines);

    let mut output = File::create(&output_path).expect("Failed to create output file");

    for line in unique_lines {
        writeln!(output, "[Dead Code Analysis] {}", line).expect("Failed to write to output file");
    }

    println!("Unique lines saved to {}", output_path);
}
