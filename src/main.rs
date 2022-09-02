#![feature(let_else)]

use ::colored::Colorize;
use ::std::collections::HashMap;
use ::std::ffi::OsStr;
use ::std::ffi::OsString;
use ::std::io;
use ::std::path::PathBuf;
use ::std::process::exit;

mod fc;
mod langs;
use fc::FileContent;
use langs::Language;
use langs::LanguageInfo;
use langs::LanguageSummary;

fn ignored(path: &PathBuf) -> bool {
    path.file_name() == Some(OsStr::new("package-lock.json"))
}

fn scan_dir(options: Options) -> io::Result<()> {
    let dir = &OsString::from(&options.root_dir);
    let mut summary = HashMap::<Language, LanguageSummary>::default();

    for path in ignore::Walk::new(dir)
        .flatten()
        .map(|entry| entry.into_path())
        .filter(|path| path.is_file())
    {
        if ignored(&path) {
            continue;
        }

        let Ok(content) = FileContent::new(path.to_path_buf()) else {
            continue;
        };

        summary
            .entry(content.language)
            .or_insert_with(|| LanguageSummary::from(content.language))
            .lines += content.lines;
    }

    println!();
    let mut summary = summary.iter().collect::<Vec<_>>();
    summary.sort_by(|a, b| b.1.lines.cmp(&a.1.lines));

    let result_iter = || {
        let mut count = 0;
        summary
            .iter()
            .filter(|(lang, _)| !options.excluded.contains(lang))
            .take_while(move |_| {
                if let Some(max) = &options.head {
                    if count >= *max {
                        return false;
                    }

                    count += 1;
                }

                true
            })
    };

    result_iter().for_each(|(_, stat)| println!(" {}", stat));

    let total_lines = result_iter()
        .map(|(_, stat)| stat.lines)
        .reduce(|acc, lines| acc + lines)
        .unwrap_or(0);

    let mut filled = 0;

    println!();
    print!(" ");
    result_iter().for_each(|(_, stat)| {
        let percent = stat.lines * 100 / total_lines;
        let lang = LanguageInfo::from(&stat.language);

        let Some(color) = lang.color else {
            return;
        };

        filled += percent;

        print!("{}", color.on_color(&*" ".repeat(percent)));
    });

    print!("{}", " ".repeat(100 - filled).on_white());

    println!();
    println!();

    Ok(())
}

#[derive(Clone, Debug)]
struct Options {
    excluded: Vec<Language>,
    head: Option<usize>,
    root_dir: String,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            excluded: vec![],
            head: None,
            root_dir: ".".to_string(),
        }
    }
}

fn main() -> io::Result<()> {
    let mut options = Options::default();

    let mut args = std::env::args().skip(1);
    // let dir = args.next().unwrap_or(".".to_string());

    while let Some(arg) = args.next() {
        if (arg.len() == 2 && arg.starts_with('-')) || args.len() > 3 && arg.starts_with("--") {
            match arg.as_ref() {
                "-h" | "-t" | "--top" => {
                    options.head = args
                        .next()
                        .expect(&format!("Expected a number to follow {} flag", arg))
                        .parse::<usize>()
                        .expect(&format!("Unable to parse {} as a number", arg))
                        .into();
                }
                "-x" | "--exclude" => {
                    let arg = args.next();
                    let list = arg
                        .as_ref()
                        .map(|value| value.split(","))
                        .expect("Expected a language identifier to follow -x flag");
                    for lang in list {
                        options.excluded.push(
                            Language::from_extension(OsStr::new(&lang))
                                .expect("Unrecognized language identifier"),
                        );
                    }
                }
                _ => {
                    println!("Unrecognized option: {}", arg);
                    exit(1);
                }
            }
        } else {
            options.root_dir = arg;
        }
    }

    // println!("{:?}", options);
    scan_dir(options)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn scan_is_ok_on_missing_dirs() {
       let options = Options { root_dir: "/nope_i_am_missing".to_string(), ..Options::default() };
       let result = scan_dir(options).map_err(|e| e.kind());
       let expected = Ok(());
       assert_eq!(expected, result);
    }

    #[test]
    fn scan_is_ok_on_empty_dirs() {
       let empty_path = "./test_fixtures/empty";
       fs::create_dir_all(empty_path).unwrap();

       let options = Options { root_dir: empty_path.to_string(), ..Options::default() };
       let result = scan_dir(options).map_err(|e| e.kind());
       let expected = Ok(());
       assert_eq!(expected, result);
    }
}

