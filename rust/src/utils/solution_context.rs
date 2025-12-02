use std::path::PathBuf;

#[allow(dead_code)]
pub struct SolutionContext {
    pub year: u16,
    pub day: u8,
    pub part: u8,
    pub test: bool,
    pub input: String,
    pub input_lines: Vec<String>,
    pub answer: Option<String>,
}

impl SolutionContext {
    pub fn for_problem(year: u16, day: u8, part: u8, test: bool) -> Self {
        let input = get_problem_input(year, day, part, test);
        let answer = get_problem_answer(year, day, part, test);

        Self {
            year,
            day,
            part,
            test,
            input: input.clone(),
            input_lines: input.lines().map(ToString::to_string).collect(),
            answer,
        }
    }
}

fn find_data_directory() -> PathBuf {
    std::env::current_dir()
        .expect("failed to get the current working directory")
        .ancestors()
        .find(|p| p.join(".git").exists())
        .expect("failed to find the .git directory")
        .join(".data")
}

fn try_read(file_type: &str, year: u16, file: String) -> Option<String> {
    let path = find_data_directory()
        .join(file_type)
        .join(year.to_string())
        .join(file);

    std::fs::read_to_string(path).ok()
}

fn get_problem_input(year: u16, day: u8, part: u8, test: bool) -> String {
    let suffix = if test { "_Test" } else { "" };

    let file_part = format!("D{day:0>2}P{part}{suffix}.txt");
    if let Some(input) = try_read("inputs", year, file_part.clone()) {
        return input.trim().to_string();
    }

    let file_day = format!("D{day:0>2}{suffix}.txt");
    if let Some(input) = try_read("inputs", year, file_day.clone()) {
        return input.trim().to_string();
    }

    panic!("missing input file, expected: [{file_part:?}] or [{file_day:?}]")
}

fn get_problem_answer(year: u16, day: u8, part: u8, test: bool) -> Option<String> {
    let suffix = if test { "_Test" } else { "" };

    let file = format!("D{day:0>2}P{part}{suffix}.txt");
    if let Some(answer) = try_read("answers", year, file) {
        return Some(answer.trim().to_string());
    }

    None
}
