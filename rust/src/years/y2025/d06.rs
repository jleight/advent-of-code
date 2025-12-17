use std::collections::BTreeMap;

pub fn part_1(input: &str) -> u64 {
    input
        .lines()
        .fold(BTreeMap::new(), |mut a, e| {
            let split = e.split_whitespace();

            for (i, s) in split.enumerate() {
                let t = a
                    .entry(i)
                    .or_insert((0, 1));

                if let Ok(number) = s.parse::<u64>() {
                    t.0 += number;
                    t.1 *= number;
                } else if s == "*" {
                    t.0 = t.1;
                }
            }

            a
        })
        .iter()
        .map(|(_, (v, _))| v)
        .sum()
}

pub fn part_2(input: &str) -> u64 {
    let mut total = 0;

    let mut sum = 0;
    let mut product = 1;

    let cols = input
        .lines()
        .max_by(|a, b| a.len().cmp(&b.len()))
        .expect("no input lines found")
        .len();

    for i in 0..cols {
        let i = cols - i - 1;

        let col: String = input
            .lines()
            .map(|l| {
                l.chars()
                    .nth(i)
                    .unwrap_or(' ')
            })
            .collect();

        let mut col = col.trim();

        if col.is_empty() {
            continue;
        }

        let operation = if col.ends_with('+') {
            col = col
                .trim_end_matches('+')
                .trim_end();
            Some(Operation::Add)
        } else if col.ends_with('*') {
            col = col
                .trim_end_matches('*')
                .trim_end();
            Some(Operation::Multiply)
        } else {
            None
        };

        let number = col
            .parse::<u64>()
            .expect("failed to parse number");

        sum += number;
        product *= number;

        match operation {
            Some(Operation::Add) => total += sum,
            Some(Operation::Multiply) => total += product,
            None => continue,
        }

        sum = 0;
        product = 1;
    }

    total
}

enum Operation {
    Add,
    Multiply,
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};
    use crate::aoc::{Result, assert_solution};

    const YEAR: u16 = 2025;
    const DAY: u8 = 6;

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
