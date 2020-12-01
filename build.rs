use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;

const SOLUTION: &str = "use crate::AoCDay;

pub struct Code;

impl AoCDay for Code {
    fn part1(&self, _input: &mut dyn std::io::Read, _extra_argss: &[String]) -> String {
        todo!()
    }

    fn part2(&self, _input: &mut dyn std::io::Read, _extra_args: &[String]) -> String {
        todo!()
    }
}
";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    add_year_in_init();
    session_and_input_dir();
    set_up_solution_files();
    set_up_lib_file();
    set_up_day_file();
}

fn set_up_solution_files() {
    fs::create_dir_all("src/solutions/").expect("Cannot create solution directory");
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
    let input_dir = option_env!("AOC_INPUT").unwrap_or_else(|| {
        println!("Note: Using `inputs` directory in project root");
        "inputs"
    });
    std::fs::create_dir_all(input_dir).expect("Failed to create input directory");
    let env = format!("AOC_SESSION={}\nAOC_INPUT={}", session, input_dir);
    fs::write(".env", env).expect("Failed to create .env file");
}

fn add_year_in_init() {
    // let year = env!("AOC_YEAR");
    let year = "2020";
    assert!(
        year.chars().all(char::is_numeric),
        "Year should be a number"
    );
    let contents = fs::read_to_string("src/init.rs").expect("Need to open src/init.rs");
    let contents = contents.replace("0xFFFF", &format!("{}", year));
    fs::write("src/init.rs", contents).expect("Cannot write to src/init.rs");
}
