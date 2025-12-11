use itertools::Itertools;

pub fn solve(input: &str) -> String {
    let red_tiles: Vec<Point> = input
        .lines()
        .map(Point::parse)
        .collect();

    red_tiles
        .iter()
        .tuple_combinations()
        .map(|(a, b)| a.area(b))
        .max()
        .expect("iterator was empty")
        .to_string()
}

#[derive(Debug, PartialEq)]
struct Point {
    x: u64,
    y: u64,
}

impl Point {
    fn parse(line: &str) -> Self {
        let split: Vec<&str> = line.split(',').collect();

        Self {
            x: split
                .first()
                .expect("invalid point: missing x")
                .parse()
                .expect("invalid point: invalid x"),
            y: split
                .get(1)
                .expect("invalid point: missing y")
                .parse()
                .expect("invalid point: invalid y"),
        }
    }

    const fn area(&self, other: &Self) -> u64 {
        let dx = self.x.abs_diff(other.x) + 1;
        let dy = self.y.abs_diff(other.y) + 1;

        dx * dy
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc::{InputType, Problem};
    use eyre::Result;

    #[test]
    fn test_sample() -> Result<()> {
        let problem = Problem::load(2025, 9)?;

        let input = problem.get_input(1, &InputType::Sample)?;
        let answer = problem.get_answer(1, &InputType::Sample)?;

        assert_eq!(answer, super::solve(&input));

        Ok(())
    }

    #[test]
    fn test_full() -> Result<()> {
        let problem = Problem::load(2025, 9)?;

        let input = problem.get_input(1, &InputType::Full)?;
        let answer = problem.get_answer(1, &InputType::Full)?;

        assert_eq!(answer, super::solve(&input));

        Ok(())
    }
}
