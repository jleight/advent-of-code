use crate::utils::SolutionContext;

pub fn solve(ctx: &SolutionContext) -> String {
    let mut current = 50i16;
    let mut count = 0u16;

    for (i, line) in ctx.input_lines.iter().enumerate() {
        let direction = if line.starts_with('R') { 1i16 } else { -1 };

        let Ok(steps) = line[1..].parse::<i16>() else {
            panic!("line {i} is invalid: {line}");
        };

        current = (current + (100 + steps * direction)) % 100;

        if current == 0 {
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
        let ctx = SolutionContext::for_problem(2025, 1, 1, true);
        let answer = ctx.answer.clone().unwrap();

        assert_eq!(answer, super::solve(&ctx));
    }

    #[test]
    fn test_full() {
        let ctx = SolutionContext::for_problem(2025, 1, 1, false);
        let answer = ctx.answer.clone().unwrap();

        assert_eq!(answer, super::solve(&ctx));
    }
}
