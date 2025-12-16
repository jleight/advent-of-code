use serde_json::Value;
use std::fs::File;

#[derive(Debug)]
pub struct SolutionTest {
    pub input: String,
    pub part_1: Option<String>,
    pub part_2: Option<String>,
}

impl SolutionTest {
    pub fn load(year: u16, day: u8, name: &str) -> Self {
        let data_file_path = std::env::current_dir()
            .expect("failed to get current directory")
            .ancestors()
            .find(|p| p.join(".git").exists())
            .expect("failed to find root of git repository")
            .join(".data")
            .join("data.json");

        let Ok(data_file) = File::open(data_file_path.clone()) else {
            panic!("failed to open data file: {}", data_file_path.display());
        };

        let Ok(json) = serde_json::from_reader::<_, Value>(data_file) else {
            panic!("failed to parse data file: {}", data_file_path.display());
        };

        let Some(tests) = json.get(&format!("{year}-{day:0>2}")) else {
            panic!("no tests for day {year}-{day:0>2}");
        };

        let Some(test) = tests.get(name) else {
            panic!("no test named {name}");
        };

        let input = test
            .get("input")
            .expect("test has no input")
            .as_str()
            .expect("input is not a string")
            .trim()
            .to_string();

        let part_1 = test
            .get("part_1")
            .and_then(|s| s.as_str())
            .map(|s| s.trim().to_string());
        let part_2 = test
            .get("part_2")
            .and_then(|s| s.as_str())
            .map(|s| s.trim().to_string());

        SolutionTest {
            input,
            part_1,
            part_2,
        }
    }
}
