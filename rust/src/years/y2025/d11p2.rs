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

    dfs(&graph, "svr", false, false, &mut HashMap::new()).to_string()
}

fn dfs<'a>(
    graph: &HashMap<&'a str, Vec<&'a str>>,
    device: &'a str,
    dac: bool,
    fft: bool,
    cache: &mut HashMap<(&'a str, bool, bool), u64>,
) -> u64 {
    if device == "out" {
        return u64::from(dac && fft);
    }

    if let Some(cached) = cache.get(&(device, dac, fft)) {
        return *cached;
    }

    let dac = dac || device == "dac";
    let fft = fft || device == "fft";

    let result = graph
        .get(device)
        .map_or(0, |next| {
            next.iter()
                .map(|n| dfs(graph, n, dac, fft, cache))
                .sum()
        });

    cache.insert((device, dac, fft), result);
    result
}

#[cfg(test)]
mod tests {
    use crate::aoc::{InputType, Problem};
    use eyre::Result;

    #[test]
    fn test_sample() -> Result<()> {
        let problem = Problem::load(2025, 11)?;

        let input = problem.get_input(2, &InputType::Sample)?;
        let answer = problem.get_answer(2, &InputType::Sample)?;

        assert_eq!(answer, super::solve(&input));

        Ok(())
    }

    #[test]
    fn test_full() -> Result<()> {
        let problem = Problem::load(2025, 11)?;

        let input = problem.get_input(2, &InputType::Full)?;
        let answer = problem.get_answer(2, &InputType::Full)?;

        assert_eq!(answer, super::solve(&input));

        Ok(())
    }
}
