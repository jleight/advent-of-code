use itertools::Itertools;
use std::collections::HashMap;

pub fn solve(input: &str) -> String {
    let graph = input
        .lines()
        .flat_map(|line| {
            let (key, values) = line
                .split_once(": ")
                .expect("expected line to contain ':'");

            values
                .split_whitespace()
                .map(|v| (key, v))
                .collect::<Vec<_>>()
        })
        .into_group_map();

    dfs(&graph, "you").to_string()
}

fn dfs(graph: &HashMap<&str, Vec<&str>>, device: &str) -> u64 {
    if device == "out" {
        1
    } else if let Some(next) = graph.get(device) {
        next.iter()
            .map(|n| dfs(graph, n))
            .sum()
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc::{InputType, Problem};
    use eyre::Result;

    #[test]
    fn test_sample() -> Result<()> {
        let problem = Problem::load(2025, 11)?;

        let input = problem.get_input(1, &InputType::Sample)?;
        let answer = problem.get_answer(1, &InputType::Sample)?;

        assert_eq!(answer, super::solve(&input));

        Ok(())
    }

    #[test]
    fn test_full() -> Result<()> {
        let problem = Problem::load(2025, 11)?;

        let input = problem.get_input(1, &InputType::Full)?;
        let answer = problem.get_answer(1, &InputType::Full)?;

        assert_eq!(answer, super::solve(&input));

        Ok(())
    }
}
