use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    #[arg(short, long)]
    pub input: PathBuf,

    #[arg(short, long)]
    pub output: PathBuf,

    #[arg(short, long, default_value_t = 30)]
    pub quality: u8,

    #[arg(short, long)]
    pub threads: Option<usize>,

    #[arg(long, default_value_t = false)]
    pub overwrite: bool,
}
