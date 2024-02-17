mod owner;

use std::{error::Error, fs, os::unix::fs::MetadataExt, path::PathBuf};

use chrono::{DateTime, Local};
use clap::{App, Arg};
use owner::Owner;
use tabular::{Row, Table};
use users::{get_group_by_gid, get_user_by_uid};

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
    let paths = find_files(&config.paths, config.show_hidden)?;
    if config.long {
        println!("{}", format_output(&paths)?);
    } else {
        for path in paths {
            println!("{}", path.display());
        }
    }
    Ok(())
}

fn find_files(paths: &[String], show_hidden: bool) -> MyResult<Vec<PathBuf>> {
    let mut results = vec![];
    for name in paths {
        match fs::metadata(name) {
            Err(e) => eprintln!("{}: {}", name, e),
            Ok(meta) => {
                if meta.is_dir() {
                    for entry in fs::read_dir(name)? {
                        let entry = entry?;
                        let path = entry.path();
                        let is_hidden = path.file_name().map_or(false, |file_name| file_name.to_string_lossy().starts_with("."));
                        if !is_hidden || show_hidden {
                            results.push(entry.path());
                        }
                    }
                } else {
                    results.push(PathBuf::from(name));
                }
            }
        }
    }
    Ok(results)
}

fn format_output(paths: &[PathBuf]) -> MyResult<String> {
    let fmt = "{:<}{:<}  {:>}  {:<}  {:<}  {:>}  {:<}  {:<}";
    let mut table = Table::new(fmt);
    for path in paths {
        let metadata = path.metadata()?;
        
        let uid = metadata.uid();
        let user = get_user_by_uid(uid).map(|u| u.name().to_string_lossy().into_owned()).unwrap_or_else(|| uid.to_string());
        
        let gid = metadata.gid();
        let group = get_group_by_gid(gid).map(|g| g.name().to_string_lossy().into_owned()).unwrap_or_else(|| gid.to_string());

        let file_type = if path.is_dir() { "d" } else { "-" };
        let perms = format_mode(metadata.mode());
        let modified = DateTime::<Local>::from(metadata.modified()?);

        table.add_row(Row::new()
            .with_cell(file_type)
            .with_cell(perms)
            .with_cell(metadata.nlink())
            .with_cell(user)
            .with_cell(group)
            .with_cell(metadata.len())
            .with_cell(modified.format("%b %d %y %H:%M"))
            .with_cell(path.display()));
    }
    Ok(format!("{}", table))
}

fn format_mode(mode: u32) -> String {
    format!("{}{}{}",
        mk_triple(mode, Owner::User),
        mk_triple(mode, Owner::Group),
        mk_triple(mode, Owner::Other),
    )
}

fn mk_triple(mode: u32, owner: Owner) -> String {
    let [read, write, execute] = owner.masks();
    format!("{}{}{}",
        if mode & read    == 0 {"-"} else {"r"},
        if mode & write   == 0 {"-"} else {"w"},
        if mode & execute == 0 {"-"} else {"x"},
    )
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{find_files, format_mode, format_output, mk_triple, owner::Owner};

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

    fn long_match(line: &str, expected_name: &str, expected_perms: &str, expected_size: Option<&str>) {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        assert!(parts.len() > 0 && parts.len() <= 10);

        let perms = parts.get(0).unwrap();
        assert_eq!(perms, &expected_perms);

        if let Some(size) = expected_size {
            let file_size = parts.get(4).unwrap();
            assert_eq!(file_size, &size);
        }

        let display_name = parts.last().unwrap();
        assert_eq!(display_name, &expected_name)
    }

    #[test]
    fn test_format_output_one() {
        let bustle_path = "tests/inputs/bustle.txt";
        let bustle = PathBuf::from(bustle_path);

        let res = format_output(&[bustle]);
        assert!(res.is_ok());

        let out = res.unwrap();
        let lines = out.split("\n").filter(|s| !s.is_empty()).collect::<Vec<_>>();
        assert_eq!(lines.len(), 1);

        let line1 = lines.first().unwrap();
        long_match(&line1, bustle_path, "-rw-r--r--", Some("193"));
    }

    #[test]
    fn test_format_output_two() {
        let res = format_output(&[PathBuf::from("tests/inputs/dir"), PathBuf::from("tests/inputs/empty.txt")]);
        assert!(res.is_ok());

        let out = res.unwrap();
        let mut lines = out.split("\n").filter(|s| !s.is_empty()).collect::<Vec<_>>();
        lines.sort();
        assert_eq!(lines.len(), 2);

        let empty_line = lines.remove(0);
        long_match(&empty_line, "tests/inputs/empty.txt", "-rw-r--r--", Some("0"));

        let dir_line = lines.remove(0);
        long_match(&dir_line, "tests/inputs/dir", "drwxr-xr-x", None);
    }

    #[test]
    fn test_mk_triple() {
        assert_eq!(mk_triple(0o751, Owner::User),  "rwx");
        assert_eq!(mk_triple(0o751, Owner::Group), "r-x");
        assert_eq!(mk_triple(0o751, Owner::Other), "--x");
        assert_eq!(mk_triple(0o600, Owner::Other), "---");
    }
}