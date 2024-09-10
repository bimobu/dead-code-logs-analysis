use clap::Parser;
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
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

    delete_file(&output_path);

    let skiplist = get_skiplist(&directory_path);

    let mut unique_lines = HashSet::new();
    let mut skipped_lines = HashSet::new();

    let paths = fs::read_dir(&directory_path).expect("could not read directory");

    for path in paths {
        let file = File::open(path.unwrap().path()).expect("Failed to open input file");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                if let Some(pos) = line.find("[Dead Code Analysis]") {
                    let dead_code_log = line[pos + "[Dead Code Analysis]".len()..]
                        .trim()
                        .to_string();
                    if skip_dead_code_log(&dead_code_log, &skiplist) {
                        skipped_lines.insert(dead_code_log);
                    } else {
                        unique_lines.insert(dead_code_log);
                    }
                }
            }
        }
    }

    write_output_file(&output_path, unique_lines, skipped_lines)
}

fn write_output_file(
    output_path: &String,
    unique_lines: HashSet<String>,
    skipped_lines: HashSet<String>,
) {
    let unique = Vec::from_iter(unique_lines).join("\n");
    let skipped = Vec::from_iter(skipped_lines).join("\n");

    let content = format!("Unique Dead Code Logs:\n\n{unique}\n\nSkipped:\n\n{skipped}");

    println!("\n{}\n", content);

    fs::write(&output_path, content).expect("Could not write to file");
}

fn delete_file(path: &String) {
    let path = Path::new(&path);

    if path.exists() {
        fs::remove_file(path).expect("Could not delete the output file");
    }
}

fn get_skiplist(directory_path: &String) -> HashSet<String> {
    let skiplist_path = format!("{}/skiplist.txt", &directory_path);
    let file = File::open(skiplist_path).expect("Failed to open skiplist");
    let reader = BufReader::new(file);
    let mut skiplist = HashSet::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            skiplist.insert(line);
        }
    }

    skiplist
}

fn skip_dead_code_log(dead_code_log: &String, skiplist: &HashSet<String>) -> bool {
    let file = dead_code_log
        .split("'")
        .nth(dead_code_log.split("'").count() - 2)
        .expect("Could not get file from dead code log");
    skiplist.contains(file)
}
