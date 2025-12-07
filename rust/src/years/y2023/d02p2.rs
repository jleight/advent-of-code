pub fn solve(input: &str) -> String {
    input
        .lines()
        .map(Game::parse)
        .map(|g| g.power())
        .sum::<u32>()
        .to_string()
}

#[derive(Debug)]
struct Reveal {
    red: u32,
    green: u32,
    blue: u32,
}

impl Reveal {
    fn parse(input: &str) -> Self {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for cube_count in input.split(',') {
            let split: Vec<&str> = cube_count
                .split_whitespace()
                .collect();

            assert_eq!(
                split.len(),
                2,
                "expected '[count] [color]', got {cube_count}"
            );

            let Ok(count) = split[0].parse::<u32>() else {
                panic!("failed to parse count from '{}'", split[0]);
            };

            match split[1] {
                "red" => red += count,
                "green" => green += count,
                "blue" => blue += count,
                _ => panic!("expected '[count] [color]', got {cube_count}"),
            }
        }

        Self { red, green, blue }
    }
}

#[derive(Debug)]
struct Game {
    reveals: Vec<Reveal>,
}

impl Game {
    fn parse(input: &str) -> Self {
        let Some((_, reveals)) = input.split_once(':') else {
            panic!("expected 'Game [id]: [reveals...]', got {input}");
        };

        let reveals = reveals
            .split("; ")
            .map(Reveal::parse)
            .collect();

        Self { reveals }
    }

    fn power(&self) -> u32 {
        let min_red = self
            .reveals
            .iter()
            .map(|r| r.red)
            .max()
            .unwrap_or(1);
        let min_green = self
            .reveals
            .iter()
            .map(|r| r.green)
            .max()
            .unwrap_or(1);
        let min_blue = self
            .reveals
            .iter()
            .map(|r| r.blue)
            .max()
            .unwrap_or(1);

        min_red * min_green * min_blue
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc::{InputType, Problem};
    use eyre::Result;

    #[test]
    fn test_sample() -> Result<()> {
        let problem = Problem::load(2023, 2)?;

        let input = problem.get_input(2, &InputType::Sample)?;
        let answer = problem.get_answer(2, &InputType::Sample)?;

        assert_eq!(answer, super::solve(&input));

        Ok(())
    }

    #[test]
    fn test_full() -> Result<()> {
        let problem = Problem::load(2023, 2)?;

        let input = problem.get_input(2, &InputType::Full)?;
        let answer = problem.get_answer(2, &InputType::Full)?;

        assert_eq!(answer, super::solve(&input));

        Ok(())
    }
}
