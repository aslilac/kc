use std::ffi::OsStr;
use std::process::exit;

use crate::langs::Language;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Options {
	pub excluded: Vec<Language>,
	pub head: Option<usize>,
	pub root_dir: String,
}

impl Default for Options {
	fn default() -> Self {
		Self {
			excluded: vec![],
			head: None,
			root_dir: ".".to_string(),
		}
	}
}

impl<S> FromIterator<S> for Options
where
	S: AsRef<str>,
{
	fn from_iter<I>(args: I) -> Self
	where
		I: IntoIterator<Item = S>,
	{
		let mut options = Options::default();
		let mut args = args.into_iter();

		while let Some(arg) = args.next() {
			let arg = arg.as_ref();
			let is_flag = (arg.len() == 2 && arg.starts_with('-'))
				|| (arg.len() > 3 && arg.starts_with("--"));

			if !is_flag {
				options.root_dir = arg.to_string();
				continue;
			}

			match arg.as_ref() {
				"-h" | "-t" | "--top" => {
					options.head = args
						.next()
						.expect(&format!("expected a number to follow {} flag", arg))
						.as_ref()
						.parse::<usize>()
						.expect(&format!("unable to parse {} as a number", arg))
						.into();
				}
				"-x" | "--exclude" => {
					let arg = args.next();
					let list = arg
						.as_ref()
						.map(|value| value.as_ref().split(","))
						.expect("expected a language identifier to follow -x flag");
					for lang in list {
						options.excluded.push(
							Language::from_extension(OsStr::new(lang))
								.expect("unrecognized language identifier"),
						);
					}
				}
				_ => {
					println!("unrecognized option: {}", arg);
					exit(1);
				}
			}
		}

		options
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use Language::*;

	#[test]
	fn from_args() {
		assert_eq!(
			["-h", "10"].into_iter().collect::<Options>(),
			Options {
				head: Some(10),
				..Default::default()
			},
		);

		assert_eq!(
			["--top", "10"].into_iter().collect::<Options>(),
			Options {
				head: Some(10),
				..Default::default()
			},
		);

		assert_eq!(
			["-x", "ts"].into_iter().collect::<Options>(),
			Options {
				excluded: vec![TypeScript],
				..Default::default()
			},
		);

		assert_eq!(
			["--exclude", "ts"].into_iter().collect::<Options>(),
			Options {
				excluded: vec![TypeScript],
				..Default::default()
			},
		);

		assert_eq!(
			["-x", "gleam", "-x", "rs,ts"]
				.into_iter()
				.collect::<Options>(),
			Options {
				excluded: vec![Gleam, Rust, TypeScript],
				..Default::default()
			},
		);

		assert_eq!(
			["./test"].into_iter().collect::<Options>(),
			Options {
				root_dir: "./test".to_string(),
				..Default::default()
			},
		);

		assert_eq!(
			["-h", "10", "./test", "-x", "ts"]
				.into_iter()
				.collect::<Options>(),
			Options {
				excluded: vec![TypeScript],
				head: Some(10),
				root_dir: "./test".to_string(),
				..Default::default()
			},
		);
	}
}
