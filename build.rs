#![allow(unused)]

use std::fs;
use std::fs::OpenOptions;
use std::io::{self, prelude::*};

const SOLUTION: &str = "use crate::AoCDay;

pub struct Code;

impl AoCDay for Code {
    fn part1(&self, _input: &dyn std::io::Read, _extra_argss: &[String]) -> String {
        todo!()
    }

    fn part2(&self, _input: &dyn std::io::Read, _extra_args: &[String]) -> String {
        todo!()
    }
}
";

fn main() {
    // println!("cargo:rerun-if-changed=src/main.rs");
    // println!("cargo:rerun-if-changed=src/lib.rs");

    let (year, author) = year_and_author();
    session_and_input_dir();
    set_up_solution_files();
    set_up_lib_file();
    set_up_day_file();
}

fn year_and_author() -> (u16, &'static str) {
    let year = env!("AOC_YEAR", "Need a year")
        .parse()
        .expect("Year should be a number");
    let author = env!("AOC_AUTHOR", "Need an author");
    (year, author)
}

fn set_up_solution_files() {
    fs::create_dir_all("src/solutions/");
    for day in 1..=25 {
        let path = format!("src/solutions/day{}.rs", day);
        fs::write(path, SOLUTION).expect("Failed to create solution file");
    }
}

fn set_up_lib_file() {
    let mut file = OpenOptions::new()
        .append(true)
        .open("src/lib.rs")
        .expect("Failed to open src/lib.rs");
    writeln!(file, "\npub mod solutions {{").expect("Failed to define solution module");
    for day in 1..=25 {
        writeln!(file, "    pub mod day{};", day).expect("Failed to define solution module");
    }
    writeln!(
        file,
        "}}

pub use solutions::*;"
    )
    .expect("Failed to define solution module");
}

fn set_up_day_file() {
    let contents = fs::read_to_string("src/day.rs").expect("src/day.rs cannot be opened");
    let mut match_code = String::from("match self.day.get() {\n");
    for day in 1..=25 {
        match_code.push_str(&format!("{0} => &day{0}::Code,\n", day))
    }
    match_code.push_str("_ => unreachable!(),\n}");
    let contents = contents.replace("/* Add match code */", &match_code);
    fs::write("src/day.rs", contents).expect("Cannot write to src/day.rs");
}

fn session_and_input_dir() {
    let session = option_env!("AOC_SESSION").unwrap_or_else(|| {
        println!("NOTE: Session key needs to be added in .env");
        ""
    });
    todo!()
}
