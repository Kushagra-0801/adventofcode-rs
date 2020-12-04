# Advent of Code Template

A [rust](https://www.rust-lang.org/) template for [Advent of Code](https://adventofcode.com).

## Initial Setup

1. Clone this repo.
2. Setup your environment:
    * `export AOC_YEAR=<year>`
    * `export AOC_SESSION=<Your session key>`
    * `export AOC_INPUT=<Your input directory>`
3. Run _**ONCE**_: `cargo build`
4. Delete `build.rs` - you don't need to build again.

## Daily Challenges

1. Download the day's file: `cargo run -- init <day>`.  The file will be downloaded into the `AOC_INPUT` directory.
2. Write your code in the "day" files in `src/solutions/dayX.rs`
3. Build and run your code: `cargo run -- run <day> [--part <part>] [--extra arg1 arg2 arg3] [--input path/to/input]`
    * The arguments within [ ] are optional.
    * Skipping `--part` will run both part 1 and part 2.
    * Skipping `--extra` will give an empty `Vec<T>` of extra args.
    * Skipping `--input` will look for the download input in `AOC_INPUT`.

## Notes

1. The module name is aoc-2020 but the template should work for any year
2. The solution files are part of the same crate and so, their dependencies are shared.
3. `build.rs` is very brittle and made only for the initial state of the repo.
4. The author in `Cargo.toml` is me. Change it if you want.

## Warning!
_**Do not run `build.rs` after you have started writing your code.**_

