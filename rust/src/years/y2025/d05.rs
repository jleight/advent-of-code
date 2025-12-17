pub fn part_1(input: &str) -> i32 {
    let mut fresh = Vec::new();
    let mut count = 0;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        if line.contains('-') {
            let (start, end) = line
                .split_once('-')
                .expect("range should contain a '-'");

            let start = start
                .parse::<u64>()
                .expect("start is not a number");
            let end = end
                .parse::<u64>()
                .expect("end is not a number");

            fresh.push(start..=end);
        } else {
            let id = line
                .parse::<u64>()
                .expect("line is not a number");

            if fresh
                .iter()
                .any(|range| range.contains(&id))
            {
                count += 1;
            }
        }
    }

    count
}

pub fn part_2(input: &str) -> u64 {
    let mut fresh = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            break;
        }

        let (start, end) = line
            .split_once('-')
            .expect("range should contain a '-'");

        let start = start
            .parse::<u64>()
            .expect("start is not a number");
        let end = end
            .parse::<u64>()
            .expect("end is not a number");

        fresh.push(start..=end);
    }

    loop {
        let mut removed = false;

        for ai in 0..fresh.len() {
            for bi in 0..fresh.len() {
                if ai == bi {
                    continue;
                }

                let a = fresh[ai].clone();
                let b = fresh[bi].clone();

                let contains_start = a.contains(b.start());
                let contains_end = a.contains(b.end());

                if !contains_start && !contains_end {
                    continue;
                }

                let new_start = if contains_start { a.start() } else { b.start() };
                let new_end = if contains_end { a.end() } else { b.end() };

                fresh[ai] = *new_start..=*new_end;
                fresh.swap_remove(bi);
                removed = true;
                break;
            }

            if removed {
                break;
            }
        }

        if !removed {
            break;
        }
    }

    fresh
        .iter()
        .map(|range| range.end() - range.start() + 1)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};
    use crate::aoc::{Result, assert_solution};

    const YEAR: u16 = 2025;
    const DAY: u8 = 5;

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
