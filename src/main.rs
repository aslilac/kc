use std::env;

mod color;
mod config;
mod fc;
mod langs;
mod options;
mod reporters;
mod scan;

fn main() -> anyhow::Result<()> {
	let options = env::args().skip(1).collect();
	scan::scan(options)
}
