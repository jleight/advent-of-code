use itertools::Itertools;
use std::collections::HashMap;

pub fn part_1(input: &str) -> u64 {
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

    dfs1(&graph, "you")
}

pub fn part_2(input: &str) -> u64 {
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

    dfs2(&graph, "svr", false, false, &mut HashMap::new())
}

fn dfs1(graph: &HashMap<&str, Vec<&str>>, device: &str) -> u64 {
    if device == "out" {
        1
    } else if let Some(next) = graph.get(device) {
        next.iter()
            .map(|n| dfs1(graph, n))
            .sum()
    } else {
        0
    }
}

fn dfs2<'a>(
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
                .map(|n| dfs2(graph, n, dac, fft, cache))
                .sum()
        });

    cache.insert((device, dac, fft), result);
    result
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};
    use crate::aoc::{assert_solution, Result};

    const YEAR: u16 = 2025;
    const DAY: u8 = 11;

    #[test]
    fn part_1_sample_a() -> Result<()> {
        assert_solution!(part_1, "a");
        Ok(())
    }

    #[test]
    fn part_1_full() -> Result<()> {
        assert_solution!(part_1, "full");
        Ok(())
    }

    #[test]
    fn part_2_sample_b() -> Result<()> {
        assert_solution!(part_2, "b");
        Ok(())
    }

    #[test]
    fn part_2_full() -> Result<()> {
        assert_solution!(part_2, "full");
        Ok(())
    }
}
