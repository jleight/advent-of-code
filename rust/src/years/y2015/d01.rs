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
    use crate::aoc::test_aoc;

    #[test]
    fn test_part_1_sample_a() {
        test_aoc!(2015, 1, "a", part_1);
    }

    #[test]
    fn test_part_1_sample_b() {
        test_aoc!(2015, 1, "b", part_1);
    }

    #[test]
    fn test_part_1_sample_c() {
        test_aoc!(2015, 1, "c", part_1);
    }

    #[test]
    fn test_part_1_sample_d() {
        test_aoc!(2015, 1, "d", part_1);
    }

    #[test]
    fn test_part_1_sample_e() {
        test_aoc!(2015, 1, "e", part_1);
    }

    #[test]
    fn test_part_1_sample_f() {
        test_aoc!(2015, 1, "f", part_1);
    }

    #[test]
    fn test_part_1_sample_g() {
        test_aoc!(2015, 1, "g", part_1);
    }

    #[test]
    fn test_part_1_sample_h() {
        test_aoc!(2015, 1, "h", part_1);
    }

    #[test]
    fn test_part_1_sample_i() {
        test_aoc!(2015, 1, "i", part_1);
    }

    #[test]
    fn test_part_1_full() {
        test_aoc!(2015, 1, "full", part_1);
    }

    #[test]
    fn test_part_2_sample_j() {
        test_aoc!(2015, 1, "j", part_2);
    }

    #[test]
    fn test_part_2_sample_k() {
        test_aoc!(2015, 1, "k", part_2);
    }

    #[test]
    fn test_part_2_full() {
        test_aoc!(2015, 1, "full", part_2);
    }
}
