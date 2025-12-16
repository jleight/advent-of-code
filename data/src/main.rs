use serde::{Deserialize, Serialize};
use serde_json::json;
use snafu::prelude::*;
use snafu::{OptionExt, ResultExt, Snafu};
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::{FileType, read_to_string, write};
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("Current working directory is invalid"))]
    InvalidWorkingDirectory { source: std::io::Error },

    #[snafu(display("Could not find a .git directory above: {cwd}"))]
    MissingGitDirectory { cwd: String },

    #[snafu(display("Could not find a .data directory in the git project root: {git_root}"))]
    MissingDataDirectory { git_root: String },
}

type Result<T, E = Error> = std::result::Result<T, E>;

fn find_data_root() -> Result<PathBuf> {
    let cwd = current_dir().context(InvalidWorkingDirectorySnafu {})?;

    let git_root = cwd
        .ancestors()
        .find(|a| a.join(".git").exists())
        .context(MissingGitDirectorySnafu {
            cwd: cwd.display().to_string(),
        })?;

    let data_root = git_root.join(".data");

    ensure!(
        data_root.exists(),
        MissingDataDirectorySnafu {
            git_root: git_root
                .display()
                .to_string()
        }
    );
    Ok(data_root)
}

fn find_toml_files(data_dir: PathBuf) -> Vec<PathBuf> {
    let is_toml = |e: &DirEntry| {
        if !FileType::is_file(&e.file_type()) {
            return false;
        }

        let extension = e
            .path()
            .extension()
            .unwrap_or_default();
        extension == "toml"
    };

    WalkDir::new(data_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(is_toml)
        .map(DirEntry::into_path)
        .collect()
}

fn convert_file(
    toml_file: &Path,
    data_file: &DataFile,
) {
    let json_file = toml_file.with_extension("json");

    let value = json!(data_file.tests);
    let json = value.to_string();

    if let Err(e) = write(json_file, json) {
        eprintln!("failed to write json file: {e}");
    }
}

fn main() -> Result<()> {
    let data_dir = find_data_root()?;
    let toml_files = find_toml_files(data_dir);

    for toml_file in toml_files {
        let Ok(content) = read_to_string(&toml_file) else {
            eprintln!("failed to read file: {}", toml_file.display());
            continue;
        };

        match toml::from_str::<DataFile>(&content) {
            Ok(parsed) => convert_file(&toml_file, &parsed),
            Err(e) => eprintln!("failed to parse file '{}': {}", toml_file.display(), e),
        }
    }

    Ok(())
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
