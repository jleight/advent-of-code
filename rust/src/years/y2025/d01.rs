use crate::aoc::Error::UnexpectedInput;
use crate::aoc::{FailedToParseInputSnafu, IntoResult, Result};
use snafu::ResultExt;

pub fn part_1(input: &str) -> Result<u16> {
    parse(input)?
        .iter()
        .fold((50, 0), |(current, count), (direction, steps)| {
            let direction = match direction {
                Direction::Right => 1i16,
                Direction::Left => -1,
            };

            let current = (current + (100 + steps * direction)) % 100;
            let count = if current == 0 { count + 1 } else { count };

            (current, count)
        })
        .1
        .into_result()
}

pub fn part_2(input: &str) -> Result<i32> {
    parse(input)?
        .iter()
        .fold((50, 0), |(current, count), (direction, steps)| {
            let (mut current, mut count) = match direction {
                Direction::Right => (current + steps, count),
                Direction::Left => (current - steps, count),
            };
            
            while current >= 100 {
                current -= 100;
                count += 1;
            }

            while current < 0 {
                current += 100;
                count += 1;
            }

            (current, count)
        })
        .1
        .into_result()
}

fn parse(input: &str) -> Result<Vec<(Direction, i16)>> {
    input
        .lines()
        .map(|line| {
            let direction = match line.chars().next() {
                Some('R') => Direction::Right,
                Some('L') => Direction::Left,
                _ => {
                    return Err(UnexpectedInput {
                        expected: "red, green, or blue",
                        got: line
                            .chars()
                            .next()
                            .unwrap_or(' ')
                            .to_string(),
                    });
                }
            };

            let steps = line[1..]
                .parse::<i16>()
                .context(FailedToParseInputSnafu {
                    expected: "[RL]\\d+",
                    got: line,
                })?;

            Ok((direction, steps))
        })
        .collect()
}

enum Direction {
    Right,
    Left,
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};
    use crate::aoc::{Result, assert_solution};

    const YEAR: u16 = 2025;
    const DAY: u8 = 1;

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
