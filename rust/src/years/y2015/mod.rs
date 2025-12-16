mod d01;

pub fn get_solver(day: u8, part: u8) -> Option<fn(&str) -> String> {
    match (day, part) {
        (1, 1) => Some(d01::part_1),
        _ => None,
    }
}
