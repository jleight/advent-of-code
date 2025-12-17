use itertools::Itertools;
use z3::ast::Int;
use z3::{Optimize, SatResult};

pub fn part_1(input: &str) -> usize {
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
                            let l = usize::try_from(*l).expect("invalid light index");
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

    sum
}

pub fn part_2(input: &str) -> u64 {
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

    sum
}

#[derive(Debug)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<u64>>,
    joltages: Vec<u64>,
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
                '}' => break,
                '.' => lights.push(false),
                '#' => lights.push(true),
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
            lights,
            buttons,
            joltages: collection.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};
    use crate::aoc::{Result, assert_solution};

    const YEAR: u16 = 2025;
    const DAY: u8 = 10;

    #[test]
    fn part_1_sample() -> Result<()> {
        assert_solution!(part_1, "sample");
        Ok(())
    }

    #[test]
    fn part_1_full() -> Result<()> {
        assert_solution!(part_1, "full");
        Ok(())
    }

    #[test]
    fn part_2_sample() -> Result<()> {
        assert_solution!(part_2, "sample");
        Ok(())
    }

    #[test]
    fn part_2_full() -> Result<()> {
        assert_solution!(part_2, "full");
        Ok(())
    }
}
