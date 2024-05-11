use anyhow::anyhow;
use colored::Colorize;
use std::fmt;
use std::fmt::Display;
use terminal_size::terminal_size;
use terminal_size::Width;

use crate::langs::Language;
use crate::langs::LanguageInfo;
use crate::langs::LanguageSummary;
use crate::options::Options;

pub struct TerminalReporter;

impl TerminalReporter {
	pub fn report(
		summaries: Vec<(Language, LanguageSummary)>,
		options: Options,
	) -> anyhow::Result<()> {
		let dir_path = &options.root_dir;
		let term_size = terminal_size();
		let width = match term_size {
			Some((Width(w), _)) => w.into(),
			None => 80,
		};
		let inner_width = width - 2; // we have a padding of 1 character on each side

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
}

pub struct TerminalLanguageSummary<'a, 'b>(&'a LanguageSummary, &'b Options);

impl<'a, 'b> TerminalLanguageSummary<'a, 'b> {
	pub fn new(
		summary: &'a LanguageSummary,
		options: &'b Options,
	) -> TerminalLanguageSummary<'a, 'b> {
		TerminalLanguageSummary(summary, options)
	}
}

impl<'a, 'b> Display for TerminalLanguageSummary<'a, 'b> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let TerminalLanguageSummary(summary, options) = self;

		// We have to count this length by hand because, unfortunately, escape codes count
		let left_side_width = summary.language.info().name.len() + 4; // circle + 2 leading spaces + 1 trailing space
		let right_side = if options.detailed {
			format!("{} - {}", summary.lines, summary.blank_lines)
		} else {
			format!("{}", summary.lines)
		};
		let width = f.width().unwrap_or(0) - left_side_width - (right_side.len() + 1);
		let inlay = format!("{:.>width$}", "", width = width)
			.bright_black()
			.to_string();
		write!(f, "{} {} {}", summary.language, inlay, right_side)?;

		if options.blame {
			let mut files = summary.files.iter().peekable();
			while let Some(file) = files.next() {
				let graph_char = if files.peek().is_some() { '├' } else { '└' };
				write!(f, "\n {} {}", graph_char, file.display())?;
			}
		}

		Ok(())
	}
}
