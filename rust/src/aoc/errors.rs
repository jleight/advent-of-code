use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub(crate) enum Error {
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
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
