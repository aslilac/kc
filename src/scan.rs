use anyhow::anyhow;
use colored::Colorize;
use std::collections::HashMap;
use std::ffi::OsString;
use std::io;
use std::io::ErrorKind::*;
use std::path::Path;

use crate::config::default_ignore_rule;
use crate::fc::FileContent;
use crate::langs::Language;
use crate::langs::LanguageInfo;
use crate::langs::LanguageSummary;
use crate::options::Options;

pub fn scan(options: Options) -> anyhow::Result<()> {
	let mut summary: HashMap<Language, LanguageSummary> = Default::default();
	let dir = &OsString::from(&options.root_dir);
	let dir_path = Path::new(dir);

	if !dir_path.is_dir() {
		return Err(anyhow!(io::Error::new(
			NotADirectory,
			format!("{} is not a directory", dir_path.display()),
		)));
	}

	let mut walk = ignore::WalkBuilder::new(dir_path);
	walk
		.hidden(!options.include_hidden)
		.ignore(!options.include_ignored)
		.git_ignore(!options.include_ignored)
		.git_exclude(!options.include_ignored);

	// Also ignore some known obnoxious files by default
	if !options.include_ignored {
		walk.filter_entry(default_ignore_rule);
	}

	for path in walk
		.build()
		.flatten()
		.map(|entry| entry.into_path())
		.filter(|path| path.is_file())
	{
		let Ok(content) = FileContent::new(path.to_path_buf()) else {
			continue;
		};

		let summary = summary
			.entry(content.language)
			.or_insert_with(|| LanguageSummary::from(content.language));
		summary.lines += content.lines;
		summary.files.push(path);
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

	let inner_width = options.width - 2; // we have a padding of 1 character on each side

	result_iter().for_each(|(_, stat)| {
		println!(
			" {:width$}",
			stat.to_terminal_display(),
			width = inner_width
		)
	});

	let total_lines = result_iter()
		.map(|(_, stat)| stat.lines)
		.reduce(|acc, lines| acc + lines)
		.ok_or_else(|| anyhow!("no code found in \"{}\"", dir_path.display()))?;

	let mut filled = 0;

	result_iter().for_each(|(_, stat)| {
		// If there are 0 total lines, then just say everything is 0%.
		let percent = (stat.lines * inner_width)
			.checked_div(total_lines)
			.unwrap_or(0);
		let lang = LanguageInfo::from(&stat.language);

		let Some(color) = lang.color else {
			filled += percent;
			print!("{}", " ".repeat(percent).on_white());
			return;
		};

		// Print padding and such on first fill
		if filled == 0 {
			println!();
			print!(" ");
		}

		filled += percent;
		print!("{}", color.on_color(&*" ".repeat(percent)));
	});

	// Don't print a bar at all if it'd just all be uncategorized.
	if filled != 0 {
		print!("{}", " ".repeat(inner_width - filled).on_white());
		println!();
		println!();
	}

	Ok(())
}
