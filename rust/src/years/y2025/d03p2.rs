pub fn solve(input: &str) -> String {
    input
        .lines()
        .map(|bank| {
            let bank_len = bank.len();

            let mut stack: Vec<u32> = Vec::new();

            for (i, battery) in bank.chars().enumerate() {
                let battery = battery
                    .to_digit(10)
                    .expect("battery is not a digit");

                let bank_left = bank_len - i;
                let mut stack_needs = 12 - stack.len();

                if bank_left == stack_needs {
                    stack.push(battery);
                    continue;
                }

                while stack
                    .pop_if(|top| *top < battery)
                    .is_some()
                {
                    stack_needs += 1;

                    if bank_left == stack_needs {
                        break;
                    }
                }

                if stack_needs > 0 {
                    stack.push(battery);
                }
            }

            stack
                .iter()
                .fold(String::new(), |a, e| a + &e.to_string())
                .parse::<u128>()
                .unwrap()
        })
        .sum::<u128>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::aoc::{InputType, Problem};
    use eyre::Result;

    #[test]
    fn test_sample() -> Result<()> {
        let problem = Problem::load(2025, 3)?;

        let input = problem.get_input(2, &InputType::Sample)?;
        let answer = problem.get_answer(2, &InputType::Sample)?;

        assert_eq!(answer, super::solve(&input));

        Ok(())
    }

    #[test]
    fn test_full() -> Result<()> {
        let problem = Problem::load(2025, 3)?;

        let input = problem.get_input(2, &InputType::Full)?;
        let answer = problem.get_answer(2, &InputType::Full)?;

        assert_eq!(answer, super::solve(&input));

        Ok(())
    }
}
