use anyhow::{Error, Ok};
use clap::{command, Parser};

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
}

#[derive(Debug, Parser)]
pub struct CvsOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,
    #[arg(short, long, default_value = "output.json")]
    pub output: String,
    #[arg(short, long, default_value = ",")]
    pub delimiter: char,
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn verify_input_file(filename: &str) -> Result<String, Error> {
    if std::path::Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err(anyhow::anyhow!("File not found"))
    }
}
