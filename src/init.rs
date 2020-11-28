use std::env;
use std::path::Path;

use color_eyre::eyre::Result;
use structopt::StructOpt;

use crate::day::Day;

const YEAR: u16 = 0xFFFF;

#[derive(StructOpt, Debug)]
pub struct Init {
    day: Day,
}

impl Init {
    pub fn initialize(&self) -> Result<()> {
        let input_path = env::var("AOC_INPUT")?;
        let input_path = Path::new(&input_path).join(format!("Day{}", self.day.get()));
        if input_path.is_file() {
            Ok(())
        } else {
            let session = env::var("AOC_SESSION")?;
            let client = reqwest::blocking::Client::builder().gzip(true).build()?;
            let mut response = client
                .get(&self.input_url())
                .header(reqwest::header::COOKIE, format!("session={}", session))
                .send()?
                .error_for_status()?;
            if let Some(parent) = input_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let mut file = std::fs::File::create(input_path)?;
            response.copy_to(&mut file)?;
            Ok(())
        }
    }

    fn input_url(&self) -> String {
        format!(
            "https://adventofcode.com/{}/day/{}/input",
            YEAR,
            self.day.get()
        )
    }
}
