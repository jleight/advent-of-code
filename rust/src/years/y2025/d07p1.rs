use std::collections::BTreeSet;

pub fn solve(input: &str) -> String {
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

    splits.to_string()
}

#[cfg(test)]
mod tests {
    use crate::aoc::{InputType, Problem};
    use eyre::Result;

    #[test]
    fn test_sample() -> Result<()> {
        let problem = Problem::load(2025, 7)?;

        let input = problem.get_input(1, &InputType::Sample)?;
        let answer = problem.get_answer(1, &InputType::Sample)?;

        assert_eq!(answer, super::solve(&input));

        Ok(())
    }

    #[test]
    fn test_full() -> Result<()> {
        let problem = Problem::load(2025, 7)?;

        let input = problem.get_input(1, &InputType::Full)?;
        let answer = problem.get_answer(1, &InputType::Full)?;

        assert_eq!(answer, super::solve(&input));

        Ok(())
    }
}
