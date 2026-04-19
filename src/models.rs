use std::{
    fmt::{self, Debug, Display, Formatter},
    path::PathBuf,
};

use clap::Parser;

/// CLI parameters parsed by [`clap`].
///
/// Either `--file-path` or `--directory` must be provided; they are mutually exclusive.
///
/// # Examples
///
/// ```bash
/// riops --query "foo" --file-path ./file.txt
/// riops --query "foo" --directory ./src --ignore-case --whole-word
/// riops --query "foo" --directory ./src --extension rs --extension toml
/// ```
#[derive(Parser, Debug)]
pub struct Parameters {
    /// The search term to look for.
    #[arg(short, long)]
    pub query: String,

    /// Path to a single file to search.
    ///
    /// Mutually exclusive with `--directory`.
    #[arg(short, long = "file-path", conflicts_with = "directory")]
    pub file_path: Option<String>,

    /// Perform a case-insensitive search.
    ///
    /// Both the query and each line are lowercased before comparison.
    #[arg(short, long = "ignore-case")]
    pub ignore_case: bool,

    /// Match whole words only.
    ///
    /// The query must be surrounded by non-alphanumeric, non-underscore
    /// characters or string boundaries.
    #[arg(short, long = "whole-word")]
    pub whole_word: bool,

    /// Recursively search files inside this directory.
    ///
    /// Defaults to the current directory (`.`) when the flag is provided
    /// without a value. Mutually exclusive with `--file-path`.
    ///
    /// By default only `.txt` files are searched. Use `--extension` to
    /// target other file types.
    #[arg(short, long, num_args = 0..=1, default_missing_value = ".", conflicts_with = "file_path")]
    pub directory: Option<PathBuf>,

    /// Print a one-line summary per file instead of every matching line.
    #[arg(short, long)]
    pub simple_search: bool,

    /// Restrict directory search to files with these extensions.
    ///
    /// Can be specified multiple times to allow several extensions at once.
    /// When omitted, defaults to `.txt` only.
    ///
    /// # Examples
    ///
    /// ```bash
    /// # Search only Rust source files
    /// riops -q "fn main" -d ./src --extension rs
    ///
    /// # Search Rust and TOML files
    /// riops -q "rayon" -d . -e rs -e toml
    /// ```
    #[arg(short = 'e', long = "extension")]
    pub file_extension: Option<Vec<String>>,
}

/// Aggregates all matching lines found within a single file.
pub struct FileMatchModel {
    /// The path of the file that was searched.
    pub file_name: String,
    /// All lines within the file that matched the query.
    pub lines: Vec<LineMatchModel>,
}

impl FileMatchModel {
    /// Creates a new [`FileMatchModel`].
    ///
    /// # Arguments
    ///
    /// * `file_name` - The path or display name of the source file.
    /// * `lines`     - The collection of matching lines.
    pub fn new(file_name: String, lines: Vec<LineMatchModel>) -> Self {
        FileMatchModel { file_name, lines }
    }
}

impl Display for FileMatchModel {
    /// Formats the file match as:
    ///
    /// ```text
    /// File: <file_name>
    /// Line <n>: <content>
    /// ...
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "File: {}", self.file_name)?;
        for line in &self.lines {
            write!(f, "\n{}", line)?;
        }
        Ok(())
    }
}

/// Represents a single line in a file that matched the search query.
pub struct LineMatchModel {
    /// 1-based line number within the file.
    pub line: usize,
    /// The raw content of the matching line.
    pub content: String,
}

impl LineMatchModel {
    /// Creates a new [`LineMatchModel`].
    ///
    /// # Arguments
    ///
    /// * `line`    - 1-based line number.
    /// * `content` - The text content of the line.
    pub fn new(line: usize, content: String) -> Self {
        LineMatchModel { line, content }
    }
}

impl Display for LineMatchModel {
    /// Formats the line match as `Line <n>: <content>`.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Line {}: {}", self.line, self.content)
    }
}
