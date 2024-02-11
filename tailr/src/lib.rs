use std::{error::Error, fs::File, io::BufRead};

use clap::{App, Arg};
use once_cell::sync::OnceCell;
use regex::Regex;

type MyResult<T> = Result<T, Box<dyn Error>>;

static NUM_RE: OnceCell<Regex> = OnceCell::new();

#[derive(Debug, PartialEq)]
enum TakeValue {
    PlusZero,
    TakeNum(i64),
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: TakeValue,
    bytes: Option<TakeValue>,
    quiet: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("tailr")
        .version("0.1.0")
        .author("Hajime Nakamura <h.nakamura0903@gmail.com>")
        .about("Rust tail")
        .arg(Arg::with_name("files").value_name("FILE").help("Input file(s)").required(true).multiple(true))
        .arg(Arg::with_name("lines").short("n").long("lines").value_name("LINES").help("Number of lines").default_value("10"))
        .arg(Arg::with_name("bytes").short("c").long("bytes").value_name("BYTES").conflicts_with("lines").help("Number of bytes"))
        .arg(Arg::with_name("quiet").short("q").long("quiet").help("Suppress headers"))
        .get_matches();

    let lines = matches.value_of("lines").map(parse_num).transpose().map_err(|e| format!("illegal line count -- {}", e))?;
    let bytes = matches.value_of("bytes").map(parse_num).transpose().map_err(|e| format!("illegal byte count -- {}", e))?;
    Ok(Config { files: matches.values_of_lossy("files").unwrap(), lines: lines.unwrap(), bytes, quiet: matches.is_present("quiet") })
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files.iter() {
        match File::open(filename) {
            Ok(_) => {
                let (total_lines, total_bytes) = count_lines_bytes(filename)?;
                println!("{} has {} lines and {} bytes", filename, total_lines, total_bytes);
            },
            Err(e) => eprintln!("{}: {}", filename, e),
        }
    }
    Ok(())
}

fn parse_num(val: &str) -> MyResult<TakeValue> {
    let num_re = NUM_RE.get_or_init(|| Regex::new(r"^([+-])?(\d+)$").unwrap());
    match num_re.captures(val) {
        Some(caps) => {
            let sign = caps.get(1).map_or("-", |m| m.as_str());
            let num = format!("{}{}", sign, caps.get(2).unwrap().as_str());
            if let Ok(val) = num.parse() {
                if sign=="+" && val==0 {
                    Ok(TakeValue::PlusZero)
                } else {
                    Ok(TakeValue::TakeNum(val))
                } 
            } else {
                Err(From::from(val))
            }
        },
        _ => Err(From::from(val))
    }
}

fn count_lines_bytes(filename: &str) -> MyResult<(i64, i64)> {
    unimplemented!()
}

fn print_lines(mut file: impl BufRead, num_lines: &TakeValue, total_lines: i64) -> MyResult<()> {
    unimplemented!()
}

fn get_start_index(tak_val: &TakeValue, total: i64) -> Option<u64> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use crate::get_start_index;
    use crate::parse_num;
    use crate::count_lines_bytes;
    use crate::TakeValue::*;

    
    #[test]
    fn test_parse_num() {
        let res = parse_num("3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(-3));

        let res = parse_num("+3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(3));

        let res = parse_num("-3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(-3));

        let res = parse_num("0");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(0));

        let res = parse_num("+0");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), PlusZero);

        let res = parse_num(&i64::MAX.to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(i64::MIN + 1));

        let res = parse_num(&(i64::MIN + 1).to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(i64::MIN + 1));

        let res = parse_num(&format!("+{}", i64::MAX));
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(i64::MAX));

        let res = parse_num(&i64::MIN.to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(i64::MIN));

        let res = parse_num("3.14");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "3.14");

        let res = parse_num("foo");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "foo");
    }

    #[test]
    fn test_count_lines_bytes() {
        let res = count_lines_bytes("tests/inputs/one.txt");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), (1, 24));

        let res = count_lines_bytes("tests/inputs/ten.txt");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), (10, 49));
    }

    #[test]
    fn test_get_start_index() {
        assert_eq!(get_start_index(&PlusZero, 0), None);
        assert_eq!(get_start_index(&PlusZero, 1), Some(0));
        assert_eq!(get_start_index(&TakeNum(0), 1), None);
        assert_eq!(get_start_index(&TakeNum(1), 0), None);
        assert_eq!(get_start_index(&TakeNum(2), 1), None);

        assert_eq!(get_start_index(&TakeNum(1), 10), Some(0));
        assert_eq!(get_start_index(&TakeNum(2), 10), Some(1));
        assert_eq!(get_start_index(&TakeNum(3), 10), Some(2));

        assert_eq!(get_start_index(&TakeNum(-1), 10), Some(9));
        assert_eq!(get_start_index(&TakeNum(-2), 10), Some(8));
        assert_eq!(get_start_index(&TakeNum(-3), 10), Some(7));

        assert_eq!(get_start_index(&TakeNum(-20), 10), Some(0));
    }
}