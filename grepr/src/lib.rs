use std::{collections::btree_map::Entry, error::Error};

use clap::{App, Arg};
use regex::{Regex, RegexBuilder};


type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    pattern: Regex,
    files: Vec<String>,
    recursive: bool,
    count: bool,
    invert_match: bool
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("grepr")
        .version("0.1.0")
        .author("Hajime Nakamura <h.nakamura0903@gmail.com>")
        .about("Rust grep")
        .arg(Arg::with_name("pattern").value_name("PATTERN").help("Search pattern").required(true))
        .arg(Arg::with_name("files").value_name("FILE").help("Input file(s)").multiple(true).default_value("-"))
        .arg(Arg::with_name("insensitive").short("i").long("insensitive").help("Case-insensitive").takes_value(false))
        .arg(Arg::with_name("recursive").short("r").long("recursive").help("Recursive search").takes_value(false))
        .arg(Arg::with_name("count").short("c").long("count").help("Count occurrences").takes_value(false))
        .arg(Arg::with_name("invert").short("v").long("invert-match").help("Invert match").takes_value(false))
        .get_matches();

    let pattern = matches.value_of("pattern").unwrap();
    let pattern = RegexBuilder::new(pattern).case_insensitive(matches.is_present("insensitive")).build().map_err(|_| format!("Invalid pattern \"{}\"", pattern))?;
    Ok(Config { 
        pattern: pattern, 
        files: matches.values_of_lossy("files").unwrap(),
        recursive: matches.is_present("recursive"), 
        count: matches.is_present("count"), 
        invert_match: matches.is_present("invert"), 
    })

}

pub fn run(config: Config) -> MyResult<()> {
    let entries = find_files(&config.files, config.recursive);
    for entry in entries {
        match entry {
            Err(e) => eprintln!("{}", e),
            Ok(filename) => println!("file \"{}\"", filename),
        }
    }

    Ok(())
}

fn find_files(paths: &[String], recursive: bool) -> Vec<MyResult<String>> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use rand::{distributions::Alphanumeric, Rng};

    use crate::find_files;


    #[test]
    fn test_find_files() {
        let files = find_files(&["./tests/inputs/fox.txt".to_string()], false);
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].as_ref().unwrap(), "./tests/inputs/fox.txt");

        let files = find_files(&["./tests/inputs".to_string()], false);
        assert_eq!(files.len(), 1);
        if let Err(e) = &files[0] {
            assert_eq!(e.to_string(), "./tests/inputs is a directory");
        }

        let res = find_files(&["./tests/inputs".to_string()], true);
        let mut files = res.iter().map(|r| r.as_ref().unwrap().replace("\\", "/")).collect::<Vec<String>>();
        files.sort();
        assert_eq!(files.len(), 4);
        assert_eq!(files, vec!["./tests/inputs/bustle.txt","./tests/inputs/empty.txt","./tests/inputs/fox.txt","./tests/inputs/nobody.txt"]);

        let bad = rand::thread_rng().sample_iter(&Alphanumeric).take(7).map(char::from).collect::<String>();
        let files = find_files(&[bad], false);
        assert_eq!(files.len(), 1);
        assert!(files[0].is_err());
    }
}