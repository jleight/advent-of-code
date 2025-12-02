#![warn(
    future_incompatible,
    keyword_idents,
    let_underscore,
    nonstandard_style,
    refining_impl_trait,
    rust_2018_compatibility,
    rust_2018_idioms,
    rust_2021_compatibility,
    rust_2024_compatibility,
    unused
)]
#![warn(clippy::pedantic, clippy::nursery)]

use crate::utils::SolutionContext;
use crate::years::get_solver;
use std::env;
use std::time::Instant;

mod utils;
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

    let test = env::args().nth(4) == Some("--test".to_string());

    let Some(solver) = get_solver(year, day, part) else {
        panic!("missing solution: Y{year}D{day:0>2}P{part}");
    };

    let ctx = SolutionContext::for_problem(year, day, part, test);
    let answer = ctx.answer.clone();

    let start = Instant::now();
    let solution = solver(&ctx);
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
