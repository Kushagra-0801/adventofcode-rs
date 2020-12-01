use std::io::Read;

/// Trait for every solution
///
/// All methods take a `dyn Read`. It can be given a file object (actual input) or a string (test cases)
///
/// Extra args are for cases where there are extra details in the question that are not part of the input itself
pub trait AoCDay {
    fn part1(&self, input: &mut dyn Read, extra_args: &[String]) -> String;
    fn part2(&self, input: &mut dyn Read, extra_args: &[String]) -> String;
    /// This method should be implemented if solving both parts together is more efficient than doing them one at a time
    fn both(&self, input: &mut dyn Read, extra_args: &[String]) -> String {
        let p1 = self.part1(input, extra_args);
        let p2 = self.part2(input, extra_args);
        format!(
            "Part1: {}\n\
            Part2: {}",
            p1, p2
        )
    }
}
