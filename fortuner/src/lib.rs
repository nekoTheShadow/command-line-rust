use std::{error::Error, path::PathBuf};

use clap::{App, Arg};
use regex::{Regex, RegexBuilder};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    sources: Vec<String>,
    pattern: Option<Regex>,
    seed: Option<u64>
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("fortuner")
        .version("0.1.0")
        .author("Hajime Nakamura <h.nakamura0903@gmail.com>")
        .about("Rust fortune")
        .arg(Arg::with_name("sources").value_name("FILE").multiple(true).required(true).help("Input files or directories"))
        .arg(Arg::with_name("pattern").value_name("PATTERN").short("m").long("pattern").help("Pattern"))
        .arg(Arg::with_name("insensitive").short("i").long("insensitive").help("Case-insensitive pattern matching").takes_value(false))
        .arg(Arg::with_name("seed").value_name("SEED").short("s").long("seed").help("Random seed"))
        .get_matches();

    let pattern = matches.value_of("pattern").map(|val| {
        RegexBuilder::new(val).case_insensitive(matches.is_present("insensitive")).build().map_err(|_| format!("Invalid --pattern \"{}\"", val))
    }).transpose()?;
    Ok(Config { sources: matches.values_of_lossy("sources").unwrap(), pattern, seed: matches.value_of("seed").map(parse_u64).transpose()? })
}

pub fn run(config: Config) -> MyResult<()> {
    let files = find_files(&config.sources);
    println!("{:#?}", files);
    Ok(())
}

fn parse_u64(val: &str) -> MyResult<u64> {
    val.parse().map_err(|_| format!("\"{}\" not a  valid integer", val).into())
}

fn find_files(paths: &[String]) -> MyResult<Vec<PathBuf>> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use crate::{find_files, parse_u64};

    #[test]
    fn test_parse_u64() {
        let res = parse_u64("a");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "\"a\" not a valid integer");

        let res = parse_u64("0");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 0);

        let res = parse_u64("4");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 4);
    }

    #[test]
    fn test_find_files() {
        let res = find_files(&["./tests/inputs/jokes".to_string()]);
        assert!(res.is_ok());
        let files = res.unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(files.get(0).unwrap().to_string_lossy(), "./tests/inputs/jokes");

        let res = find_files(&["/path/does/not/exist".to_string()]);
        assert!(res.is_err());

        let res = find_files(&["./tests/inputs".to_string()]);
        assert!(res.is_ok());
        let files = res.unwrap();
        assert_eq!(files.len(), 5);
        let first = files.get(0).unwrap().display().to_string();
        assert!(first.contains("ascii-art"));
        let last = files.last().unwrap().display().to_string();
        assert!(last.contains("quotes"));

        let res = find_files(&[
            "./tests/inputs/jokes".to_string(),
            "./tests/inputs/ascii-art".to_string(),
            "./tests/inputs/jokes".to_string(),
        ]);
        assert!(res.is_ok());
        let files = res.unwrap();
        assert_eq!(files.len(), 2);
        if let Some(filename) = files.first().unwrap().file_name() {
            assert_eq!(filename.to_string_lossy(), "ascii-art".to_string())
        }
        if let Some(filename) = files.last().unwrap().file_name() {
            assert_eq!(filename.to_string_lossy(), "jokes".to_string())
        }
    }

}