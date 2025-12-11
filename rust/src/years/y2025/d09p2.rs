use itertools::Itertools;

pub fn solve(input: &str) -> String {
    let red_tiles: Vec<Point> = input
        .lines()
        .map(Point::parse)
        .collect();

    let edges: Vec<(&Point, &Point)> = red_tiles
        .iter()
        .circular_tuple_windows()
        .collect();

    red_tiles
        .iter()
        .tuple_combinations()
        .map(|(a, b)| ((a, b), a.area(b)))
        .sorted_by_key(|x| x.1)
        .rev()
        .find(|((a, b), _)| {
            edges
                .iter()
                .all(|(s, e)| {
                    a.x.max(b.x) <= s.x.min(e.x)
                        || a.x.min(b.x) >= s.x.max(e.x)
                        || a.y.max(b.y) <= s.y.min(e.y)
                        || a.y.min(b.y) >= s.y.max(e.y)
                })
        })
        .expect("no solution found")
        .1
        .to_string()
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i64,
    y: i64,
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

        let input = problem.get_input(2, &InputType::Sample)?;
        let answer = problem.get_answer(2, &InputType::Sample)?;

        assert_eq!(answer, super::solve(&input));

        Ok(())
    }

    #[test]
    fn test_full() -> Result<()> {
        let problem = Problem::load(2025, 9)?;

        let input = problem.get_input(2, &InputType::Full)?;
        let answer = problem.get_answer(2, &InputType::Full)?;

        assert_eq!(answer, super::solve(&input));

        Ok(())
    }
}
