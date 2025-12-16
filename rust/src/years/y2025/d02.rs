pub fn part_1(input: &str) -> u64 {
    let mut sum = 0;

    for range in input.split(',') {
        let (start, end) = range
            .split_once('-')
            .expect("range should contain a '-'");

        let start = start
            .parse::<u64>()
            .expect("start is not a number");
        let end = end
            .parse::<u64>()
            .expect("end is not a number");

        for i in start..=end {
            let str = i.to_string();

            if str.len() % 2 != 0 {
                continue;
            }

            let middle = str.len() / 2;
            let left = &str[..middle];
            let right = &str[middle..];

            if left == right {
                sum += i;
            }
        }
    }

    sum
}

pub fn part_2(input: &str) -> u64 {
    let mut sum = 0;

    for range in input.split(',') {
        let (start, end) = range
            .split_once('-')
            .expect("range should contain a '-'");

        let start = start
            .parse::<u64>()
            .expect("start is not a number");
        let end = end
            .parse::<u64>()
            .expect("end is not a number");

        for i in start..=end {
            let str = i.to_string();
            let str_len = str.len();

            let half_len = str_len / 2;

            let mut success = false;

            for j in 1..=half_len {
                if str_len % j != 0 {
                    continue;
                }

                let pattern = str[0..j].to_string();
                let test = pattern.repeat(str_len / j);

                if str == test {
                    success = true;
                    break;
                }
            }

            if success {
                sum += i;
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};
    use crate::aoc::{assert_solution, Result};

    const YEAR: u16 = 2025;
    const DAY: u8 = 2;

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
