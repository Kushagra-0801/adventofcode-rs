use std::path::PathBuf;

use color_eyre::eyre::Result;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Config {
    /// Session Cookie
    #[structopt(long)]
    pub session: String,
    /// Path to the input files.
    #[structopt(long)]
    pub input_path: PathBuf,
}

impl Config {
    pub fn load() -> Result<Self> {
        todo!()
    }

    pub fn save(&self) -> Result<()> {
        todo!()
    }
}
