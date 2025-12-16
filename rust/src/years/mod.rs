mod y2015;
mod y2023;
mod y2025;

pub fn get_solver(year: u16, day: u8, part: u8) -> Option<fn(&str) -> String> {
    match year {
        2015 => y2015::get_solver(day, part),
        2023 => y2023::get_solver(day, part),
        2025 => y2025::get_solver(day, part),
        _ => None,
    }
}
