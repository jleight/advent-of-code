use crate::aoc::Error::UnexpectedInput;
use crate::aoc::{FailedToParseInputSnafu, IntoResult, Result, UnexpectedInputSnafu};
use snafu::{ensure, OptionExt, ResultExt};
use std::str::FromStr;

pub fn part_1(input: &str) -> Result<u32> {
    parse(input)?
        .iter()
        .filter_map(|g| if g.is_valid() { Some(g.id) } else { None })
        .sum::<u32>()
        .into_result()
}

pub fn part_2(input: &str) -> Result<u32> {
    parse(input)?
        .iter()
        .map(Game::power)
        .sum::<u32>()
        .into_result()
}

fn parse(input: &str) -> Result<Vec<Game>> {
    input.lines().map(Game::from_str).collect()

}

#[derive(Debug)]
struct Reveal {
    red: u32,
    green: u32,
    blue: u32,
}

impl FromStr for Reveal {
    type Err = crate::aoc::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for cube_count in s.split(',') {
            let split: Vec<&str> = cube_count
                .split_whitespace()
                .collect();

            ensure!(split.len() == 2, UnexpectedInputSnafu {
                expected: "[count] [color]",
                got: cube_count,
            });

            let count = split[0].parse::<u32>().context(FailedToParseInputSnafu{
                expected: "[count]",
                got: split[0],
            })?;

            match split[1] {
                "red" => red += count,
                "green" => green += count,
                "blue" => blue += count,
                _ => return Err(UnexpectedInput{
                    expected: "red, green, or blue",
                    got: split[1].to_string(),
                }),
            }
        }

        Ok(Self { red, green, blue })
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    reveals: Vec<Reveal>,
}

impl FromStr for Game {
    type Err = crate::aoc::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game, reveals) = s.split_once(':').context(UnexpectedInputSnafu{
            expected: "Game [id]: [reveals...]",
            got: s,
        })?;

        let id = game
            .replace("Game ", "")
            .parse::<u32>()
            .context(FailedToParseInputSnafu {
                expected: "Game [id]",
                got: game,
            })?;
        let reveals: Result<_, _> = reveals
            .split("; ")
            .map(Reveal::from_str)
            .collect();

        Ok(Self { id, reveals: reveals? })
    }
}

impl Game {
    fn is_valid(&self) -> bool {
        self.reveals
            .iter()
            .all(|r| r.red <= 12 && r.green <= 13 && r.blue <= 14)
    }

    fn power(&self) -> u32 {
        let min_red = self
            .reveals
            .iter()
            .map(|r| r.red)
            .max()
            .unwrap_or(1);
        let min_green = self
            .reveals
            .iter()
            .map(|r| r.green)
            .max()
            .unwrap_or(1);
        let min_blue = self
            .reveals
            .iter()
            .map(|r| r.blue)
            .max()
            .unwrap_or(1);

        min_red * min_green * min_blue
    }
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};
    use crate::aoc::{assert_solution, Result};

    const YEAR: u16 = 2023;
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
