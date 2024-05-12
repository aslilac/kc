use anyhow::anyhow;
use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::thread::spawn;

use crate::config::default_ignore_rule;
use crate::fc::FileContent;
use crate::langs::Language;
use crate::langs::LanguageSummary;
use crate::options::Options;
use crate::reporters::html::HtmlReporter;
use crate::reporters::terminal::TerminalReporter;
use crate::reporters::total_lines::TotalLinesReporter;
use crate::reporters::Reporter::*;

pub fn scan(options: Options) -> anyhow::Result<()> {
	let mut summaries: HashMap<Language, LanguageSummary> = Default::default();
	let dir_path = &options.root_dir;

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

	let mut summaries = summaries.into_iter().collect::<Vec<_>>();
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

	match options.reporter {
		Html => HtmlReporter::report(summaries, options),
		Terminal => TerminalReporter::report(summaries, options),
		TotalLines => TotalLinesReporter::report(summaries, options),
	}
}
