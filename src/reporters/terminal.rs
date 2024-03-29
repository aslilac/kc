use colored::Colorize;
use std::fmt;
use std::fmt::Display;

use crate::langs::LanguageSummary;
use crate::options::Options;

pub struct TerminalLanguageSummary<'a>(&'a LanguageSummary, bool);

impl<'a> TerminalLanguageSummary<'a> {
	pub fn new(summary: &'a LanguageSummary, options: &Options) -> TerminalLanguageSummary<'a> {
		TerminalLanguageSummary(summary, options.blame)
	}
}

impl<'a> Display for TerminalLanguageSummary<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let TerminalLanguageSummary(summary, blame) = self;

		// We have to count this length by hand because, unfortunately, escape codes count
		let left_side_width = summary.language.info().name.len() + 4; // circle + 2 leading spaces + 1 trailing space
		let right_side = format!("{}", summary.lines);
		let width = f.width().unwrap_or(0) - left_side_width - (right_side.len() + 1);
		let inlay = format!("{:.>width$}", "", width = width)
			.bright_black()
			.to_string();
		write!(f, "{} {} {}", summary.language, inlay, right_side)?;

		if *blame {
			let mut files = summary.files.iter().peekable();
			while let Some(file) = files.next() {
				let graph_char = if files.peek().is_some() { '├' } else { '└' };
				write!(f, "\n {} {}", graph_char, file.display())?;
			}
		}

		Ok(())
	}
}
