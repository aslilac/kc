use std::str::FromStr;

pub mod html;
pub mod terminal;
pub mod total_lines;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reporter {
	Html,
	Terminal,
	TotalLines,
}

impl FromStr for Reporter {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_ascii_lowercase().as_ref() {
			"html" => Ok(Self::Html),
			"terminal" => Ok(Self::Terminal),
			"total" | "total_lines" | "total-lines" | "totalLines" => Ok(Self::TotalLines),
			_ => Err(()),
		}
	}
}

impl Reporter {
	pub fn help() -> &'static str {
		"\"html\", \"terminal\", \"total-lines\""
	}
}
