use crate::utils::SolutionContext;

mod d01p1;
mod d01p2;

pub fn get_solver(day: u8, part: u8) -> Option<fn(&SolutionContext) -> String> {
    match (day, part) {
        (1, 1) => Some(d01p1::solve),
        (1, 2) => Some(d01p2::solve),
        _ => None,
    }
}
