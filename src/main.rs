use clap::Parser;
use minigrep;
use std::process;

fn main() {
    let args = minigrep::cli::Args::parse();

    match minigrep::app(&args) {
        Ok(matches) => {
            for item in &matches {
                println!("{}", item);
            }
        }
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    }
}
