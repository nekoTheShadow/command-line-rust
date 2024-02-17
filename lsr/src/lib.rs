use std::{error::Error, path::PathBuf};

use clap::{App, Arg};
use tabular::{Row, Table};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    long: bool,
    show_hidden: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("lsr")
        .version("0.1.0")
        .author("Hajime Nakamura <h.nakamura0903@gmail.com>")
        .about("Rust ls")
        .arg(Arg::with_name("paths").value_name("PATH").help("Files and/or directories").default_value(".").multiple(true))
        .arg(Arg::with_name("long").takes_value(false).help("Long listing").short("l").long("long"))
        .arg(Arg::with_name("all").takes_value(false).help("Show all files").short("a").long("all"))
        .get_matches();

    Ok(Config {
        paths: matches.values_of_lossy("paths").unwrap(),
        long: matches.is_present("long"),
        show_hidden: matches.is_present("all"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}

fn find_files(paths: &[String], show_hidden: bool) -> MyResult<Vec<PathBuf>> {
    unimplemented!()
}

fn format_output(paths: &[PathBuf]) -> MyResult<String> {
    let fmt = "{:<}{:<}  {:>}  {:<}  {:<}  {:>}  {:<}  {:<}";
    let mut table = Table::new(fmt);
    for path in paths {
        table.add_row(Row::new().with_cell("").with_cell("").with_cell("").with_cell("").with_cell("").with_cell("").with_cell("").with_cell(""));
    }
    unimplemented!()
}

fn format_mode(mode: u32) -> String {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use crate::{find_files, format_mode};

    #[test]
    fn test_find_files() {
        let res = find_files(&["tests/inputs".to_string()], false);
        assert!(res.is_ok());
        let mut filenames = res.unwrap().iter().map(|entry| entry.display().to_string()).collect::<Vec<_>>();
        filenames.sort();
        assert_eq!(filenames, ["tests/inputs/bustle.txt", "tests/inputs/dir", "tests/inputs/empty.txt", "tests/inputs/fox.txt"]);

        let res = find_files(&["tests/inputs/.hidden".to_string()], false);
        assert!(res.is_ok());
        let filenames = res.unwrap().iter().map(|entry| entry.display().to_string()).collect::<Vec<_>>();
        assert_eq!(filenames, ["tests/inputs/.hidden"]);

        let res = find_files(&["tests/inputs/bustle.txt".to_string(), "tests/inputs/dir".to_string()], false);
        assert!(res.is_ok());
        let mut filenames = res.unwrap().iter().map(|entry| entry.display().to_string()).collect::<Vec<_>>();
        filenames.sort();
        assert_eq!(filenames, ["tests/inputs/bustle.txt", "tests/inputs/dir/spiders.txt"]);
    }

    #[test]
    fn test_find_files_hidden() {
        let res = find_files(&["tests/inputs".to_string()], true);
        assert!(res.is_ok());
        let mut filenames = res.unwrap().iter().map(|entry| entry.display().to_string()).collect::<Vec<_>>();
        filenames.sort();
        assert_eq!(filenames, ["tests/inputs/.hidden", "tests/inputs/bustle.txt", "tests/inputs/dir", "tests/inputs/empty.txt", "tests/inputs/fox.txt"]);
    }

    #[test]
    fn test_format_mode() {
        assert_eq!(format_mode(0o755), "rwxr-xr-x");
        assert_eq!(format_mode(0o421), "r---w---x");
    }
}