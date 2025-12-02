use crate::utils::SolutionContext;

pub fn solve(ctx: &SolutionContext) -> String {
    let mut current = 50;
    let mut count = 0;

    for (i, line) in ctx.input_lines.iter().enumerate() {
        let Ok(steps) = line[1..].parse::<i32>() else {
            panic!("line {i} is invalid: {line}");
        };

        current = if line.starts_with('R') {
            current + steps
        } else {
            current - steps
        };

        while current >= 100 {
            current -= 100;
            count += 1;
        }

        while current < 0 {
            current += 100;
            count += 1;
        }
    }

    count.to_string()
}

#[cfg(test)]
mod tests {
    use crate::utils::SolutionContext;

    #[test]
    fn test_test() {
        let ctx = SolutionContext::for_problem(2025, 1, 2, true);
        let answer = ctx.answer.clone().unwrap();

        assert_eq!(answer, super::solve(&ctx));
    }

    #[test]
    fn test_full() {
        let ctx = SolutionContext::for_problem(2025, 1, 2, false);
        let answer = ctx.answer.clone().unwrap();

        assert_eq!(answer, super::solve(&ctx));
    }
}
