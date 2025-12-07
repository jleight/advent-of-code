pub fn solve(input: &str) -> String {
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

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use crate::aoc::{InputType, Problem};
    use eyre::Result;

    #[test]
    fn test_sample() -> Result<()> {
        let problem = Problem::load(2025, 4)?;

        let input = problem.get_input(2, &InputType::Sample)?;
        let answer = problem.get_answer(2, &InputType::Sample)?;

        assert_eq!(answer, super::solve(&input));

        Ok(())
    }

    #[test]
    fn test_full() -> Result<()> {
        let problem = Problem::load(2025, 4)?;

        let input = problem.get_input(2, &InputType::Full)?;
        let answer = problem.get_answer(2, &InputType::Full)?;

        assert_eq!(answer, super::solve(&input));

        Ok(())
    }
}
