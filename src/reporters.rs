use std::str::FromStr;

pub mod html;
pub mod markdown;
pub mod terminal;
pub mod total_lines;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reporter {
	Html,
	Markdown,
	Terminal,
	TotalLines,
}

impl FromStr for Reporter {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_ascii_lowercase().as_ref() {
			"html" => Ok(Self::Html),
			"md" | "markdown" => Ok(Self::Markdown),
			"terminal" => Ok(Self::Terminal),
			"total" | "total_lines" | "total-lines" | "totalLines" => Ok(Self::TotalLines),
			_ => Err(()),
		}
	}
}

impl Reporter {
	pub fn help() -> &'static str {
		r#""html", "markdown", "terminal", "total-lines""#
	}
}
