use crate::aoc::{
    FailedToReadSolutionFileSnafu, InvalidSolutionFileSnafu, InvalidSolutionTestInputSnafu,
    InvalidWorkingDirectorySnafu, MissingDataDirectorySnafu, MissingGitDirectorySnafu,
    MissingSolutionFileSnafu, MissingSolutionTestInputSnafu, MissingSolutionTestSnafu, Result,
};
use serde_json::Value;
use snafu::ResultExt;
use snafu::prelude::*;
use std::env::current_dir;
use std::fs::File;
use std::path::PathBuf;

#[derive(Debug)]
pub struct SolutionTest {
    pub input: String,
    pub part_1: Option<String>,
    pub part_2: Option<String>,
}

impl SolutionTest {
    pub fn load(year: u16, day: u8, name: &str) -> Result<Self> {
        let file_path = find_data_root()?
            .join(year.to_string())
            .join(format!("{day:0>2}.json"));
        ensure!(
            file_path.exists(),
            MissingSolutionFileSnafu {
                path: file_path
                    .display()
                    .to_string()
            }
        );

        let file = File::open(file_path.clone()).context(FailedToReadSolutionFileSnafu {
            path: file_path
                .display()
                .to_string(),
        })?;

        let json = serde_json::from_reader::<_, Value>(file).context(InvalidSolutionFileSnafu {
            path: file_path
                .display()
                .to_string(),
        })?;

        let test = json
            .get(&name)
            .context(MissingSolutionTestSnafu {
                year,
                day,
                name: name.to_string(),
            })?;

        let input = test
            .get("input")
            .context(MissingSolutionTestInputSnafu {
                year,
                day,
                name: name.to_string(),
            })?
            .as_str()
            .context(InvalidSolutionTestInputSnafu {
                year,
                day,
                name: name.to_string(),
            })?
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

        Ok(SolutionTest {
            input,
            part_1,
            part_2,
        })
    }
}

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

macro_rules! test_solution {
    ($year:literal, $day:literal, $test:literal, $part:ident) => {
        let solution = crate::aoc::SolutionTest::load($year, $day, &$test)?;
        assert_eq!(solution.$part, Some($part(&solution.input.as_str())));
    };
}

pub(crate) use test_solution;
