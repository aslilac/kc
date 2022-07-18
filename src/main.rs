#![feature(let_else)]

mod fc;
mod langs;

use std::collections::HashMap;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::fmt;
use std::fmt::Display;
use std::fs::read;
use std::io;

use colored::Colorize;

use fc::FileContent;
use langs::Language;

use crate::langs::LanguageInfo;

#[derive(Clone, Debug)]
struct LanguageStats {
    pub language: Language,
    pub lines: usize,
}

impl Display for LanguageStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:89}  {:>9}", self.language, self.lines)
    }
}

impl LanguageStats {
    pub fn for_(language: Language) -> Self {
        Self { language, lines: 0 }
    }
}

fn scan_dir(dir: &OsStr) -> io::Result<()> {
    let mut stats = HashMap::<Language, LanguageStats>::default();

    for entry in ignore::Walk::new(dir).flatten() {
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let text = String::from_utf8(read(path)?);
        let mut content = FileContent::new(path.to_path_buf());

        // meh
        let Ok(text) = text else {
            continue;
        };

        let Some(language) = content.language else {
            continue;
        };

        content.lines = text.lines().count();
        stats
            .entry(language)
            .or_insert_with(|| LanguageStats::for_(language))
            .lines += content.lines;
        // println!("{}", content);
    }

    println!();
    let mut stats = stats.iter().collect::<Vec<_>>();
    stats.sort_by(|a, b| b.1.lines.cmp(&a.1.lines));
    stats.iter().for_each(|(_, stat)| println!(" {}", stat));

    let total_lines = stats
        .iter()
        .map(|(_, stat)| stat.lines)
        .reduce(|acc, lines| acc + lines)
        .unwrap();
    // .scan(0, |acc, (_, stat)| Some(*acc + stat.lines));
    // println!("{:?}", stats);

    let mut filled = 0;

    println!();
    print!(" ");
    stats.iter().for_each(|(_, stat)| {
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

fn main() -> io::Result<()> {
    let mut args = std::env::args().skip(1);
    let dir = args.next().unwrap_or(".".to_string());

    scan_dir(&OsString::from(&dir))?;

    Ok(())
}
