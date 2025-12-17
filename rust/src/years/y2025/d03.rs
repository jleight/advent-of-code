pub fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|bank| {
            let a = bank
                .chars()
                .map(|c| {
                    c.to_digit(10)
                        .expect("char is not a digit")
                })
                .max()
                .expect("bank is empty");

            let ai = bank
                .chars()
                .position(|c| c.to_digit(10).unwrap() == a)
                .unwrap();

            let (rest, swap) = if ai == bank.len() - 1 {
                (bank[..ai].to_string(), true)
            } else {
                (bank[ai + 1..].to_string(), false)
            };

            let b = rest
                .chars()
                .map(|c| {
                    c.to_digit(10)
                        .expect("char is not a digit")
                })
                .max()
                .expect("rest is empty");

            if swap { b * 10 + a } else { a * 10 + b }
        })
        .sum()
}

pub fn part_2(input: &str) -> u128 {
    input
        .lines()
        .map(|bank| {
            let bank_len = bank.len();

            let mut stack: Vec<u32> = Vec::new();

            for (i, battery) in bank.chars().enumerate() {
                let battery = battery
                    .to_digit(10)
                    .expect("battery is not a digit");

                let bank_left = bank_len - i;
                let mut stack_needs = 12 - stack.len();

                if bank_left == stack_needs {
                    stack.push(battery);
                    continue;
                }

                while stack
                    .pop_if(|top| *top < battery)
                    .is_some()
                {
                    stack_needs += 1;

                    if bank_left == stack_needs {
                        break;
                    }
                }

                if stack_needs > 0 {
                    stack.push(battery);
                }
            }

            stack
                .iter()
                .fold(String::new(), |a, e| a + &e.to_string())
                .parse::<u128>()
                .unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};
    use crate::aoc::{Result, assert_solution};

    const YEAR: u16 = 2025;
    const DAY: u8 = 3;

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
