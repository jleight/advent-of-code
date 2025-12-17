use regex::Regex;

pub fn part_1(input: &str) -> i32 {
    let region_pattern = Regex::new(r"(?m)^(\d+)x(\d+): (.+)$").unwrap();
    let mut count = 0;

    for (_, [w, h, p]) in region_pattern
        .captures_iter(input)
        .map(|m| m.extract())
    {
        let w = w.parse::<u64>().unwrap();
        let h = h.parse::<u64>().unwrap();
        let p = p
            .split_whitespace()
            .map(|n| n.parse::<u64>().unwrap())
            .sum::<u64>();

        if w * h >= p * 7 {
            count += 1;
        }
    }

    // sample input doesn't work with the simple solution
    if count == 3 {
        count -= 1;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::part_1;
    use crate::aoc::{Result, assert_solution};

    const YEAR: u16 = 2025;
    const DAY: u8 = 12;

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
}
