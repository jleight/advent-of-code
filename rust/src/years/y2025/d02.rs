use crate::aoc::Result;
use crate::aoc::{FailedToParseInputSnafu, IntoResult, UnexpectedInputSnafu};
use snafu::{OptionExt, ResultExt};
use std::ops::RangeInclusive;

pub fn part_1(input: &str) -> Result<u64> {
    parse(input)?
        .filter(|(_, s)| s.len() % 2 == 0)
        .filter_map(|(i, s)| {
            let middle = s.len() / 2;
            let left = &s[..middle];
            let right = &s[middle..];

            if left == right { Some(i) } else { None }
        })
        .sum::<u64>()
        .into_result()
}

pub fn part_2(input: &str) -> Result<u64> {
    parse(input)?
        .filter_map(|(i, s)| {
            let len = s.len();
            let half_len = len / 2;

            for j in 1..=half_len {
                if len % j != 0 {
                    continue;
                }

                let pattern = s[0..j].to_string();
                let test = pattern.repeat(len / j);

                if s == test {
                    return Some(i);
                }
            }

            None
        })
        .sum::<u64>()
        .into_result()
}

fn parse(input: &str) -> Result<impl Iterator<Item = (u64, String)>> {
    input
        .split(',')
        .map(|range| {
            let (start, end) = range
                .split_once('-')
                .context(UnexpectedInputSnafu {
                    expected: "[start]-[end]",
                    got: range,
                })?;

            let start = start
                .parse::<u64>()
                .context(FailedToParseInputSnafu {
                    expected: "\\d+",
                    got: start,
                })?;
            let end = end
                .parse::<u64>()
                .context(FailedToParseInputSnafu {
                    expected: "\\d+",
                    got: end,
                })?;

            Ok::<RangeInclusive<u64>, crate::aoc::Error>(start..=end)
        })
        .flat_map(Result::into_iter)
        .flat_map(RangeInclusive::into_iter)
        .map(|i| (i, i.to_string()))
        .into_result()
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};
    use crate::aoc::{Result, assert_solution};

    const YEAR: u16 = 2025;
    const DAY: u8 = 2;

    #[test]
    fn part_1_sample() -> Result<()> {
        assert_solution!(part_1, "sample");
        Ok(())
    }

    #[test]
    fn part_1_full() -> Result<()> {
        assert_solution!(part_1, "full");
        Ok(())
    }

    #[test]
    fn part_2_sample() -> Result<()> {
        assert_solution!(part_2, "sample");
        Ok(())
    }

    #[test]
    fn part_2_full() -> Result<()> {
        assert_solution!(part_2, "full");
        Ok(())
    }
}
