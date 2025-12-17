use crate::aoc::{InvalidArgSnafu, Result, SolutionTest};
use crate::years::get_solver;
use snafu::prelude::*;
use std::env::args;
use std::time::Instant;

mod aoc;
mod years;

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
