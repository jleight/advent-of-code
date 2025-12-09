use eyre::{OptionExt, Result, WrapErr, eyre};
use rpkl::from_config;
use serde::Deserialize;
use std::path::PathBuf;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Problem {
    pub year: u16,
    pub day: u8,
    pub answers: Option<Answers>,
    pub inputs: Option<Inputs>,
}

#[derive(Debug, Deserialize)]
pub struct Answers {
    pub part_1: Option<Part>,
    pub part_2: Option<Part>,
}

#[derive(Debug, Deserialize)]
pub struct Part {
    pub sample: Option<String>,
    pub full: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Inputs {
    pub sample_1: Option<String>,
    pub sample_2: Option<String>,
    pub full: Option<String>,
}

pub enum InputType {
    Sample,
    Full,
}

impl Problem {
    pub fn load(year: u16, day: u8) -> Result<Self> {
        let problem_file = find_data_directory()?
            .join(year.to_string())
            .join(format!("{day:02}.pkl"));

        from_config::<Self>(problem_file.clone())
            .wrap_err_with(|| format!("failed to load problem file {}", problem_file.display()))
    }

    pub fn get_input(&self, part: u8, input_type: &InputType) -> Result<String> {
        let Some(inputs) = &self.inputs else {
            return Err(eyre!("problem file does not contain inputs"));
        };

        match input_type {
            InputType::Full => inputs
                .full
                .clone()
                .ok_or_eyre("missing full input"),
            InputType::Sample if part == 1 => inputs
                .sample_1
                .clone()
                .ok_or_eyre("missing sample input"),
            InputType::Sample => inputs
                .sample_2
                .clone()
                .or_else(|| inputs.sample_1.clone())
                .ok_or_eyre("missing sample input"),
        }
    }

    pub fn get_answer(&self, part: u8, input_type: &InputType) -> Result<String> {
        let Some(answers) = &self.answers else {
            return Err(eyre!("problem file does not contain answers"));
        };

        let part_answers = match part {
            1 => &answers.part_1,
            2 => &answers.part_2,
            _ => &None,
        };
        let Some(part_answers) = part_answers else {
            return Err(eyre!(
                "problem file does not contain answers for part {part}"
            ));
        };

        match input_type {
            InputType::Sample => part_answers
                .sample
                .clone()
                .ok_or_eyre("missing sample answer"),
            InputType::Full => part_answers
                .full
                .clone()
                .ok_or_eyre("missing full answer"),
        }
    }
}

fn find_data_directory() -> Result<PathBuf> {
    let dir = std::env::current_dir()?
        .ancestors()
        .find(|p| p.join(".git").exists())
        .ok_or_else(|| eyre!("failed to find the .git directory"))?
        .join(".data");
    Ok(dir)
}

#[cfg(test)]
mod tests {
    use crate::aoc::Problem;
    use eyre::Result;

    #[test]
    fn run() -> Result<()> {
        let p = Problem::load(2025, 6).expect("failed to load problem");
        println!("{p:?}");
        Ok(())
    }
}
