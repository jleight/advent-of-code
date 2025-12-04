use crate::utils::SolutionContext;

pub fn solve(ctx: &SolutionContext) -> String {
    ctx.input_lines
        .iter()
        .map(|bank| {
            let bank_len = bank.len();

            let mut stack: Vec<u32> = Vec::new();

            for (i, battery) in bank.chars().enumerate() {
                let battery = battery.to_digit(10).expect("battery is not a digit");

                let bank_left = bank_len - i;
                let mut stack_needs = 12 - stack.len();

                if bank_left == stack_needs {
                    stack.push(battery);
                    continue;
                }

                while stack.pop_if(|top| *top < battery).is_some() {
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
    use crate::utils::SolutionContext;

    #[test]
    fn test_test() {
        let ctx = SolutionContext::for_problem(2025, 3, 2, true);
        let answer = ctx.answer.clone().unwrap();

        assert_eq!(answer, super::solve(&ctx));
    }

    #[test]
    fn test_full() {
        let ctx = SolutionContext::for_problem(2025, 3, 2, false);
        let answer = ctx.answer.clone().unwrap();

        assert_eq!(answer, super::solve(&ctx));
    }
}
