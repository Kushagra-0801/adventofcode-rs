use std::io::Read;

/// Trait for every solution
///
/// All methods take a `dyn Read`. It can be given a file object (actual input) or a string (test cases)
///
/// Extra args are for cases where there are extra details in the question that are not part of the input itself
pub trait AoCDay {
    fn part1(&self, input: &dyn Read, extra_args: &[String]) -> String;
    fn part2(&self, input: &dyn Read, extra_args: &[String]) -> String;
    /// This method should be implemented if solving both parts together is more efficient than doing them one at a time
    fn both(&self, input: &dyn Read, extra_args: &[String]) -> String {
        let p1 = self.part1(input, extra_args);
        let p2 = self.part2(input, extra_args);
        format!(
            "Part1: {}\n\
            Part2: {}",
            p1, p2
        )
    }
}

pub mod solutions {
    pub mod day1;
    pub mod day10;
    pub mod day11;
    pub mod day12;
    pub mod day13;
    pub mod day14;
    pub mod day15;
    pub mod day16;
    pub mod day17;
    pub mod day18;
    pub mod day19;
    pub mod day2;
    pub mod day20;
    pub mod day21;
    pub mod day22;
    pub mod day23;
    pub mod day24;
    pub mod day25;
    pub mod day3;
    pub mod day4;
    pub mod day5;
    pub mod day6;
    pub mod day7;
    pub mod day8;
    pub mod day9;
}

pub use solutions::*;
