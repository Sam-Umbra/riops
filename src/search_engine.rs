//! Core search logic for single files and parallel directory traversal.

use rayon::prelude::*;
use std::{
    borrow::Cow,
    fs::File,
    io::{BufRead, BufReader, Error},
    path::PathBuf,
};
use walkdir::WalkDir;

use crate::models::{FileMatchModel, LineMatchModel, Parameters};

/// Searches a single file and returns all lines that match the query.
///
/// The file is read line-by-line using a [`BufReader`] to avoid loading
/// the entire file into memory at once. Each line is tested according to
/// the active flags in `params`:
///
/// | Flag          | Behavior                                                     |
/// |---------------|--------------------------------------------------------------|
/// | `ignore_case` | Both the query and the line are lowercased before comparison.|
/// | `whole_word`  | Delegates to [`is_whole_word_match`].                        |
/// | *(neither)*   | Plain substring search via [`str::contains`].                |
///
/// Lines that produce an I/O error during reading are silently skipped.
///
/// # Arguments
///
/// * `params` - Search configuration (query string, flags, etc.).
/// * `path`   - Filesystem path of the file to search.
///
/// # Returns
///
/// A `Vec<LineMatchModel>` containing every matching line with its 1-based
/// line number, or an [`Error`] if the file cannot be opened.
pub fn search_file(params: &Parameters, path: &str) -> Result<Vec<LineMatchModel>, Error> {
    // Normalize the query once outside the per-line loop.
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
            // Skip lines that fail to decode (e.g. invalid UTF-8).
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
                // Store the original (non-lowercased) line for display.
                Some(LineMatchModel::new(i + 1, line.to_string()))
            } else {
                None
            }
        })
        .collect();

    Ok(results)
}

/// Recursively searches files under `dir` in parallel and returns per-file
/// results sorted by file name.
///
/// ### File extension filtering
///
/// Which files are searched depends on `params.file_extension`:
///
/// | `--extension`        | Files searched                                   |
/// |----------------------|--------------------------------------------------|
/// | Not provided         | `.txt` files only (default)                      |
/// | One or more values   | Files whose extension matches any of the values  |
///
/// ### Parallelism
///
/// The directory tree is walked serially with [`WalkDir`] to collect eligible
/// paths, then all files are processed concurrently via a Rayon parallel
/// iterator. Results are sorted alphabetically after collection because
/// parallel iteration produces non-deterministic ordering.
///
/// Files that cannot be read are skipped with an error message on `stderr`.
/// Files with no matches are excluded from the returned vector.
///
/// # Arguments
///
/// * `dir`    - Root directory to walk (recursive).
/// * `params` - Search configuration forwarded to [`search_file`].
///
/// # Returns
///
/// A `Vec<FileMatchModel>` — one entry per file that contained at least one
/// match, sorted alphabetically by file path.
pub fn search_directory(dir: &PathBuf, params: &Parameters) -> Vec<FileMatchModel> {
    // Collect eligible files upfront so Rayon can distribute them across threads.
    let entries: Vec<_> = WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            if !e.path().is_file() {
                return false;
            }

            let file_ext = e.path().extension().and_then(|s| s.to_str());

            if let Some(allowed_exts) = &params.file_extension {
                // User supplied explicit extensions — match any of them.
                file_ext
                    .map(|ext| allowed_exts.iter().any(|p| p == ext))
                    .unwrap_or(false)
            } else {
                // Default: restrict to .txt files.
                file_ext == Some("txt")
            }
        })
        .collect();

    let mut result: Vec<FileMatchModel> = entries
        .into_par_iter()
        .filter_map(|entry| {
            let path = entry.path();
            let path_str = path.to_string_lossy().to_string();

            match search_file(params, &path_str) {
                Ok(lines) if !lines.is_empty() => Some(FileMatchModel::new(path_str, lines)),
                Ok(_) => None, // File opened fine but had no matches.
                Err(e) => {
                    eprintln!("Couldn't read {:?}: {}", path, e);
                    None
                }
            }
        })
        .collect();

    // Parallel iteration produces results in non-deterministic order; sort for
    // consistent, predictable output.
    result.sort_unstable_by(|a, b| a.file_name.cmp(&b.file_name));
    result
}

/// Returns `true` when `query` appears in `line` as a whole word.
///
/// A *whole-word* match requires that the characters immediately before and
/// after the occurrence are either absent (start/end of string) or are not
/// alphanumeric and not underscores — mirroring the word-boundary semantics
/// of most grep utilities.
///
/// # Arguments
///
/// * `line`  - The (possibly normalized) line to search.
/// * `query` - The (possibly normalized) search term.
///
/// # Returns
///
/// `false` when `query` is empty (avoids a match at every position).
fn is_whole_word_match(line: &str, query: &str) -> bool {
    if query.is_empty() {
        return false;
    }

    line.match_indices(query).any(|(start, _)| {
        let end = start + query.len();

        // The character *before* the match must be a word boundary (or absent).
        let before_ok = line[..start]
            .chars()
            .next_back()
            .map_or(true, |c| !c.is_alphanumeric() && c != '_');

        // The character *after* the match must be a word boundary (or absent).
        let after_ok = line[end..]
            .chars()
            .next()
            .map_or(true, |c| !c.is_alphanumeric() && c != '_');

        before_ok && after_ok
    })
}
