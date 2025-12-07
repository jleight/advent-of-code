pub fn solve(input: &str) -> String {
    let mut sum = 0;

    for range in input.split(',') {
        let (start, end) = range
            .split_once('-')
            .expect("range should contain a '-'");

        let start = start
            .parse::<u64>()
            .expect("start is not a number");
        let end = end
            .parse::<u64>()
            .expect("end is not a number");

        for i in start..=end {
            let str = i.to_string();

            if str.len() % 2 != 0 {
                continue;
            }

            let middle = str.len() / 2;
            let left = &str[..middle];
            let right = &str[middle..];

            if left == right {
                sum += i;
            }
        }
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use crate::aoc::{InputType, Problem};
    use eyre::Result;

    #[test]
    fn test_sample() -> Result<()> {
        let problem = Problem::load(2025, 2)?;

        let input = problem.get_input(1, &InputType::Sample)?;
        let answer = problem.get_answer(1, &InputType::Sample)?;

        assert_eq!(answer, super::solve(&input));

        Ok(())
    }

    #[test]
    fn test_full() -> Result<()> {
        let problem = Problem::load(2025, 2)?;

        let input = problem.get_input(1, &InputType::Full)?;
        let answer = problem.get_answer(1, &InputType::Full)?;

        assert_eq!(answer, super::solve(&input));

        Ok(())
    }
}
