use clap::Parser;

#[derive(Parser, Debug)]
pub struct Parameters {
    #[arg(short, long)]
    pub query: String,

    #[arg(short, long = "file-path")]
    pub file_path: String,

    #[arg(short, long = "ignore-case")]
    pub ignore_case: bool,

    #[arg(short, long = "whole-word")]
    pub whole_word: bool,

    #[arg(short, long)]
    pub directory: Option<String>,

    #[arg(short, long)]
    pub simple_search: bool,

    // TODO: Directory search

    // TODO: Simple search
    // Returns how many times the word was found, in directory search also return the file name

    // TODO: Normal Search
    // Should return the line number where the word was found, in directory search returns file name and lines

    // TODO: Integrate Threading (Use Rayon crate)
}

/* impl Parameters {
    pub fn build<T>(mut args: T) -> Result<Parameters, &'static str>
    where
        T: Iterator<Item = String>,
    {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = std::env::var("IGNORE_CASE").is_ok();

        Ok(Parameters {
            query,
            file_path,
            ignore_case,
        })
    }
} */
