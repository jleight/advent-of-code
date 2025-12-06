use crate::utils::SolutionContext;

pub fn solve(ctx: &SolutionContext) -> String {
    let mut fresh = Vec::new();
    let mut count = 0;

    for line in ctx.input_lines.iter() {
        if line.is_empty() {
            continue;
        }

        if line.contains('-') {
            let (start, end) = line.split_once('-').expect("range should contain a '-'");

            let start = start.parse::<u64>().expect("start is not a number");
            let end = end.parse::<u64>().expect("end is not a number");

            fresh.push(start..=end);
        } else {
            let id = line.parse::<u64>().expect("line is not a number");

            if fresh.iter().find(|range| range.contains(&id)).is_some() {
                count += 1;
            }
        }
    }

    count.to_string()
}

#[cfg(test)]
mod tests {
    use crate::utils::SolutionContext;

    #[test]
    fn test_test() {
        let ctx = SolutionContext::for_problem(2025, 5, 1, true);
        let answer = ctx.answer.clone().unwrap();

        assert_eq!(answer, super::solve(&ctx));
    }

    #[test]
    fn test_full() {
        let ctx = SolutionContext::for_problem(2025, 5, 1, false);
        let answer = ctx.answer.clone().unwrap();

        assert_eq!(answer, super::solve(&ctx));
    }
}
