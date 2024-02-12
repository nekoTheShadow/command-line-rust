use std::{error::Error, str::FromStr};

use chrono::{Datelike, Local, NaiveDate};
use clap::{App, Arg};


#[derive(Debug)]
pub struct Config {
    month: Option<u32>,
    year: i32,
    today: NaiveDate
}

const MONTH_NAMES: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("calr")
        .version("0.1.0")
        .author("Hajime Nakamura <h.nakamura0903@gmail.com>")
        .about("Rust cal")
        .arg(Arg::with_name("month").value_name("MONTH").short("m").help("Month name or number (1-12)").takes_value(true))
        .arg(Arg::with_name("show_current_year").value_name("SHOW_YEAR").short("y").long("year").help("Show whole current year").conflicts_with_all(&["month", "year"]).takes_value(false))
        .arg(Arg::with_name("year").value_name("YEAR").help("Year (1-9999)"))
        .get_matches();

    let mut month = matches.value_of("month").map(parse_month).transpose()?;
    let mut year = matches.value_of("year").map(parse_year).transpose()?;
    let today = Local::today();
    if matches.is_present("show_current_year") {
        month = None;
        year = Some(today.year())
    } else if month.is_none() && year.is_none() {
        month = Some(today.month());
        year = Some(today.year());
    }
    Ok(Config{
        month,
        year: year.unwrap_or_else(|| today.year()),
        today: today.naive_local()
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}

fn parse_int<T: FromStr>(val: &str) -> MyResult<T>{
    val.parse().map_err(|_| format!("Invalid integer \"{}\"", val).into())
}

fn parse_year(year: &str) -> MyResult<i32> {
    parse_int(year).and_then(|num| {
        if (1..=9999).contains(&num) {
            Ok(num)
        } else {
            Err(format!("year \"{}\" not in the range 1 through 9999", year).into())
        }
    })
}

fn parse_month(month: &str) -> MyResult<u32> {
    match parse_int(month) {
        Ok(num) => {
            if (1..=12).contains(&num) {
                Ok(num)
            } else {
                Err(format!("month \"{}\" not in the range 1 through 12", month).into())
            }
        },
        _ => {
            let lower = &month.to_lowercase();
            let matches = MONTH_NAMES.iter().enumerate().filter_map(|(i, name)| {
                if name.to_lowercase().starts_with(lower) {
                    Some(i+1)
                } else {
                    None
                }
            }).collect::<Vec<_>>();
            if matches.len() == 1 {
                Ok(matches[0] as u32)
            } else {
                Err(format!("Invalid month \"{}\"", month).into())
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse_int, parse_month, parse_year};


    #[test]
    fn test_parse_int() {
        let res = parse_int::<usize>("1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1usize);

        let res = parse_int::<i32>("-1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), -1i32);

        let res = parse_int::<i64>("foo");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Invalid integer \"foo\"");
    }

    #[test]
    fn test_parse_year() {
        let res = parse_year("1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1i32);

        let res = parse_year("9999");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 9999i32);

        let res = parse_year("0");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "year \"0\" not in the range 1 through 9999");

        let res = parse_year("10000");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "year \"10000\" not in the range 1 through 9999");

        let res = parse_year("foo");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Invalid integer \"foo\"");
    }

    #[test]
    fn test_parse_month() {
        let res = parse_month("1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1u32);

        let res = parse_month("12");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 12u32);

        let res = parse_month("jan");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1u32);

        let res = parse_month("0");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "month \"0\" not in the range 1 through 12");

        let res = parse_month("13");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "month \"13\" not in the range 1 through 12");

        let res = parse_month("foo");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Invalid month \"foo\"");
    }
}
