use anyhow::anyhow;
use colored::Colorize;
use std::collections::HashMap;
use std::ffi::OsString;
use std::path::Path;
use std::sync::mpsc::channel;
use std::thread::spawn;

use crate::config::default_ignore_rule;
use crate::fc::FileContent;
use crate::langs::Language;
use crate::langs::LanguageInfo;
use crate::langs::LanguageSummary;
use crate::options::Options;

pub fn scan(options: Options) -> anyhow::Result<()> {
	let mut summaries: HashMap<Language, LanguageSummary> = Default::default();
	let dir = &OsString::from(&options.root_dir);
	let dir_path = Path::new(dir);

	if !dir_path.is_dir() {
		return Err(anyhow!("{} is not a directory", dir_path.display()));
	}

	let (tx, rx) = channel();
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
		let tx = tx.clone();
		spawn(move || {
			if let Ok(content) = FileContent::new(path.to_path_buf()) {
				tx.send((path, content)).unwrap();
			};
		});
	}

	// `rx` will close once all handles to `tx` have been dropped. The clones will get dropped
	// when their threads complete, which leaves this one, which we need to drop before we try to
	// drain the channel.
	drop(tx);

	while let Ok((path, content)) = rx.recv() {
		let summary = summaries
			.entry(content.language)
			.or_insert_with(|| LanguageSummary::from(content.language));
		summary.lines += content.lines;
		summary.blank_lines += content.blank_lines;
		summary.files.push(path);
	}

	let mut summaries = summaries.iter().collect::<Vec<_>>();
	summaries.sort_by(|a, b| b.1.lines.cmp(&a.1.lines));

	if !options.excluded.is_empty() {
		summaries.retain(|(lang, _)| !options.excluded.contains(lang))
	}

	if !options.only_include.is_empty() {
		summaries.retain(|(lang, _)| options.only_include.contains(lang))
	}

	if let Some(max) = &options.head {
		summaries.truncate(*max);
	}

	// We wait until here to handle `--lines` because it allows us to still respect
	// other options like `--exclude` and `--top`.
	if options.total_lines_only {
		let total_lines = summaries
			.iter()
			.map(|(_, summary)| summary.lines)
			.sum::<usize>();

		println!("{}", total_lines);
		return Ok(());
	}

	let inner_width = options.width - 2; // we have a padding of 1 character on each side

	println!();
	for (_, summary) in summaries.iter() {
		println!(
			" {:width$}",
			summary.to_terminal_display(&options),
			width = inner_width
		)
	}

	let total_lines = summaries
		.iter()
		.map(|(_, summary)| summary.lines)
		.reduce(|acc, lines| acc + lines)
		.ok_or_else(|| anyhow!("no code found in \"{}\"", dir_path.display()))?;

	let mut filled = 0;

	for (_, stat) in summaries.iter() {
		// If there are 0 total lines, then just say everything is 0%.
		let percent = (stat.lines * inner_width)
			.checked_div(total_lines)
			.unwrap_or(0);
		if percent == 0 {
			continue;
		}

		// Print padding and such on first fill
		if filled == 0 {
			println!();
			print!(" ");
		}
		filled += percent;

		let lang = LanguageInfo::from(&stat.language);
		match lang.color {
			Some(color) => print!("{}", color.on_color(&*" ".repeat(percent))),
			None => print!("{}", " ".repeat(percent).on_white()),
		};
	}

	// Don't print a bar at all if it'd just all be uncategorized.
	if filled != 0 {
		print!("{}", " ".repeat(inner_width - filled).on_white());
		println!();
		println!();
	}

	Ok(())
}
