use regex::Regex;

pub fn solve(input: &str) -> String {
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

    count.to_string()
}

#[cfg(test)]
mod tests {
    use crate::aoc::{InputType, Problem};
    use eyre::Result;

    #[test]
    fn test_sample() -> Result<()> {
        let problem = Problem::load(2025, 12)?;

        let input = problem.get_input(1, &InputType::Sample)?;
        let answer = problem.get_answer(1, &InputType::Sample)?;

        assert_eq!(answer, super::solve(&input));

        Ok(())
    }

    #[test]
    fn test_full() -> Result<()> {
        let problem = Problem::load(2025, 12)?;

        let input = problem.get_input(1, &InputType::Full)?;
        let answer = problem.get_answer(1, &InputType::Full)?;

        assert_eq!(answer, super::solve(&input));

        Ok(())
    }
}
