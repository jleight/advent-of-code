#[allow(unused_macros)]
macro_rules! assert_solution {
    ($part:ident, $test:literal) => {
        let solution = crate::aoc::SolutionTest::load(YEAR, DAY, $test)?;
        let input = &solution.input.as_str();
        let output = solution.$part;

        let result = crate::aoc::ToOptionString::to_option_string(&$part(input));

        assert_eq!(output, result);
    };
}

#[cfg(test)]
pub(crate) use assert_solution;
