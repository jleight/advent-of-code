use crate::utils::SolutionContext;
use std::collections::HashMap;

pub fn solve(ctx: &SolutionContext) -> String {
    let digits = HashMap::from([
        ("1", 1),
        ("one", 1),
        ("2", 2),
        ("two", 2),
        ("3", 3),
        ("three", 3),
        ("4", 4),
        ("four", 4),
        ("5", 5),
        ("five", 5),
        ("6", 6),
        ("six", 6),
        ("7", 7),
        ("seven", 7),
        ("8", 8),
        ("eight", 8),
        ("9", 9),
        ("nine", 9),
    ]);

    ctx.input_lines
        .iter()
        .map(|line| {
            let mut first = 0;
            let mut last = 0;

            for i in 0..line.len() {
                if let Some(f) = digits
                    .keys()
                    .find(|w| line[i..].starts_with(*w))
                    .map(|w| digits[w])
                {
                    first = f;
                    break;
                }
            }

            for i in (0..line.len()).rev() {
                if let Some(l) = digits
                    .keys()
                    .find(|w| line[i..].starts_with(*w))
                    .map(|w| digits[w])
                {
                    last = l;
                    break;
                }
            }

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
        let ctx = SolutionContext::for_problem(2023, 1, 2, true);
        let answer = ctx.answer.clone().unwrap();

        assert_eq!(answer, super::solve(&ctx));
    }

    #[test]
    fn test_full() {
        let ctx = SolutionContext::for_problem(2023, 1, 2, false);
        let answer = ctx.answer.clone().unwrap();

        assert_eq!(answer, super::solve(&ctx));
    }
}
