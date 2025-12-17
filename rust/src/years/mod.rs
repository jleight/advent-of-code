use crate::aoc::Error::{InvalidSolutionReturnType, MissingSolver};
use crate::aoc::{Result, ToOptionString};

pub mod y2015 {
    pub mod d01;
}

pub mod y2023 {
    pub mod d01;
    pub mod d02;
}

pub mod y2025 {
    pub mod d01;
    pub mod d02;
    pub mod d03;
    pub mod d04;
    pub mod d05;
    pub mod d06;
    pub mod d07;
    pub mod d08;
    pub mod d09;
    pub mod d10;
    pub mod d11;
    pub mod d12;
}

macro_rules! solver {
    ($year:tt, $day:tt, $part:tt) => {
        Ok(|i| {
            $year::$day::$part(i)
                .to_option_string()
                .ok_or_else(|| InvalidSolutionReturnType {})
        })
    };
}

pub fn get_solver(year: u16, day: u8, part: u8) -> Result<fn(&str) -> Result<String>> {
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
