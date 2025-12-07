pub fn solve(input: &str) -> String {
    let mut current = 50i16;
    let mut count = 0u16;

    for (i, line) in input.lines().enumerate() {
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
    use crate::aoc::{InputType, Problem};
    use eyre::Result;

    #[test]
    fn test_sample() -> Result<()> {
        let problem = Problem::load(2025, 1)?;

        let input = problem.get_input(1, &InputType::Sample)?;
        let answer = problem.get_answer(1, &InputType::Sample)?;

        assert_eq!(answer, super::solve(&input));

        Ok(())
    }

    #[test]
    fn test_full() -> Result<()> {
        let problem = Problem::load(2025, 1)?;

        let input = problem.get_input(1, &InputType::Full)?;
        let answer = problem.get_answer(1, &InputType::Full)?;

        assert_eq!(answer, super::solve(&input));

        Ok(())
    }
}
