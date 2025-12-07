use crate::aoc::{InputType, Problem};
use crate::years::get_solver;
use std::env;
use std::time::Instant;

mod aoc;
mod years;

fn main() {
    let year = env::args()
        .nth(1)
        .and_then(|y| y.parse::<u16>().ok())
        .expect("invalid argument: year");

    let day = env::args()
        .nth(2)
        .and_then(|d| d.parse::<u8>().ok())
        .expect("invalid argument: day");

    let part = env::args()
        .nth(3)
        .and_then(|p| p.parse::<u8>().ok())
        .expect("invalid argument: part");

    let input_type = if env::args().nth(4) == Some("--sample".to_string()) {
        InputType::Sample
    } else {
        InputType::Full
    };

    let Some(solver) = get_solver(year, day, part) else {
        panic!("missing solution: Y{year}D{day:0>2}P{part}");
    };

    let problem = Problem::load(year, day).expect("failed to load problem data");

    let input = problem
        .get_input(part, &input_type)
        .expect("failed to load sample input");
    let answer = problem
        .get_answer(part, &input_type)
        .ok();

    let start = Instant::now();
    let solution = solver(&input);
    let delta = start.elapsed();

    let check = match answer {
        Some(a) if a == solution => "🟢",
        Some(_) => "🔴",
        None => "🟡",
    };

    println!("Problem : Y{year}D{day:0>2}P{part} ");
    println!("Solution: {solution} {check}");
    println!("Time    : {delta:?}");
}
