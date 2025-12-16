pub fn part_1(input: &str) -> u16 {
    let mut current = 50i16;
    let mut count = 0u16;

    for (i, line) in input.lines().enumerate() {
        let direction = if line.starts_with('R') { 1i16 } else { -1 };

        let Ok(steps) = line[1..].parse::<i16>() else {
            panic!("line {i} is invalid: {line}");
        };

        current = (current + (100 + steps * direction)) % 100;

        if current == 0 {
            count += 1;
        }
    }

    count
}

pub fn part_2(input: &str) -> i32 {
    let mut current = 50;
    let mut count = 0;

    for (i, line) in input.lines().enumerate() {
        let Ok(steps) = line[1..].parse::<i32>() else {
            panic!("line {i} is invalid: {line}");
        };

        current = if line.starts_with('R') {
            current + steps
        } else {
            current - steps
        };

        while current >= 100 {
            current -= 100;
            count += 1;
        }

        while current < 0 {
            current += 100;
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};
    use crate::aoc::{assert_solution, Result};

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
