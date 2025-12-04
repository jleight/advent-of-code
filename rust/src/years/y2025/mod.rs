use crate::utils::SolutionContext;

mod d01p1;
mod d01p2;
mod d02p1;
mod d02p2;
mod d03p1;
mod d03p2;

pub fn get_solver(day: u8, part: u8) -> Option<fn(&SolutionContext) -> String> {
    match (day, part) {
        (1, 1) => Some(d01p1::solve),
        (1, 2) => Some(d01p2::solve),
        (2, 1) => Some(d02p1::solve),
        (2, 2) => Some(d02p2::solve),
        (3, 1) => Some(d03p1::solve),
        (3, 2) => Some(d03p2::solve),
        _ => None,
    }
}
