use crate::aoc::Error::{InvalidSolutionReturnType, MissingSolver};
use crate::aoc::{InvalidArgSnafu, Result, SolutionTest, ToOptionString};
use snafu::prelude::*;
use std::env::args;
use std::time::Instant;

mod aoc;

mod years {
    macro_rules! load_solvers {
        ($year:tt, $($day:tt),*) => {
            pub mod $year {
                $(pub mod $day;)*
            }
        };
    }

    load_solvers!(y2015, d01);
    load_solvers!(y2023, d01, d02);
    load_solvers!(
        y2025, d01, d02, d03, d04, d05, d06, d07, d08, d09, d10, d11, d12
    );
}

macro_rules! solver {
    ($year:tt, $day:tt, $part:tt) => {
        Ok(|i| {
            years::$year::$day::$part(i)
                .to_option_string()
                .ok_or_else(|| InvalidSolutionReturnType {})
        })
    };
}

fn get_solver(year: u16, day: u8, part: u8) -> Result<fn(&str) -> Result<String>> {
    match (year, day, part) {
        (2015, 1, 1) => solver!(y2015, d01, part_1),
        (2015, 1, 2) => solver!(y2015, d01, part_2),
        (2023, 1, 1) => solver!(y2023, d01, part_1),
        (2023, 1, 2) => solver!(y2023, d01, part_2),
        (2023, 2, 1) => solver!(y2023, d02, part_1),
        (2023, 2, 2) => solver!(y2023, d02, part_2),
        (2025, 1, 1) => solver!(y2025, d01, part_1),
        (2025, 1, 2) => solver!(y2025, d01, part_2),
        (2025, 2, 1) => solver!(y2025, d02, part_1),
        (2025, 2, 2) => solver!(y2025, d02, part_2),
        (2025, 3, 1) => solver!(y2025, d03, part_1),
        (2025, 3, 2) => solver!(y2025, d03, part_2),
        (2025, 4, 1) => solver!(y2025, d04, part_1),
        (2025, 4, 2) => solver!(y2025, d04, part_2),
        (2025, 5, 1) => solver!(y2025, d05, part_1),
        (2025, 5, 2) => solver!(y2025, d05, part_2),
        (2025, 6, 1) => solver!(y2025, d06, part_1),
        (2025, 6, 2) => solver!(y2025, d06, part_2),
        (2025, 7, 1) => solver!(y2025, d07, part_1),
        (2025, 7, 2) => solver!(y2025, d07, part_2),
        (2025, 8, 1) => solver!(y2025, d08, part_1),
        (2025, 8, 2) => solver!(y2025, d08, part_2),
        (2025, 9, 1) => solver!(y2025, d09, part_1),
        (2025, 9, 2) => solver!(y2025, d09, part_2),
        (2025, 10, 1) => solver!(y2025, d10, part_1),
        (2025, 10, 2) => solver!(y2025, d10, part_2),
        (2025, 11, 1) => solver!(y2025, d11, part_1),
        (2025, 11, 2) => solver!(y2025, d11, part_2),
        (2025, 12, 1) => solver!(y2025, d12, part_1),
        _ => Err(MissingSolver { year, day, part }),
    }
}

fn main() -> Result<()> {
    let year = args()
        .nth(1)
        .and_then(|y| y.parse::<u16>().ok())
        .context(InvalidArgSnafu { name: "year" })?;
    let day = args()
        .nth(2)
        .and_then(|d| d.parse::<u8>().ok())
        .context(InvalidArgSnafu { name: "day" })?;
    let part = args()
        .nth(3)
        .and_then(|p| p.parse::<u8>().ok())
        .context(InvalidArgSnafu { name: "part" })?;
    let test = args()
        .nth(4)
        .unwrap_or_else(|| "full".to_string());

    let solver = get_solver(year, day, part)?;

    let solution_test = SolutionTest::load(year, day, &test)?;

    let input = solution_test.input;
    let output = match part {
        1 => solution_test.part_1,
        2 => solution_test.part_2,
        _ => panic!("invalid part: {part}"),
    };

    let start = Instant::now();
    let solution = solver(&input)?;
    let delta = start.elapsed();

    let check = match output {
        Some(a) if a == solution => "🟢",
        Some(_) => "🔴",
        None => "🟡",
    };

    println!("Problem : Y{year}D{day:0>2}P{part} ");
    println!("Solution: {solution} {check}");
    println!("Time    : {delta:?}");

    Ok(())
}
