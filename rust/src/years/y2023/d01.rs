use crate::aoc::{Result, SolutionFailedSnafu};
use snafu::OptionExt;
use std::collections::HashMap;

pub fn part_1(input: &str) -> Result<u32> {
    input
        .lines()
        .map(|line| {
            let mut digits = line
                .chars()
                .filter_map(|c| c.to_digit(10));

            let first = digits
                .next()
                .context(SolutionFailedSnafu {})?;
            let last = digits
                .next_back()
                .unwrap_or(first);

            Ok(first * 10 + last)
        })
        .sum()
}

pub fn part_2(input: &str) -> Result<i32> {
    let digits = HashMap::from([
        ("1", 1),
        ("one", 1),
        ("2", 2),
        ("two", 2),
        ("3", 3),
        ("three", 3),
        ("4", 4),
        ("four", 4),
        ("5", 5),
        ("five", 5),
        ("6", 6),
        ("six", 6),
        ("7", 7),
        ("seven", 7),
        ("8", 8),
        ("eight", 8),
        ("9", 9),
        ("nine", 9),
    ]);

    input
        .lines()
        .map(|line| {
            let try_get_starting_digit = |i: usize| {
                digits
                    .keys()
                    .find(|w| line[i..].starts_with(*w))
                    .and_then(|w| digits.get(w))
            };

            let first = (0..line.len())
                .find_map(try_get_starting_digit)
                .context(SolutionFailedSnafu {})?;

            let last = (0..line.len())
                .rev()
                .find_map(try_get_starting_digit)
                .context(SolutionFailedSnafu {})?;

            Ok(first * 10 + last)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};
    use crate::aoc::{Result, assert_solution};

    const YEAR: u16 = 2023;
    const DAY: u8 = 1;

    #[test]
    fn part_1_sample_a() -> Result<()> {
        assert_solution!(part_1, "a");
        Ok(())
    }

    #[test]
    fn part_1_full() -> Result<()> {
        assert_solution!(part_1, "full");
        Ok(())
    }

    #[test]
    fn part_2_sample_b() -> Result<()> {
        assert_solution!(part_2, "b");
        Ok(())
    }

    #[test]
    fn part_2_full() -> Result<()> {
        assert_solution!(part_2, "full");
        Ok(())
    }
}
