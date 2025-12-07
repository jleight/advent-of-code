pub fn solve(input: &str) -> String {
    let mut current = 50;
    let mut count = 0;

    for (i, line) in input.lines().enumerate() {
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
    use crate::aoc::{InputType, Problem};
    use eyre::Result;

    #[test]
    fn test_sample() -> Result<()> {
        let problem = Problem::load(2025, 1)?;

        let input = problem.get_input(2, &InputType::Sample)?;
        let answer = problem.get_answer(2, &InputType::Sample)?;

        assert_eq!(answer, super::solve(&input));

        Ok(())
    }

    #[test]
    fn test_full() -> Result<()> {
        let problem = Problem::load(2025, 1)?;

        let input = problem.get_input(2, &InputType::Full)?;
        let answer = problem.get_answer(2, &InputType::Full)?;

        assert_eq!(answer, super::solve(&input));

        Ok(())
    }
}
