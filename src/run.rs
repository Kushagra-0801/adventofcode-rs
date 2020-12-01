use std::env;
use std::fmt::Display;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use color_eyre::eyre::Result;
use structopt::StructOpt;

use crate::day::Day;

#[derive(StructOpt)]
pub struct Run {
    /// Problem day to run
    day: Day,
    /// Path to input file
    #[structopt(long)]
    input: Option<PathBuf>,
    /// Any extra arguments in question
    #[structopt(long)]
    extra: Vec<String>,
    /// Part to run
    #[structopt(long, short, default_value)]
    part: Part,
}

#[derive(StructOpt)]
enum Part {
    Part1,
    Part2,
    Both,
}

impl FromStr for Part {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "part1" | "1" => Ok(Self::Part1),
            "part2" | "2" => Ok(Self::Part2),
            "both" | "b" => Ok(Self::Both),
            _ => Err("Unknown"),
        }
    }
}

impl Default for Part {
    fn default() -> Self {
        Self::Both
    }
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Part::Part1 => write!(f, "part1"),
            Part::Part2 => write!(f, "part2"),
            Part::Both => write!(f, "both"),
        }
    }
}

impl Run {
    pub fn run(&self) -> Result<String> {
        let mut file;
        if let Some(ref path) = self.input {
            file = File::open(path)?;
        } else {
            let path = env::var("AOC_INPUT")?;
            let path = Path::new(&path).join(format!("Day{}", self.day.get()));
            file = File::open(path)?;
        }
        let code = self.day.get_code();
        let extra_args = &self.extra;
        let output = match self.part {
            Part::Part1 => code.part1(&mut file, extra_args),
            Part::Part2 => code.part2(&mut file, extra_args),
            Part::Both => code.both(&mut file, extra_args),
        };
        Ok(output)
    }
}
