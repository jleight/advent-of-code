pub fn solve(input: &str) -> String {
    let mut total = 0;

    let mut sum = 0;
    let mut product = 1;

    let cols = input
        .lines()
        .max_by(|a, b| a.len().cmp(&b.len()))
        .expect("no input lines found")
        .len();

    for i in 0..cols {
        let i = cols - i - 1;

        let col: String = input
            .lines()
            .map(|l| {
                l.chars()
                    .nth(i)
                    .unwrap_or(' ')
            })
            .collect();

        let mut col = col.trim();

        if col.is_empty() {
            continue;
        }

        let operation = if col.ends_with('+') {
            col = col
                .trim_end_matches('+')
                .trim_end();
            Some(Operation::Add)
        } else if col.ends_with('*') {
            col = col
                .trim_end_matches('*')
                .trim_end();
            Some(Operation::Multiply)
        } else {
            None
        };

        let number = col
            .parse::<u64>()
            .expect("failed to parse number");

        sum += number;
        product *= number;

        match operation {
            Some(Operation::Add) => total += sum,
            Some(Operation::Multiply) => total += product,
            None => continue,
        };

        sum = 0;
        product = 1;
    }

    total.to_string()
}

enum Operation {
    Add,
    Multiply,
}

#[cfg(test)]
mod tests {
    use crate::aoc::{InputType, Problem};
    use eyre::Result;

    #[test]
    fn test_sample() -> Result<()> {
        let problem = Problem::load(2025, 6)?;

        let input = problem.get_input(2, &InputType::Sample)?;
        let answer = problem.get_answer(2, &InputType::Sample)?;

        assert_eq!(answer, super::solve(&input));

        Ok(())
    }

    #[test]
    fn test_full() -> Result<()> {
        let problem = Problem::load(2025, 6)?;

        let input = problem.get_input(2, &InputType::Full)?;
        let answer = problem.get_answer(2, &InputType::Full)?;

        assert_eq!(answer, super::solve(&input));

        Ok(())
    }
}
