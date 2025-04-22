pub mod cli;

use std::error;
use std::fs;
use std::io;

pub fn app(args: &cli::Args) -> Result<Vec<String>, Box<dyn error::Error>> {
    let file = fs::File::open(&args.filepath).unwrap();
    let reader = io::BufReader::new(file);
    return grep_string_stream(&args.pattern, reader);
}

fn grep_string_stream<T>(pattern: &str, reader: T) -> Result<Vec<String>, Box<dyn error::Error>>
where
    T: io::BufRead,
{
    let mut matches: Vec<String> = Vec::new();

    if !pattern.is_empty() {
        for line in reader.lines().filter_map(Result::ok) {
            if line.contains(pattern) {
                matches.push(line);
            }
        }
    }

    return Ok(matches);
}
