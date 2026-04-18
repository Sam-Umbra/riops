use clap::Parser;
use riops::{
    models::Parameters,
    search_engine::{search_directory, search_file},
};
use std::{error::Error, process};

fn main() {
    let params = Parameters::parse();

    if let Err(e) = run(params) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

fn run(params: Parameters) -> Result<(), Box<dyn Error>> {
    if let Some(dir) = &params.directory {
        let file_matches = search_directory(dir, &params);
        if params.simple_search {
            for file_match in &file_matches {
                println!(
                    "{} in {}: {} occurrences(s)",
                    params.query,
                    file_match.file_name,
                    file_match.lines.len()
                )
            }
        } else {
            for file_match in file_matches {
                println!("{file_match}\n");
            }
        }
    } else {
        let path = params
            .file_path
            .as_deref()
            .ok_or("File path is required when not using --directory")?;
        let line_matches = search_file(&params, path)?;
        if params.simple_search {
            println!("{}: {} Occurrences(s)", params.query, line_matches.len())
        } else {
            for line_match in line_matches {
                println!("{line_match}");
            }
        }
    }

    Ok(())
}
