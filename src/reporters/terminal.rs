use colored::Colorize;
use std::fmt;
use std::fmt::Display;

use crate::langs::LanguageSummary;

pub struct TerminalLanguageSummary<'a>(&'a LanguageSummary);

impl<'a> TerminalLanguageSummary<'a> {
	pub fn new(summary: &'a LanguageSummary) -> TerminalLanguageSummary<'a> {
		TerminalLanguageSummary(summary)
	}
}

impl<'a> Display for TerminalLanguageSummary<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let TerminalLanguageSummary(summary) = self;

		// We have to count this length by hand because, unfortunately, escape codes count
		let left_side_width = summary.language.info().name.len() + 4; // circle + 2 leading spaces + 1 trailing space
		let right_side = format!("{}", summary.lines);
		let width = f.width().unwrap_or(0) - left_side_width - (right_side.len() + 1);
		let inlay = format!("{:.>width$}", "", width = width)
			.bright_black()
			.to_string();
		write!(f, "{} {} {}", summary.language, inlay, right_side)?;

		// for file in &summary.files {
		// 	write!(f, "   - {}\n", file.display())?;
		// }

		Ok(())
	}
}
