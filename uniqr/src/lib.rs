use std::error::Error;

use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    in_file: String,
    out_file: Option<String>,
    count: bool,
}

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("uniqr")
        .version("0.1.0")
        .author("Hajime Nakamura <h.nakamura0903@gmail.com>")
        .about("Rust uniq")
        .arg(Arg::with_name("in_file").value_name("IN_FILE").help("Input file").default_value("-"))
        .arg(Arg::with_name("out_file").value_name("OUT_FILE").help("Output file"))
        .arg(Arg::with_name("count").short("c").help("Show counts").long("count").takes_value(false))
        .get_matches();
    Ok(Config{
        in_file: matches.value_of_lossy("in_file").unwrap().to_string(),
        out_file: matches.value_of_lossy("out_file").map(String::from),
        count: matches.is_present("count")
    })
}