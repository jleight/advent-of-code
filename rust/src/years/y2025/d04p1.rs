pub fn solve(input: &str) -> String {
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

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use crate::aoc::{InputType, Problem};
    use eyre::Result;

    #[test]
    fn test_sample() -> Result<()> {
        let problem = Problem::load(2025, 4)?;

        let input = problem.get_input(1, &InputType::Sample)?;
        let answer = problem.get_answer(1, &InputType::Sample)?;

        assert_eq!(answer, super::solve(&input));

        Ok(())
    }

    #[test]
    fn test_full() -> Result<()> {
        let problem = Problem::load(2025, 4)?;

        let input = problem.get_input(1, &InputType::Full)?;
        let answer = problem.get_answer(1, &InputType::Full)?;

        assert_eq!(answer, super::solve(&input));

        Ok(())
    }
}
