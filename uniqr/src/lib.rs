use std::{error::Error, fs::File, io::{self, BufRead, BufReader, Write}};

use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    in_file: String,
    out_file: Option<String>,
    count: bool,
}

pub fn run(config: Config) -> MyResult<()> {
    let mut out_file: Box<dyn Write>  = match &config.out_file {
        Some(out_name) => Box::new(File::create(out_name)?),
        _ => Box::new(io::stdout()),
    };
    let mut print = |count: u64, text: &str| -> MyResult<()> {
        if count > 0 {
            if config.count {
                write!(out_file, "{:>4} {}", count, text)?;
            } else {
                write!(out_file, "{}", text)?;
            }
        }
        Ok(())
    };


    let mut file = open(&config.in_file).map_err(|e| format!("{}: {}", config.in_file, e))?;
    let mut line = String::new();
    let mut previous = String::new();
    let mut count = 0;

    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }

        if line.trim_end() != previous.trim_end() {
            print(count, &previous)?;
            previous = line.clone();
            count = 0;
        }

        count += 1;
        line.clear();
    }

    print(count, &previous)?;

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

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}