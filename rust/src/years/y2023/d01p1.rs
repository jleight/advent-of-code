use crate::utils::SolutionContext;

pub fn solve(ctx: &SolutionContext) -> String {
    ctx.input_lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            let mut digits = line.chars().filter(char::is_ascii_digit);

            let Some(first) = digits.next().and_then(|c| c.to_digit(10)) else {
                panic!("line {i} does not contain any digits: {line}");
            };

            let last = digits.next_back().and_then(|c| c.to_digit(10)).unwrap_or(first);

            first * 10 + last
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::utils::SolutionContext;

    #[test]
    fn test_test() {
        let ctx = SolutionContext::for_problem(2023, 1, 1, true);
        let answer = ctx.answer.clone().unwrap();

        assert_eq!(answer, super::solve(&ctx));
    }

    #[test]
    fn test_full() {
        let ctx = SolutionContext::for_problem(2023, 1, 1, false);
        let answer = ctx.answer.clone().unwrap();

        assert_eq!(answer, super::solve(&ctx));
    }
}
