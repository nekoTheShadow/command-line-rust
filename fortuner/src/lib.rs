use std::error::Error;

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
    println!("{:#?}", config);
    Ok(())
}

fn parse_u64(val: &str) -> MyResult<u64> {
    val.parse().map_err(|_| format!("\"{}\" not a  valid integer", val).into())
}

#[cfg(test)]
mod tests {
    use crate::parse_u64;

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
}