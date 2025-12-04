use crate::utils::SolutionContext;

pub fn solve(ctx: &SolutionContext) -> String {
    let mut sum = 0;

    for range in ctx.input.split(',') {
        let (start, end) = range.split_once('-').expect("range should contain a '-'");

        let start = start.parse::<u64>().expect("start is not a number");
        let end = end.parse::<u64>().expect("end is not a number");

        for i in start..=end {
            let str = i.to_string();
            let str_len = str.len();

            let half_len = str_len / 2;
            
            let mut success = false;

            for j in 1..=half_len {
                if str_len % j != 0 {
                    continue;
                }

                let pattern = str[0..j].to_string();
                let test = pattern.repeat(str_len / j);

                if str == test {
                    success = true;
                    break;
                }
            }
            
            if success {
                sum += i;
            }
        }
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use crate::utils::SolutionContext;

    #[test]
    fn test_test() {
        let ctx = SolutionContext::for_problem(2025, 2, 2, true);
        let answer = ctx.answer.clone().unwrap();

        assert_eq!(answer, super::solve(&ctx));
    }

    #[test]
    fn test_full() {
        let ctx = SolutionContext::for_problem(2025, 2, 2, false);
        let answer = ctx.answer.clone().unwrap();

        assert_eq!(answer, super::solve(&ctx));
    }
}
