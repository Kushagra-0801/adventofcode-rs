# Advent of Code Template

A rust template for Advent of Code.

## Usage

run 
```
AOC_YEAR=2020 AOC_SESSION=<Your session key> AOC_INPUT=<Your input directory> cargo build
```
to set up the project.

`session` and `input dir` can be set later in `.env`.

## Notes

1. The module name is aoc-2020 but the template should work for any year
2. The solution files are part of the same crate and so, their dependencies are shared.
3. `build.rs` is very brittle and made only for the initial state of the repo.
4. The author in `Cargo.toml` is me. Change it if you want.

## Warning
### Do not run `build.rs` after you have started writing your code.

## Future Thoughts

1. Make solutions part of a workspace so they can have independent dependencies.
2. Make `build.rs` more flexible.
