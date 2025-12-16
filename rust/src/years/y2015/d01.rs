use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

pub fn part_1(input: &str) -> String {
    parse(input)
        .iter()
        .sum::<i32>()
        .to_string()
}

pub fn part_2(input: &str) -> String {
    parse(input)
        .iter()
        .enumerate()
        .map(|(i, e)| (i + 1, e))
        .fold_while(0, |a, (i, e)| {
            if a + e < 0 {
                Done(i as i32)
            } else {
                Continue(a + e)
            }
        })
        .into_inner()
        .to_string()
}

fn parse(input: &str) -> Vec<i32> {
    input
        .chars()
        .map(|c| if c == '(' { 1 } else { -1 })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};
    use crate::aoc::test_solution;
    use crate::aoc::{Result, SolutionTest};

    const YEAR: u16 = 2015;
    const DAY: u8 = 1;

    #[test]
    fn test_part_1_sample_a() -> Result<()> {
        let solution = SolutionTest::load(YEAR, DAY, "a")?;
        assert_eq!(solution.part_1, Some(part_1(&solution.input.as_str())));
        Ok(())
    }

    #[test]
    fn test_part_1_sample_b() -> Result<()> {
        test_solution!(2015, 1, "b", part_1);
        Ok(())
    }

    #[test]
    fn test_part_1_sample_c() -> Result<()> {
        test_solution!(2015, 1, "c", part_1);
        Ok(())
    }

    #[test]
    fn test_part_1_sample_d() -> Result<()> {
        test_solution!(2015, 1, "d", part_1);
        Ok(())
    }

    #[test]
    fn test_part_1_sample_e() -> Result<()> {
        test_solution!(2015, 1, "e", part_1);
        Ok(())
    }

    #[test]
    fn test_part_1_sample_f() -> Result<()> {
        test_solution!(2015, 1, "f", part_1);
        Ok(())
    }

    #[test]
    fn test_part_1_sample_g() -> Result<()> {
        test_solution!(2015, 1, "g", part_1);
        Ok(())
    }

    #[test]
    fn test_part_1_sample_h() -> Result<()> {
        test_solution!(2015, 1, "h", part_1);
        Ok(())
    }

    #[test]
    fn test_part_1_sample_i() -> Result<()> {
        test_solution!(2015, 1, "i", part_1);
        Ok(())
    }

    #[test]
    fn test_part_1_full() -> Result<()> {
        test_solution!(2015, 1, "full", part_1);
        Ok(())
    }

    #[test]
    fn test_part_2_sample_j() -> Result<()> {
        test_solution!(2015, 1, "j", part_2);
        Ok(())
    }

    #[test]
    fn test_part_2_sample_k() -> Result<()> {
        test_solution!(2015, 1, "k", part_2);
        Ok(())
    }

    #[test]
    fn test_part_2_full() -> Result<()> {
        test_solution!(2015, 1, "full", part_2);
        Ok(())
    }
}
