use crate::color::Color;
use crate::langs::LanguageInfo;
use crate::langs::LanguageSummary;
use crate::options::Options;

pub struct HtmlReporter;

const ROW_STYLES: &str = include_str!("./html_reporter.css");

impl HtmlReporter {
	pub fn report(summaries: Vec<LanguageSummary>, options: Options) -> anyhow::Result<()> {
		println!("<!doctype html>");
		print!(
			"<html>\n<head>\n<title>{} — kc</title>\n<style>\n{}</style>\n</head>\n",
			options.root_dir.display(),
			ROW_STYLES
		);
		print!("<body>\n\n");

		let total_lines = summaries.iter().map(|it| it.lines).sum::<usize>();
		let mut remaining_lines = total_lines;
		let total_lines = total_lines as f32;

		println!("<div aria-hidden class=\"bar\">");
		{
			for stat in summaries.iter() {
				// If there are 0 total lines, then just say everything is 0%.
				let percent = stat.lines as f32 / total_lines;
				if percent.is_nan() || percent < 0.02 {
					break;
				}

				remaining_lines -= stat.lines;

				let lang = LanguageInfo::from(&stat.language);
				let color = lang
					.color
					.as_ref()
					.map(Color::hex)
					.unwrap_or("gray".to_string());

				println!(
					"\t<div aria-hidden title=\"{}\" style=\"background-color: {}; flex-grow: {}\"></div>",
					lang.name, color, stat.lines,
				);
			}

			if remaining_lines > 0 {
				println!(
					"\t<div aria-hidden title=\"Other languages\" style=\"background-color: gray; flex-grow: {remaining_lines}\"></div>",
				);
			}
		}
		print!("</div>\n\n");

		print!(
			"<table>\n\
			<colgroup><col /><col width=\"15%\" /><col width=\"15%\" /></colgroup>\n\
			\t<th>Language</th><th>Lines</th><th>Blank</th>\n\n"
		);
		{
			for stat in summaries.iter() {
				let lang = LanguageInfo::from(&stat.language);
				let color = lang
					.color
					.as_ref()
					.map(Color::hex)
					.unwrap_or("gray".to_string());
				println!(
					"\t<tr><td><span style=\"color: {}\">●</span>&nbsp;{}</td><td>{}</td><td>{}</td></tr>",
					color, stat.language, stat.lines, stat.blank_lines
				);
			}
		}
		print!("</table>\n\n");

		print!("</body>\n</html>\n");
		Ok(())
	}
}
