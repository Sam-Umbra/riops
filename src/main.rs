use clap::Parser;
use std::{error::Error, fs, process};
use ugrep::{parameter::Parameters, search_engine::search_file};

fn main() {
    /* let params = Parameters::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    }); */
    let params = Parameters::parse();

    if let Err(e) = run(params) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

fn run(params: Parameters) -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string(&params.file_path)?;

    if params.directory.is_some() {
        
    } else {
        
    }

    let results = search_file(&params, &file_contents);

    for line in results {
        println!("{line}");
    }

    Ok(())
}
