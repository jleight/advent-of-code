use z3::ast::Int;
use z3::{Optimize, SatResult};

pub fn solve(input: &str) -> String {
    let machines: Vec<Machine> = input
        .lines()
        .map(Machine::parse)
        .collect();

    let mut sum = 0;

    for machine in &machines {
        let optimize = Optimize::new();

        let button_presses: Vec<_> = machine
            .buttons
            .iter()
            .map(|_| {
                let b = Int::fresh_const("button");
                optimize.assert(&b.ge(0));
                b
            })
            .collect();

        machine
            .joltages
            .iter()
            .enumerate()
            .for_each(|(i, target)| {
                let x: Vec<_> = machine
                    .buttons
                    .iter()
                    .enumerate()
                    .filter_map(|(j, button)| {
                        if button.contains(&(i as u64)) {
                            Some(button_presses[j].clone())
                        } else {
                            None
                        }
                    })
                    .collect();

                let sum = Int::add(&x);
                optimize.assert(&sum.eq(Int::from_u64(*target)));
            });

        let total_presses = Int::new_const("total");
        optimize.assert(&total_presses.eq(Int::add(&button_presses)));
        optimize.minimize(&total_presses);

        sum += match optimize.check(&[]) {
            SatResult::Sat => optimize
                .get_model()
                .unwrap()
                .eval(&total_presses, true)
                .and_then(|t| t.as_u64())
                .unwrap(),
            SatResult::Unsat => panic!("no solution found"),
            SatResult::Unknown => panic!("unknown result"),
        }
    }

    sum.to_string()
}

#[derive(Debug)]
struct Machine {
    buttons: Vec<Vec<u64>>,
    joltages: Vec<u64>,
}

impl Machine {
    fn parse(line: &str) -> Self {
        let mut buttons = Vec::new();

        let mut collection = Vec::new();
        let mut number = Vec::new();

        for c in line.chars() {
            match c {
                '[' | '.' | '#' | ']' | ' ' | '}' => (),
                '(' | '{' => collection = Vec::new(),
                ',' => {
                    let parsed = number
                        .iter()
                        .collect::<String>()
                        .parse::<u64>()
                        .expect("invalid number");
                    collection.push(parsed);
                    number = Vec::new();
                }
                ')' => {
                    let parsed = number
                        .iter()
                        .collect::<String>()
                        .parse::<u64>()
                        .expect("invalid number");
                    collection.push(parsed);
                    number = Vec::new();
                    buttons.push(collection.clone());
                }
                c => number.push(c),
            }
        }

        let parsed = number
            .iter()
            .collect::<String>()
            .parse::<u64>()
            .expect("invalid number");
        collection.push(parsed);

        Self {
            buttons,
            joltages: collection.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc::{InputType, Problem};
    use eyre::Result;

    #[test]
    fn test_sample() -> Result<()> {
        let problem = Problem::load(2025, 10)?;

        let input = problem.get_input(2, &InputType::Sample)?;
        let answer = problem.get_answer(2, &InputType::Sample)?;

        assert_eq!(answer, super::solve(&input));

        Ok(())
    }

    #[test]
    fn test_full() -> Result<()> {
        let problem = Problem::load(2025, 10)?;

        let input = problem.get_input(2, &InputType::Full)?;
        let answer = problem.get_answer(2, &InputType::Full)?;

        assert_eq!(answer, super::solve(&input));

        Ok(())
    }
}
