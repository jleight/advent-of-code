pub fn part_1(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    let rows = lines.len();
    let cols = lines[0].len();

    let mut neighbors = vec![vec![0u8; cols + 2]; rows + 2];

    for (y, line) in lines.iter().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            if cell == '.' {
                continue;
            }

            neighbors[y][x] += 1;
            neighbors[y][x + 1] += 1;
            neighbors[y][x + 2] += 1;
            neighbors[y + 1][x] += 1;
            neighbors[y + 1][x + 2] += 1;
            neighbors[y + 2][x] += 1;
            neighbors[y + 2][x + 1] += 1;
            neighbors[y + 2][x + 2] += 1;
        }
    }

    let mut sum = 0;

    for (y, line) in lines.iter().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            if cell == '@' && neighbors[y + 1][x + 1] < 4 {
                sum += 1;
            }
        }
    }

    sum
}

pub fn part_2(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    let rows = lines.len();
    let cols = lines[0].len();

    let mut floor = vec![vec![false; cols]; rows];

    for (y, line) in lines.iter().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            if cell == '.' {
                continue;
            }

            floor[y][x] = true;
        }
    }

    let mut sum = 0;

    loop {
        let mut changed = false;

        let mut neighbors = vec![vec![0u8; cols + 2]; rows + 2];

        for y in 0..rows {
            for x in 0..cols {
                if !floor[y][x] {
                    continue;
                }

                neighbors[y][x] += 1;
                neighbors[y][x + 1] += 1;
                neighbors[y][x + 2] += 1;
                neighbors[y + 1][x] += 1;
                neighbors[y + 1][x + 2] += 1;
                neighbors[y + 2][x] += 1;
                neighbors[y + 2][x + 1] += 1;
                neighbors[y + 2][x + 2] += 1;
            }
        }

        for y in 0..rows {
            for x in 0..cols {
                if floor[y][x] && neighbors[y + 1][x + 1] < 4 {
                    floor[y][x] = false;
                    sum += 1;
                    changed = true;
                }
            }
        }

        if !changed {
            break;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};
    use crate::aoc::{Result, assert_solution};

    const YEAR: u16 = 2025;
    const DAY: u8 = 4;

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
