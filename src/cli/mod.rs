mod base64;
mod csv;
mod genpass;

pub use self::{
    base64::{Base64Format, Base64SubCommand},
    csv::OutputFormat,
};
use self::{csv::CvsOpts, genpass::GenPassOpts};

use anyhow::Ok;
use clap::{command, Parser};
use std::format;

#[derive(Parser, Debug)]
#[command(name="rcli", version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "convert csv file to other format")]
    Csv(CvsOpts),

    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),

    #[command(subcommand)]
    Base64(Base64SubCommand),
}

fn verify_input_file(filename: &str) -> Result<String, anyhow::Error> {
    // if input is "-" or file exists
    if filename == "-" || std::path::Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err(anyhow::Error::msg("File does not exist"))
    }
}

// generate test for verify_input_file
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert!(verify_input_file("-").is_ok());
        assert!(verify_input_file("*").is_err());
        assert!(verify_input_file("Cargo.toml").is_ok());
        assert!(verify_input_file("not-exist").is_err());
    }
}
