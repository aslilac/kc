use crate::langs::Language;
use crate::langs::LanguageSummary;
use crate::options::Options;

pub struct TotalLinesReporter;

impl TotalLinesReporter {
	pub fn report(summaries: Vec<(Language, LanguageSummary)>, _: Options) -> anyhow::Result<()> {
		let total_lines = summaries
			.iter()
			.map(|(_, summary)| summary.lines)
			.sum::<usize>();

		println!("{}", total_lines);

		Ok(())
	}
}
