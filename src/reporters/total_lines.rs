use crate::langs::LanguageSummary;
use crate::options::Options;
use std::fmt::Write;

pub struct TotalLinesReporter;

impl TotalLinesReporter {
	pub fn report(summaries: Vec<LanguageSummary>, options: Options) -> anyhow::Result<()> {
		let total_lines = summaries.iter().map(|it| it.lines).sum::<usize>();
		let mut output = String::new();
		write!(&mut output, "{total_lines}")?;
		if options.detailed {
			let total_blank_lines = summaries.iter().map(|it| it.blank_lines).sum::<usize>();
			write!(&mut output, " ({total_blank_lines} blank lines)")?;
		}
		println!("{output}");

		Ok(())
	}
}
