use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

pub fn part_1(input: &str) -> i32 {
    parse(input).iter().sum()
}

pub fn part_2(input: &str) -> i128 {
    parse(input)
        .iter()
        .enumerate()
        .map(|(i, e)| (i as i128 + 1, i128::from(*e)))
        .fold_while(
            0,
            |a, (i, e)| {
                if a + e < 0 { Done(i) } else { Continue(a + e) }
            },
        )
        .into_inner()
}

fn parse(input: &str) -> Vec<i32> {
    input
        .chars()
        .filter_map(|c| match c {
            '(' => Some(1),
            ')' => Some(-1),
            _ => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};
    use crate::aoc::{Result, assert_solution};

    const YEAR: u16 = 2015;
    const DAY: u8 = 1;

    #[test]
    fn part_1_sample_a() -> Result<()> {
        assert_solution!(part_1, "a");
        Ok(())
    }

    #[test]
    fn part_1_sample_b() -> Result<()> {
        assert_solution!(part_1, "b");
        Ok(())
    }

    #[test]
    fn part_1_sample_c() -> Result<()> {
        assert_solution!(part_1, "c");
        Ok(())
    }

    #[test]
    fn part_1_sample_d() -> Result<()> {
        assert_solution!(part_1, "d");
        Ok(())
    }

    #[test]
    fn part_1_sample_e() -> Result<()> {
        assert_solution!(part_1, "e");
        Ok(())
    }

    #[test]
    fn part_1_sample_f() -> Result<()> {
        assert_solution!(part_1, "f");
        Ok(())
    }

    #[test]
    fn part_1_sample_g() -> Result<()> {
        assert_solution!(part_1, "g");
        Ok(())
    }

    #[test]
    fn part_1_sample_h() -> Result<()> {
        assert_solution!(part_1, "h");
        Ok(())
    }

    #[test]
    fn part_1_sample_i() -> Result<()> {
        assert_solution!(part_1, "h");
        Ok(())
    }

    #[test]
    fn part_1_full() -> Result<()> {
        assert_solution!(part_1, "full");
        Ok(())
    }

    #[test]
    fn part_2_sample_j() -> Result<()> {
        assert_solution!(part_2, "j");
        Ok(())
    }

    #[test]
    fn part_2_sample_k() -> Result<()> {
        assert_solution!(part_2, "k");
        Ok(())
    }

    #[test]
    fn part_2_full() -> Result<()> {
        assert_solution!(part_2, "full");
        Ok(())
    }
}
