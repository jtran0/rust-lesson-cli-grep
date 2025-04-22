use clap::{self, Parser};
use std::fs;
use std::path;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    pub pattern: String,
    #[arg(value_parser = validate_filepath)]
    pub filepath: path::PathBuf,
}

fn validate_filepath(arg: &str) -> Result<path::PathBuf, String> {
    fs::File::open(arg).map_err(|err| err.to_string())?;
    Ok(arg.into())
}
