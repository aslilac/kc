use std::str::FromStr;

pub mod html;
pub mod json;
pub mod terminal;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reporter {
	Html,
	Json,
	Terminal,
}

impl FromStr for Reporter {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_ascii_lowercase().as_ref() {
			"html" => Ok(Self::Html),
			"json" => Ok(Self::Json),
			"terminal" => Ok(Self::Terminal),
			_ => Err(()),
		}
	}
}

impl Reporter {
	pub fn help() -> &'static str {
		"\"html\", \"json\", or \"terminal\""
	}
}
