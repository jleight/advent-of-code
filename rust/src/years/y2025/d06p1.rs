use std::collections::BTreeMap;

pub fn solve(input: &str) -> String {
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
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::aoc::{InputType, Problem};
    use eyre::Result;

    #[test]
    fn test_sample() -> Result<()> {
        let problem = Problem::load(2025, 6)?;

        let input = problem.get_input(1, &InputType::Sample)?;
        let answer = problem.get_answer(1, &InputType::Sample)?;

        assert_eq!(answer, super::solve(&input));

        Ok(())
    }

    #[test]
    fn test_full() -> Result<()> {
        let problem = Problem::load(2025, 6)?;

        let input = problem.get_input(1, &InputType::Full)?;
        let answer = problem.get_answer(1, &InputType::Full)?;

        assert_eq!(answer, super::solve(&input));

        Ok(())
    }
}
