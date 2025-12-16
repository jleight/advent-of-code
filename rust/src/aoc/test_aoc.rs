macro_rules! test_aoc {
    ($year:literal, $day:literal, $test:literal, $part:ident) => {
        let solution = crate::aoc::SolutionTest::load($year, $day, &$test);

        assert_eq!(solution.$part, Some($part(&solution.input.as_str())),);
    };
}

pub(crate) use test_aoc;
