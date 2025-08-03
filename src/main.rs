use std::env;

mod color;
mod config;
mod fc;
mod langs;
mod options;
mod reporters;
mod scan;

fn main() -> anyhow::Result<()> {
	let options = options::Options::from(env::args().skip(1))?;
	scan::scan(options)
}
