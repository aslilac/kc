use colored::Colorize;
use std::collections::HashSet;
use std::ffi::OsStr;
use std::process::exit;
use terminal_size::terminal_size;
use terminal_size::Height;
use terminal_size::Width;

use crate::langs::Language;
use crate::reporters::Reporter;
use crate::reporters::Reporter::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Options {
	pub root_dir: String,
	pub width: usize,
	pub reporter: Reporter,
	pub include_hidden: bool,
	pub include_ignored: bool,
	pub blame: bool,
	pub head: Option<usize>,
	pub excluded: HashSet<Language>,
	pub only_include: HashSet<Language>,
	pub total_lines_only: bool,
}

impl Default for Options {
	fn default() -> Self {
		let term_size = terminal_size();
		let width = match term_size {
			Some((Width(w), Height(_))) => w.into(),
			None => 100,
		};

		Self {
			root_dir: ".".to_string(),
			width,
			reporter: Terminal,
			include_hidden: false,
			include_ignored: false,
			blame: false,
			head: None,
			excluded: Default::default(),
			only_include: Default::default(),
			total_lines_only: false,
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
			let is_flag =
				(arg.len() >= 2 && arg.starts_with('-')) || (arg.len() >= 3 && arg.starts_with("--"));

			if !is_flag {
				options.root_dir = arg.to_string();
				continue;
			}

			match arg.as_ref() {
				"-v" | "-V" | "-version" | "--version" => {
					println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
					exit(0);
				}
				"-h" | "-help" | "--help" | "-?" => {
					println!(
						"{} {}",
						env!("CARGO_PKG_NAME").bold().magenta(),
						env!("CARGO_PKG_VERSION").bold().magenta()
					);
					println!("{}", include_str!("./help.txt"));
					exit(0);
				}
				"-reporter" | "--reporter" => {
					options.reporter = args
						.next()
						.expect(&format!("expected a reporter to follow {} flag", arg))
						.as_ref()
						.parse::<Reporter>()
						.expect(&format!("{} flag expects one of {}", arg, Reporter::help()));
				}
				"-a" => {
					options.include_hidden = true;
				}
				"-A" => {
					options.include_ignored = true;
				}
				"-aA" | "-Aa" | "-aa" | "-AA" => {
					options.include_hidden = true;
					options.include_ignored = true;
				}
				"-blame" | "--blame" => {
					options.blame = true;
				}
				"-t" | "-top" | "--top" => {
					options.head = args
						.next()
						.expect(&format!("expected a number to follow {} flag", arg))
						.as_ref()
						.parse::<usize>()
						.expect(&format!("unable to parse \"{}\" as a number", arg))
						.into();
				}
				"-x" | "-exclude" | "--exclude" => {
					let exclusions = args.next();
					let list = exclusions
						.as_ref()
						.expect(&format!("expected a language to follow {} flag", arg))
						.as_ref()
						.split(",");
					for lang in list {
						options.excluded.insert(
							Language::from_name(lang)
								.or_else(|| Language::from_extension(OsStr::new(lang)))
								.expect(&format!("unrecognized language identifier \"{}\"", lang)),
						);
					}
				}
				"-o" | "-only" | "--only" => {
					let include = args.next();
					let list = include
						.as_ref()
						.expect(&format!("expected a language to follow {} flag", arg))
						.as_ref()
						.split(",");
					for lang in list {
						options.only_include.insert(
							Language::from_name(lang)
								.or_else(|| Language::from_extension(OsStr::new(lang)))
								.expect(&format!("unrecognized language identifier \"{}\"", lang)),
						);
					}
				}
				"-l" | "-lines" | "--lines" => {
					if options.head.is_some() {
						println!("{} is incompatible with -t/--top", arg);
						exit(64);
					}

					options.total_lines_only = true;
				}
				_ => {
					println!("unrecognized option: {}", arg);
					exit(1);
				}
			}
		}

		if !options.only_include.is_empty() && !options.excluded.is_empty() {
			println!("warning: both --only and --exclude have been set, which doesn't really make sense")
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
				excluded: [TypeScript].into(),
				..Default::default()
			},
		);

		assert_eq!(
			["--exclude", "ts"].into_iter().collect::<Options>(),
			Options {
				excluded: [TypeScript].into(),
				..Default::default()
			},
		);

		assert_eq!(
			["-x", "BQN,TypeScript"].into_iter().collect::<Options>(),
			Options {
				excluded: [Bqn, TypeScript].into(),
				..Default::default()
			},
		);

		assert_eq!(
			["-x", "gleam", "-x", "rs,ts"]
				.into_iter()
				.collect::<Options>(),
			Options {
				excluded: [Gleam, Rust, TypeScript].into(),
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
				excluded: [TypeScript].into(),
				head: Some(10),
				root_dir: "./test".to_string(),
				..Default::default()
			},
		);

		assert_eq!(
			["-l"].into_iter().collect::<Options>(),
			Options {
				total_lines_only: true,
				..Default::default()
			},
		);
	}
}
