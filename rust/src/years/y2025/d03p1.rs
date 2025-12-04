use crate::utils::SolutionContext;

pub fn solve(ctx: &SolutionContext) -> String {
    ctx.input_lines
        .iter()
        .map(|bank| {
            let a = bank
                .chars()
                .map(|c| c.to_digit(10).expect("char is not a digit"))
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
                .map(|c| c.to_digit(10).expect("char is not a digit"))
                .max()
                .expect("rest is empty");

            if swap {
                b * 10 + a
            } else {
                a * 10 + b
            }
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::utils::SolutionContext;

    #[test]
    fn test_test() {
        let ctx = SolutionContext::for_problem(2025, 3, 1, true);
        let answer = ctx.answer.clone().unwrap();

        assert_eq!(answer, super::solve(&ctx));
    }

    #[test]
    fn test_full() {
        let ctx = SolutionContext::for_problem(2025, 3, 1, false);
        let answer = ctx.answer.clone().unwrap();

        assert_eq!(answer, super::solve(&ctx));
    }
}
