use crate::langs::LanguageSummary;
use crate::options::Options;
use std::fmt;
use std::fmt::Display;

pub struct MarkdownReporter {
	summaries: Vec<LanguageSummary>,
	options: Options,
}

impl MarkdownReporter {
	pub fn new(summaries: Vec<LanguageSummary>, options: Options) -> Self {
		MarkdownReporter { summaries, options }
	}
}

impl Display for MarkdownReporter {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let dir_path = &self.options.root_dir;
		if self.summaries.is_empty() {
			writeln!(f, "no code found in {}", dir_path.display())?;
			return Ok(());
		}

		let mut longest_language_name = "Language".len();
		let mut longest_line_count = "Lines".len();

		let summaries = self
			.summaries
			.iter()
			.map(|it| {
				let summary = SerializedSummary {
					language_name: it.language.to_string(),
					lines: it.lines.to_string(),
				};
				longest_language_name = summary.language_name.len().max(longest_language_name);
				longest_line_count = summary.lines.len().max(longest_line_count);
				summary
			})
			.collect::<Vec<_>>();

		writeln!(
			f,
			"| {:<lang_width$} | {:<line_count_width$} |",
			"Language",
			"Lines",
			lang_width = longest_language_name,
			line_count_width = longest_line_count,
		)?;
		let language_dashes = "-".repeat(longest_language_name);
		let lines_dashes = "-".repeat(longest_line_count);
		writeln!(f, "|-{}-|-{}-|", language_dashes, lines_dashes)?;
		for summary in summaries {
			writeln!(
				f,
				"| {:<lang_width$} | {:>line_count_width$} |",
				summary.language_name,
				summary.lines,
				lang_width = longest_language_name,
				line_count_width = longest_line_count,
			)?;
		}

		Ok(())
	}
}

struct SerializedSummary {
	language_name: String,
	lines: String,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn column_sizing() {
		let expected = include_str!("./testdata/markdown_output.md");
		let output = format!(
			"{}",
			MarkdownReporter::new(
				vec![LanguageSummary {
					language: crate::langs::Language::TypeScript,
					lines: 113997,
					blank_lines: 0,
					files: vec![],
				}],
				Options::default(),
			)
		);

		assert_eq!(output, expected);
	}
}
