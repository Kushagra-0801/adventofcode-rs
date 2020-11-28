use std::convert::TryInto;
use std::num::NonZeroU8;
use std::str::FromStr;

use aoc_2020::*;
use structopt::StructOpt;

#[derive(Debug, Copy, Clone, Eq, PartialEq, StructOpt)]
pub struct Day {
    day: NonZeroU8,
}

impl FromStr for Day {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let day: u8 = s.parse().map_err(|_| "Day needs to be an integer")?;
        if day < 1 || day > 25 {
            Err("Day value needs to be between 1 and 25 (inclusive)")
        } else {
            let day = day.try_into().unwrap();
            Ok(Self { day })
        }
    }
}

impl Day {
    pub fn get(&self) -> u8 {
        self.day.get()
    }

    pub fn get_code(&self) -> &dyn AoCDay {
        /* Add match code */
    }
}
