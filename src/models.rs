use std::{
    fmt::{self, Debug, Display, Formatter},
    path::PathBuf,
};

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Parameters {
    #[arg(short, long)]
    pub query: String,

    #[arg(short, long = "file-path", conflicts_with = "directory")]
    pub file_path: Option<String>,

    #[arg(short, long = "ignore-case")]
    pub ignore_case: bool,

    #[arg(short, long = "whole-word")]
    pub whole_word: bool,

    #[arg(short, long, num_args = 0..=1, default_missing_value = ".", conflicts_with = "file_path")]
    pub directory: Option<PathBuf>,

    #[arg(short, long)]
    pub simple_search: bool,

    
}

pub struct FileMatchModel {
    pub file_name: String,
    pub lines: Vec<LineMatchModel>,
}

impl FileMatchModel {
    pub fn new(file_name: String, lines: Vec<LineMatchModel>) -> Self {
        FileMatchModel { file_name, lines }
    }
}

impl Display for FileMatchModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "File: {}", self.file_name)?;
        for line in &self.lines {
            write!(f, "\n{}", line)?;
        }
        Ok(())
    }
}

pub struct LineMatchModel {
    pub line: usize,
    pub content: String,
}

impl LineMatchModel {
    pub fn new(line: usize, content: String) -> Self {
        LineMatchModel { line, content }
    }
}

impl Display for LineMatchModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Line {}: {}", self.line, self.content)
    }
}