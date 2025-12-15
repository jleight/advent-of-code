use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::fs::{FileType, read_to_string};
use std::{env, fs};
use walkdir::WalkDir;

fn main() {
    let Ok(cwd) = env::current_dir() else {
        panic!("failed to get current directory");
    };

    let data_dir = cwd
        .ancestors()
        .find(|a| a.join(".git").exists())
        .expect("failed to find root of git repository")
        .join(".data");
    let output_file = data_dir.join("data.json");

    let data_files = WalkDir::new(data_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| FileType::is_file(&e.file_type()))
        .filter(|e| {
            e.path()
                .extension()
                .unwrap_or_default()
                == "toml"
        });

    let mut all_tests = HashMap::new();

    for data_file in data_files {
        let day = data_file
            .path()
            .file_stem()
            .expect("expected data file to have a name")
            .to_str()
            .expect("expected data file have a valid name");
        let year = data_file
            .path()
            .parent()
            .expect("expected data file to have a parent folder")
            .file_name()
            .expect("expected data file parent folder to have a name")
            .to_str()
            .expect("expected data file parent folder to have a valid name");
        let key = format!("{year}-{day}");

        let Ok(content) = read_to_string(data_file.path()) else {
            continue;
        };

        let Ok(parsed) = toml::from_str::<DataFile>(&content) else {
            continue;
        };

        all_tests.insert(key, parsed.tests);
    }

    let json = json!(all_tests);
    let output = json.to_string();

    fs::write(output_file, output).expect("failed to write aggregate tests file");
}

#[derive(Debug, Deserialize)]
struct DataFile {
    pub tests: HashMap<String, DataFileTest>,
}

#[derive(Debug, Deserialize, Serialize)]
struct DataFileTest {
    pub input: String,
    pub part_1: Option<String>,
    pub part_2: Option<String>,
}
