use std::ffi::OsStr;
use std::process::exit;

use crate::langs::Language;

#[derive(Clone, Debug)]
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

impl FromIterator<String> for Options {
	fn from_iter<I>(args: I) -> Self
	where
		I: IntoIterator<Item = String>,
	{
		let mut options = Options::default();
		let mut args = args.into_iter();

		while let Some(arg) = args.next() {
			let is_flag = (arg.len() == 2 && arg.starts_with('-'))
				|| (arg.len() > 3 && arg.starts_with("--"));

			if !is_flag {
				options.root_dir = arg;
				continue;
			}

			match arg.as_ref() {
				"-h" | "-t" | "--top" => {
					options.head = args
						.next()
						.expect(&format!("expected a number to follow {} flag", arg))
						.parse::<usize>()
						.expect(&format!("unable to parse {} as a number", arg))
						.into();
				}
				"-x" | "--exclude" => {
					let arg = args.next();
					let list = arg
						.as_ref()
						.map(|value| value.split(","))
						.expect("expected a language identifier to follow -x flag");
					for lang in list {
						options.excluded.push(
							Language::from_extension(OsStr::new(&lang))
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
