#![feature(io_error_more)]

use colored::Colorize;
use std::collections::HashMap;
use std::env;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::io;
use std::path::Path;
use std::path::PathBuf;

mod fc;
mod langs;
mod options;
use fc::FileContent;
use langs::Language;
use langs::LanguageInfo;
use langs::LanguageSummary;
use options::Options;

fn ignored(path: &PathBuf) -> bool {
	path.file_name() == Some(OsStr::new("package-lock.json"))
}

fn scan_dir(options: Options) -> io::Result<()> {
	let mut summary: HashMap<Language, LanguageSummary> = Default::default();
	let dir = &OsString::from(&options.root_dir);
	let dir_path = Path::new(dir);

	if !dir_path.is_dir() {
		return Err(io::Error::new(
			io::ErrorKind::NotADirectory,
			format!("{} is not a directory", dir_path.display()),
		));
	}

	for path in ignore::Walk::new(dir_path)
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
		.expect(&format!("no code found in {}", dir_path.display()));

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

fn main() -> io::Result<()> {
	let options = env::args().skip(1).collect();
	scan_dir(options)
}
