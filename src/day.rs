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
        match self.day.get() {
            1 => &day1::Code,
            2 => &day2::Code,
            3 => &day3::Code,
            4 => &day4::Code,
            5 => &day5::Code,
            6 => &day6::Code,
            7 => &day7::Code,
            8 => &day8::Code,
            9 => &day9::Code,
            10 => &day10::Code,
            11 => &day11::Code,
            12 => &day12::Code,
            13 => &day13::Code,
            14 => &day14::Code,
            15 => &day15::Code,
            16 => &day16::Code,
            17 => &day17::Code,
            18 => &day18::Code,
            19 => &day19::Code,
            20 => &day20::Code,
            21 => &day21::Code,
            22 => &day22::Code,
            23 => &day23::Code,
            24 => &day24::Code,
            25 => &day25::Code,
            _ => unreachable!(),
        }
    }
}
