use std::collections::{BTreeMap, BTreeSet};

pub fn part_1(input: &str) -> i32 {
    let mut beams = BTreeSet::new();
    let mut splits = 0;

    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            if c == 'S' {
                beams.insert(i);
            } else if c == '^' && beams.remove(&i) {
                splits += 1;
                beams.insert(i - 1);
                beams.insert(i + 1);
            }
        }
    }

    splits
}

pub fn part_2(input: &str) -> u64 {
    let mut beams = BTreeMap::new();

    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            if c == 'S' {
                beams.insert(i, 1);
            } else if c == '^'
                && let Some(count) = beams.remove(&i)
            {
                let left_count = *beams
                    .entry(i - 1)
                    .or_default()
                    + count;
                beams.insert(i - 1, left_count);

                let right_count = *beams
                    .entry(i + 1)
                    .or_default()
                    + count;
                beams.insert(i + 1, right_count);
            }
        }
    }

    beams
        .values()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};
    use crate::aoc::{assert_solution, Result};

    const YEAR: u16 = 2025;
    const DAY: u8 = 7;

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
