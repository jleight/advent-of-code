use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("Invalid {name} argument"))]
    InvalidArg { name: String },

    #[snafu(display("No solver found for Y{year}D{day:0>2}P{part}"))]
    MissingSolver { year: u16, day: u8, part: u8 },

    #[snafu(display("Current working directory is invalid"))]
    InvalidWorkingDirectory { source: std::io::Error },

    #[snafu(display("Could not find a .git directory above: {cwd}"))]
    MissingGitDirectory { cwd: String },

    #[snafu(display("Could not find a .data directory in the git project root: {git_root}"))]
    MissingDataDirectory { git_root: String },

    #[snafu(display("Could not find solution file: {path}"))]
    MissingSolutionFile { path: String },

    #[snafu(display("Failed to read solution file: {path}"))]
    FailedToReadSolutionFile {
        source: std::io::Error,
        path: String,
    },

    #[snafu(display("Cannot read solution file: {path}"))]
    InvalidSolutionFile {
        source: serde_json::Error,
        path: String,
    },

    #[snafu(display("Solution file {year}/{day:0>2} is missing solution test: {name}"))]
    MissingSolutionTest { year: u16, day: u8, name: String },

    #[snafu(display("Solution file {year}/{day:0>2} test {name} is missing input"))]
    MissingSolutionTestInput { year: u16, day: u8, name: String },

    #[snafu(display("Solution file {year}/{day:0>2} test {name} input is invalid"))]
    InvalidSolutionTestInput { year: u16, day: u8, name: String },

    #[snafu(display("Solution has an invalid return type"))]
    InvalidSolutionReturnType,

    #[snafu(display("Failed to parse input: expected '{expected}', got '{got}'"))]
    UnexpectedInput { expected: &'static str, got: String },

    #[snafu(display("Failed to parse input: expected '{expected}', got '{got}'"))]
    FailedToParseInput {
        source: std::num::ParseIntError,
        expected: &'static str,
        got: String,
    },

    #[snafu(display("Failed to solve the problem"))]
    SolutionFailed,
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
