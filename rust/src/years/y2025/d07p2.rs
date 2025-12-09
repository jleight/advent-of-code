use std::collections::BTreeMap;

pub fn solve(input: &str) -> String {
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
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::aoc::{InputType, Problem};
    use eyre::Result;

    #[test]
    fn test_sample() -> Result<()> {
        let problem = Problem::load(2025, 7)?;

        let input = problem.get_input(2, &InputType::Sample)?;
        let answer = problem.get_answer(2, &InputType::Sample)?;

        assert_eq!(answer, super::solve(&input));

        Ok(())
    }

    #[test]
    fn test_full() -> Result<()> {
        let problem = Problem::load(2025, 7)?;

        let input = problem.get_input(2, &InputType::Full)?;
        let answer = problem.get_answer(2, &InputType::Full)?;

        assert_eq!(answer, super::solve(&input));

        Ok(())
    }
}
