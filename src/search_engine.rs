use rayon::prelude::*;
use std::{
    borrow::Cow,
    fs::File,
    io::{BufRead, BufReader, Error},
    path::PathBuf,
};
use walkdir::WalkDir;

use crate::models::{FileMatchModel, LineMatchModel, Parameters};

pub fn search_file(params: &Parameters, path: &str) -> Result<Vec<LineMatchModel>, Error> {
    let query: Cow<str> = if params.ignore_case {
        Cow::Owned(params.query.to_lowercase())
    } else {
        Cow::Borrowed(&params.query)
    };

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let results = reader
        .lines()
        .enumerate()
        .filter_map(|(i, line_result)| {
            let line = line_result.ok()?;

            let normalized_line = if params.ignore_case {
                line.to_lowercase()
            } else {
                line.clone()
            };

            let matched = if params.whole_word {
                is_whole_word_match(&normalized_line, &query)
            } else {
                normalized_line.contains(query.as_ref())
            };

            if matched {
                Some(LineMatchModel::new(i + 1, line.to_string()))
            } else {
                None
            }
        })
        .collect();

    Ok(results)
}

pub fn search_directory(dir: &PathBuf, params: &Parameters) -> Vec<FileMatchModel> {
    let entries: Vec<_> = WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path().is_file() && e.path().extension().and_then(|s| s.to_str()) == Some("txt")
        })
        .collect();

    let mut result: Vec<FileMatchModel> = entries
        .into_par_iter()
        .filter_map(|entry| {
            let path = entry.path();
            let path_str = path.to_string_lossy().to_string();

            match search_file(params, &path_str) {
                Ok(lines) if !lines.is_empty() => Some(FileMatchModel::new(path_str, lines)),
                Ok(_) => None,
                Err(e) => {
                    eprintln!("Couldn't read {:?}: {}", path, e);
                    None
                }
            }
        })
        .collect();

    result.sort_unstable_by(|a, b| a.file_name.cmp(&b.file_name));
    result
}

fn is_whole_word_match(line: &str, query: &str) -> bool {
    if query.is_empty() {
        return false;
    }

    line.match_indices(query).any(|(start, _)| {
        let end = start + query.len();

        let before_ok = line[..start]
            .chars()
            .next_back()
            .map_or(true, |c| !c.is_alphanumeric() && c != '_');

        let after_ok = line[end..]
            .chars()
            .next()
            .map_or(true, |c| !c.is_alphanumeric() && c != '_');

        before_ok && after_ok
    })
}
