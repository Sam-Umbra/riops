use std::borrow::Cow;

use crate::parameter::Parameters;

pub fn search_file<'a>(params: &Parameters, file_contents: &'a str) -> Vec<&'a str> {
    let query = if params.ignore_case {
        params.query.to_lowercase()
    } else {
        params.query.clone()
    };

    file_contents
        .lines()
        .filter(|line| {
            let normalized_line: Cow<str> = if params.ignore_case {
                Cow::Owned(line.to_lowercase())
            } else {
                Cow::Borrowed(line)
            };

            if params.whole_word {
                is_whole_word_match(&normalized_line, &query)
            } else {
                normalized_line.contains(&query)
            }
        })
        .collect()
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

/* pub fn search<'a>(query: &str, file_contents: &'a str) -> Vec<&'a str> {
    file_contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, file_contents: &'a str) -> Vec<&'a str> {
    file_contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

pub fn search_whole_word<'a>(query: &str, file_contents: &'a str) -> Vec<&'a str> {
    file_contents
        .lines()
        .filter(|&line| line == query)
        .collect()
} */
