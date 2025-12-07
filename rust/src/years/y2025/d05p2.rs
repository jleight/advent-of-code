pub fn solve(input: &str) -> String {
    let mut fresh = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            break;
        }

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
    }

    loop {
        let mut removed = false;

        for ai in 0..fresh.len() {
            for bi in 0..fresh.len() {
                if ai == bi {
                    continue;
                }

                let a = fresh[ai].clone();
                let b = fresh[bi].clone();

                let contains_start = a.contains(b.start());
                let contains_end = a.contains(b.end());

                if !contains_start && !contains_end {
                    continue;
                }

                let new_start = if contains_start { a.start() } else { b.start() };
                let new_end = if contains_end { a.end() } else { b.end() };

                fresh[ai] = *new_start..=*new_end;
                fresh.swap_remove(bi);
                removed = true;
                break;
            }

            if removed {
                break;
            }
        }

        if !removed {
            break;
        }
    }

    fresh
        .iter()
        .map(|range| range.end() - range.start() + 1)
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::aoc::{InputType, Problem};
    use eyre::Result;

    #[test]
    fn test_sample() -> Result<()> {
        let problem = Problem::load(2025, 5)?;

        let input = problem.get_input(2, &InputType::Sample)?;
        let answer = problem.get_answer(2, &InputType::Sample)?;

        assert_eq!(answer, super::solve(&input));

        Ok(())
    }

    #[test]
    fn test_full() -> Result<()> {
        let problem = Problem::load(2025, 5)?;

        let input = problem.get_input(2, &InputType::Full)?;
        let answer = problem.get_answer(2, &InputType::Full)?;

        assert_eq!(answer, super::solve(&input));

        Ok(())
    }
}
