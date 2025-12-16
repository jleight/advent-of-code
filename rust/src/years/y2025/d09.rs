use itertools::Itertools;

pub fn part_1(input: &str) -> u64 {
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
}

pub fn part_2(input: &str) -> u64 {
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
    use super::{part_1, part_2};
    use crate::aoc::{assert_solution, Result};

    const YEAR: u16 = 2025;
    const DAY: u8 = 9;

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
