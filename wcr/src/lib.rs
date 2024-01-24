use std::{error::Error, fs::File, io::{self, BufRead, BufReader}};

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

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
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
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(_) => println!("Opened {}", filename),
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}

pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;
    Ok(FileInfo{
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::{count, FileInfo};

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }
}