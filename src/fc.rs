use std::fs::read_to_string;
use std::io;
use std::io::ErrorKind::*;
use std::path::PathBuf;

use crate::langs::Language;

#[derive(Clone, Debug)]
pub struct FileContent {
	pub language: Language,
	pub lines: usize,
	pub blank_lines: usize,
}

impl FileContent {
	pub fn new(path: PathBuf) -> Result<Self, io::Error> {
		let file_name = path
			.file_name()
			.ok_or_else(|| io::Error::new(NotFound, "file must have a file name"))?;
		let language = Language::from_file_name(file_name).ok_or(io::Error::new(
			InvalidInput,
			format!("unable to determine language for {path:?}",),
		))?;

		let text = read_to_string(&path)?;
		let mut lines = 0;
		let mut blank_lines = 0;

		for line in text.lines() {
			if line.is_empty() {
				blank_lines += 1;
			}
			lines += 1;
		}

		Ok(Self {
			language,
			lines,
			blank_lines,
		})
	}
}
