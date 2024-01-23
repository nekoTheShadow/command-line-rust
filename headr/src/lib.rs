use std::error::Error;
use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}


pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("Hajime Nakamura <h.nakamura0903@gmail.com>")
        .about("Rust head")
        .arg(Arg::with_name("lines").short("n").long("lines").value_name("LINES").help("Number of lines").default_value("10"))
        .arg(Arg::with_name("bytes").short("c").long("bytes").value_name("BYTES").takes_value(true).conflicts_with("lines").help("Number of lines"))
        .arg(Arg::with_name("files").value_delimiter("FILE").help("Input files(s)").multiple(true).default_value("-"))
        .get_matches();

    let lines = matches.value_of("lines").map(parse_positive_int).transpose().map_err(|e| format!("illegal line count -- {}", e))?;
    let bytes = matches.value_of("bytes").map(parse_positive_int).transpose().map_err(|e| format!("illegal byte count -- {}", e))?;
    Ok(Config{
        files : matches.values_of_lossy("files").unwrap(),
        lines : lines.unwrap(),
        bytes
    })
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val)),
    }
}

#[test]
fn test_parse_positive_int() {
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}