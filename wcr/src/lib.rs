use std::error::Error;

use clap::{App, Arg};


type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("wcr")
        .version("0.1.0")
        .author("Hajime Nakamura <h.nakamura0903@gmail.com>")
        .about("Rust wc")
        .arg(Arg::with_name("files").value_name("FILE").help("Input file(s)").default_value("-").multiple(true))
        .arg(Arg::with_name("words").short("w").long("word").help("Show word count").takes_value(false))
        .arg(Arg::with_name("bytes").short("c").long("byte").help("Show byte count").takes_value(false))
        .arg(Arg::with_name("chars").short("m").long("char").help("Show char count").takes_value(false).conflicts_with("bytes"))
        .arg(Arg::with_name("lines").short("l").long("line").help("Show line count").takes_value(false))
        .get_matches();
    let mut lines = matches.is_present("lines");
    let mut words = matches.is_present("words");
    let mut bytes = matches.is_present("bytes");
    let mut chars = matches.is_present("chars");
    if [lines, words, bytes, chars].iter().all(|v| v==&false) {
        lines = true;
        words = true;
        bytes = true;
    }
    Ok(Config{
        files: matches.values_of_lossy("files").unwrap(),
        lines,
        words,
        bytes,
        chars,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}