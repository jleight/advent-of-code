use itertools::Itertools;

pub fn solve(input: &str) -> String {
    let machines: Vec<Machine> = input
        .lines()
        .map(Machine::parse)
        .collect();

    let mut sum = 0;

    for machine in &machines {
        for k in 1.. {
            let found = machine
                .buttons
                .iter()
                .combinations_with_replacement(k)
                .any(|c| {
                    let mut lights = vec![false; machine.lights.len()];

                    for button in &c {
                        for l in *button {
                            let l = usize::from(*l);
                            lights[l] = !lights[l];
                        }
                    }

                    lights == machine.lights
                });

            if found {
                sum += k;
                break;
            }
        }
    }

    sum.to_string()
}

#[derive(Debug)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<u8>>,
}

impl Machine {
    fn parse(line: &str) -> Self {
        let mut lights = Vec::new();
        let mut buttons = Vec::new();

        let mut collection = Vec::new();
        let mut number = Vec::new();

        for c in line.chars() {
            match c {
                '[' | ']' | ' ' => (),
                '{' => break,
                '.' => lights.push(false),
                '#' => lights.push(true),
                '(' => collection = Vec::new(),
                ',' => {
                    let parsed = number
                        .iter()
                        .collect::<String>()
                        .parse::<u8>()
                        .expect("invalid number");
                    collection.push(parsed);
                    number = Vec::new();
                }
                ')' => {
                    let parsed = number
                        .iter()
                        .collect::<String>()
                        .parse::<u8>()
                        .expect("invalid number");
                    collection.push(parsed);
                    number = Vec::new();
                    buttons.push(collection.clone());
                }
                c => number.push(c),
            }
        }

        Self { lights, buttons }
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc::{InputType, Problem};
    use eyre::Result;

    #[test]
    fn test_sample() -> Result<()> {
        let problem = Problem::load(2025, 10)?;

        let input = problem.get_input(1, &InputType::Sample)?;
        let answer = problem.get_answer(1, &InputType::Sample)?;

        assert_eq!(answer, super::solve(&input));

        Ok(())
    }

    #[test]
    fn test_full() -> Result<()> {
        let problem = Problem::load(2025, 10)?;

        let input = problem.get_input(1, &InputType::Full)?;
        let answer = problem.get_answer(1, &InputType::Full)?;

        assert_eq!(answer, super::solve(&input));

        Ok(())
    }
}
