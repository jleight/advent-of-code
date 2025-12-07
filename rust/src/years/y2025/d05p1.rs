pub fn solve(input: &str) -> String {
    let mut fresh = Vec::new();
    let mut count = 0;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        if line.contains('-') {
            let (start, end) = line
                .split_once('-')
                .expect("range should contain a '-'");

            let start = start
                .parse::<u64>()
                .expect("start is not a number");
            let end = end
                .parse::<u64>()
                .expect("end is not a number");

            fresh.push(start..=end);
        } else {
            let id = line
                .parse::<u64>()
                .expect("line is not a number");

            if fresh
                .iter()
                .any(|range| range.contains(&id))
            {
                count += 1;
            }
        }
    }

    count.to_string()
}

#[cfg(test)]
mod tests {
    use crate::aoc::{InputType, Problem};
    use eyre::Result;

    #[test]
    fn test_sample() -> Result<()> {
        let problem = Problem::load(2025, 5)?;

        let input = problem.get_input(1, &InputType::Sample)?;
        let answer = problem.get_answer(1, &InputType::Sample)?;

        assert_eq!(answer, super::solve(&input));

        Ok(())
    }

    #[test]
    fn test_full() -> Result<()> {
        let problem = Problem::load(2025, 5)?;

        let input = problem.get_input(1, &InputType::Full)?;
        let answer = problem.get_answer(1, &InputType::Full)?;

        assert_eq!(answer, super::solve(&input));

        Ok(())
    }
}
