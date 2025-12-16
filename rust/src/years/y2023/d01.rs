use std::collections::HashMap;
use snafu::OptionExt;
use crate::aoc::{SolutionFailedSnafu, Result};

pub fn part_1(input: &str) -> Result<u32> {
    input
        .lines()
        .map(|line| {
            let mut digits = line
                .chars()
                .filter_map(|c| c.to_digit(10));

            let first = digits.next().context(SolutionFailedSnafu{})?;
            let last = digits.next_back().unwrap_or(first);

            Ok(first * 10 + last)
        })
        .sum()
}

pub fn part_2(input: &str) -> u32 {
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
            let mut first = 0;
            let mut last = 0;

            for i in 0..line.len() {
                if let Some(f) = digits
                    .keys()
                    .find(|w| line[i..].starts_with(*w))
                    .map(|w| digits[w])
                {
                    first = f;
                    break;
                }
            }

            for i in (0..line.len()).rev() {
                if let Some(l) = digits
                    .keys()
                    .find(|w| line[i..].starts_with(*w))
                    .map(|w| digits[w])
                {
                    last = l;
                    break;
                }
            }

            first * 10 + last
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};
    use crate::aoc::{assert_solution, Result};

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
