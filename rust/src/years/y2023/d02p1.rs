pub fn solve(input: &str) -> String {
    input
        .lines()
        .map(Game::parse)
        .filter(Game::is_valid)
        .map(|g| g.id)
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
    id: u32,
    reveals: Vec<Reveal>,
}

impl Game {
    fn parse(input: &str) -> Self {
        let Some((game, reveals)) = input.split_once(':') else {
            panic!("expected 'Game [id]: [reveals...]', got {input}");
        };

        let id = game
            .replace("Game ", "")
            .parse::<u32>()
            .unwrap();
        let reveals = reveals
            .split("; ")
            .map(Reveal::parse)
            .collect();

        Self { id, reveals }
    }

    fn is_valid(&self) -> bool {
        self.reveals
            .iter()
            .all(|r| r.red <= 12 && r.green <= 13 && r.blue <= 14)
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc::{InputType, Problem};
    use eyre::Result;

    #[test]
    fn test_sample() -> Result<()> {
        let problem = Problem::load(2023, 2)?;

        let input = problem.get_input(1, &InputType::Sample)?;
        let answer = problem.get_answer(1, &InputType::Sample)?;

        assert_eq!(answer, super::solve(&input));

        Ok(())
    }

    #[test]
    fn test_full() -> Result<()> {
        let problem = Problem::load(2023, 2)?;

        let input = problem.get_input(1, &InputType::Full)?;
        let answer = problem.get_answer(1, &InputType::Full)?;

        assert_eq!(answer, super::solve(&input));

        Ok(())
    }
}
