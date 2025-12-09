pub fn solve(input: &str) -> String {
    input
        .lines()
        .map(|bank| {
            let a = bank
                .chars()
                .map(|c| {
                    c.to_digit(10)
                        .expect("char is not a digit")
                })
                .max()
                .expect("bank is empty");

            let ai = bank
                .chars()
                .position(|c| c.to_digit(10).unwrap() == a)
                .unwrap();

            let (rest, swap) = if ai == bank.len() - 1 {
                (bank[..ai].to_string(), true)
            } else {
                (bank[ai + 1..].to_string(), false)
            };

            let b = rest
                .chars()
                .map(|c| {
                    c.to_digit(10)
                        .expect("char is not a digit")
                })
                .max()
                .expect("rest is empty");

            if swap { b * 10 + a } else { a * 10 + b }
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::aoc::{InputType, Problem};
    use eyre::Result;

    #[test]
    fn test_sample() -> Result<()> {
        let problem = Problem::load(2025, 3)?;

        let input = problem.get_input(1, &InputType::Sample)?;
        let answer = problem.get_answer(1, &InputType::Sample)?;

        assert_eq!(answer, super::solve(&input));

        Ok(())
    }

    #[test]
    fn test_full() -> Result<()> {
        let problem = Problem::load(2025, 3)?;

        let input = problem.get_input(1, &InputType::Full)?;
        let answer = problem.get_answer(1, &InputType::Full)?;

        assert_eq!(answer, super::solve(&input));

        Ok(())
    }
}
