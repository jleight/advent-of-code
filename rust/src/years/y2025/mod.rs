mod d01p1;
mod d01p2;
mod d02p1;
mod d02p2;
mod d03p1;
mod d03p2;
mod d04p1;
mod d04p2;
mod d05p1;
mod d05p2;
mod d06p1;
mod d06p2;
mod d07p1;
mod d07p2;
mod d08p1;
mod d08p2;
mod d09p1;
mod d09p2;
mod d10p1;
mod d10p2;
mod d11p1;
mod d11p2;
mod d12p1;

pub fn get_solver(day: u8, part: u8) -> Option<fn(&str) -> String> {
    match (day, part) {
        (1, 1) => Some(d01p1::solve),
        (1, 2) => Some(d01p2::solve),
        (2, 1) => Some(d02p1::solve),
        (2, 2) => Some(d02p2::solve),
        (3, 1) => Some(d03p1::solve),
        (3, 2) => Some(d03p2::solve),
        (4, 1) => Some(d04p1::solve),
        (4, 2) => Some(d04p2::solve),
        (5, 1) => Some(d05p1::solve),
        (5, 2) => Some(d05p2::solve),
        (6, 1) => Some(d06p1::solve),
        (6, 2) => Some(d06p2::solve),
        (7, 1) => Some(d07p1::solve),
        (7, 2) => Some(d07p2::solve),
        (8, 1) => Some(d08p1::solve),
        (8, 2) => Some(d08p2::solve),
        (9, 1) => Some(d09p1::solve),
        (9, 2) => Some(d09p2::solve),
        (10, 1) => Some(d10p1::solve),
        (10, 2) => Some(d10p2::solve),
        (11, 1) => Some(d11p1::solve),
        (11, 2) => Some(d11p2::solve),
        (12, 1) => Some(d12p1::solve),
        _ => None,
    }
}
