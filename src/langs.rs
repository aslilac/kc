use colored::Colorize;
use std::ffi::OsStr;
use std::fmt;
use std::fmt::Display;
use std::path::Path;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Language {
	Assembly,
	Astro,
	Bash,
	Brainfuck,
	C,
	Carbon,
	Catrina,
	Clojure,
	CMake,
	Co,
	Cobol,
	CommonLisp,
	Css,
	Cue,
	Cxx,
	CoffeeScript,
	Cognate,
	Crystal,
	CSharp,
	D,
	Dart,
	Dhall,
	Elm,
	Elixir,
	Erlang,
	Fortran,
	FSharp,
	Gleam,
	Gn,
	Go,
	Grain,
	GraphQl,
	Gren,
	Hare,
	Haskell,
	Html,
	Idris,
	Io,
	Jai,
	Jakt,
	Java,
	JavaScript,
	Json,
	Julia,
	Koka,
	Kotlin,
	Llvm,
	Lua,
	Make,
	Markdown,
	Nim,
	NuShell,
	Oak,
	ObjectiveC,
	ObjectiveCxx,
	OCaml,
	Odin,
	Pascal,
	Php,
	Perl,
	Porth,
	PowerShell,
	Prolog,
	PureScript,
	Python,
	Terraform,
	Turquoise,
	Racket,
	Raku,
	Reason,
	Ren,
	ReScript,
	Ruby,
	Roc,
	Rust,
	Sass,
	Scala,
	Sql,
	Svelte,
	Swift,
	Tcl,
	Toml,
	TypeScript,
	Unison,
	V,
	Val,
	Vala,
	Vale,
	VisualBasic,
	Vue,
	WebAssembly,
	Wren,
	Xml,
	Yall,
	Yaml,
	YueScript,
	Zig,
}

impl Language {
	pub fn info(&self) -> LanguageInfo {
		LanguageInfo::from(self)
	}
}

impl Display for Language {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let info = LanguageInfo::from(self);

		write!(
			f,
			"{}  {}",
			info.color
				.map(|color| color.color("●"))
				.unwrap_or_else(|| "●".to_string()),
			&info.name
		)
	}
}

impl Language {
	pub fn from_file_name<S>(file_name: S) -> Option<Self>
	where
		S: AsRef<OsStr>,
	{
		use Language::*;

		match file_name.as_ref().to_str()? {
			"CMakeLists.txt" => Some(CMake),
			"Makefile" => Some(Make),
			_ => Path::new(file_name.as_ref())
				.extension()
				.and_then(Language::from_extension),
		}
	}

	pub fn from_extension<S>(ext: S) -> Option<Self>
	where
		S: AsRef<OsStr>,
	{
		use Language::*;

		match ext.as_ref().to_str()? {
			"asm" => Some(Assembly),
			"astro" => Some(Astro),
			"b" => Some(Brainfuck),
			"c" => Some(C),
			"carbon" => Some(Carbon),
			"cbl" => Some(Cobol),
			"cc" => Some(Cxx),
			"cjs" => Some(JavaScript),
			"cl" => Some(CommonLisp),
			"clj" => Some(Clojure),
			"co" => Some(Co),
			"coffee" => Some(CoffeeScript),
			"cog" => Some(Cognate),
			"cpp" => Some(Cxx),
			"cpp2" => Some(Cxx),
			"cr" => Some(Crystal),
			"cs" => Some(CSharp),
			"css" => Some(Css),
			"cts" => Some(TypeScript),
			"cue" => Some(Cue),
			"cxx" => Some(Cxx),
			"d" => Some(D),
			"dart" => Some(Dart),
			"dhall" => Some(Dhall),
			"elm" => Some(Elm),
			"erl" => Some(Erlang),
			"ex" => Some(Elixir),
			"exs" => Some(Elixir),
			"f" => Some(Fortran),
			"for" => Some(Fortran),
			"fs" => Some(FSharp),
			"f90" => Some(Fortran),
			"f95" => Some(Fortran),
			"f03" => Some(Fortran),
			"gleam" => Some(Gleam),
			"gn" => Some(Gn),
			"go" => Some(Go),
			"gql" => Some(GraphQl),
			"gr" => Some(Grain),
			"gren" => Some(Gren),
			"h" => Some(C),
			"ha" => Some(Hare),
			"hh" => Some(Cxx),
			"hpp" => Some(Cxx),
			"hs" => Some(Haskell),
			"htm" => Some(Html),
			"html" => Some(Html),
			"hxx" => Some(Cxx),
			"idr" => Some(Idris),
			"io" => Some(Io),
			"jai" => Some(Jai),
			"jakt" => Some(Jakt),
			"java" => Some(Java),
			"jl" => Some(Julia),
			"js" => Some(JavaScript),
			"json" => Some(Json),
			"jsonc" => Some(Json),
			"jsx" => Some(JavaScript),
			"kk" => Some(Koka),
			"kt" => Some(Kotlin),
			"kts" => Some(Kotlin),
			"l" => Some(CommonLisp),
			"lisp" => Some(CommonLisp),
			"ll" => Some(Llvm),
			"lsp" => Some(CommonLisp),
			"lua" => Some(Lua),
			"m" => Some(ObjectiveC),
			"md" => Some(Markdown),
			"mjs" => Some(JavaScript),
			"mk" => Some(Make),
			"ml" => Some(OCaml),
			"mm" => Some(ObjectiveCxx),
			"mts" => Some(TypeScript),
			"nim" => Some(Nim),
			"nu" => Some(NuShell),
			"oak" => Some(Oak),
			"odin" => Some(Odin),
			"pas" => Some(Pascal),
			"php" => Some(Php),
			"pl" => Some(Perl),
			"pm" => Some(Perl),
			"pl6" => Some(Raku),
			"pm6" => Some(Raku),
			"porth" => Some(Porth),
			"pro" => Some(Prolog),
			"ps1" => Some(PowerShell),
			"purs" => Some(PureScript),
			"py" => Some(Python),
			"p6" => Some(Raku),
			"q" => Some(Turquoise),
			"raku" => Some(Raku),
			"rakumod" => Some(Raku),
			"rb" => Some(Ruby),
			"re" => Some(Reason),
			"ren" => Some(Ren),
			"res" => Some(ReScript),
			"rina" => Some(Catrina),
			"rkt" => Some(Racket),
			"roc" => Some(Roc),
			"rs" => Some(Rust),
			"s" => Some(Assembly),
			"sass" => Some(Sass),
			"scala" => Some(Scala),
			"sh" => Some(Bash),
			"sql" => Some(Sql),
			"svelte" => Some(Svelte),
			"swift" => Some(Swift),
			"tcl" => Some(Tcl),
			"tf" => Some(Terraform),
			"toml" => Some(Toml),
			"ts" => Some(TypeScript),
			"tsx" => Some(TypeScript),
			"u" => Some(Unison),
			"v" => Some(V),
			"val" => Some(Val),
			"vala" => Some(Vala),
			"vale" => Some(Vale),
			"vb" => Some(VisualBasic),
			"vue" => Some(Vue),
			"wat" => Some(WebAssembly),
			"wren" => Some(Wren),
			"xml" => Some(Xml),
			"yall" => Some(Yall),
			"yaml" => Some(Yaml),
			"yml" => Some(Yaml),
			"yue" => Some(YueScript),
			"zig" => Some(Zig),
			_ => None,
		}
	}
}

#[derive(Clone, Debug)]
pub struct LanguageInfo {
	pub name: String,
	pub color: Option<Color>,
}

macro_rules! info {
	( $n:expr $(,)? ) => {{
		LanguageInfo {
			name: $n.into(),
			color: None,
		}
	}};

	( $n:expr , color: $c:expr $(,)? ) => {{
		LanguageInfo {
			name: $n.into(),
			color: Some($c.into()),
		}
	}};
}

impl LanguageInfo {
	pub fn from(lang: &Language) -> Self {
		use Language::*;

		match lang {
			Assembly => info!("Assembly"),
			Astro => info!("Astro", color: 0xa78bfa),
			Bash => info!("Bash", color: [50, 50, 50]),
			Brainfuck => info!("Brainfuck"),
			// C => info!("C", color: [163, 176, 240]),
			C => info!("C", color: [40, 48, 126]),
			Carbon => info!("Carbon"),
			Catrina => info!("Catrina", color: [255, 105, 180]),
			Clojure => info!("Clojure", color: [0, 112, 255]),
			CMake => info!("CMake"),
			Co => info!("Co"),
			Cobol => info!("Cobol", color: [0, 112, 255]),
			CoffeeScript => info!("CoffeeScript", color: 0x3e2723),
			Cognate => info!("Cognate"),
			CommonLisp => info!("CommonLisp"),
			Crystal => info!("Crystal", color: 0x000000),
			CSharp => info!("C#", color: [5, 142, 12]),
			Css => info!("CSS", color: 0x563d7c),
			Cue => info!("Cue"),
			Cxx => info!("C++", color: [25, 65, 122]),
			D => info!("D", color: 0xb03931),
			Dart => info!("Dart", color: 0x40c4ff),
			Dhall => info!("Dhall"),
			Elm => info!("Elm", color: 0x60b5cc),
			Elixir => info!("Elixir", color: 0x4e2a8e),
			Erlang => info!("Erlang", color: 0xa2003e),
			FSharp => info!("F#", color: 0xb845fc),
			Fortran => info!("Fortran"),
			Gleam => info!("Gleam", color: 0xffaff3),
			Gn => info!("gn"),
			Go => info!("Go", color: 0x00add8),
			Grain => info!("Grain", color: [255, 133, 14]),
			GraphQl => info!("GraphQL", color: 0xe10098),
			Gren => info!("Gren", color: 0xff6600),
			Hare => info!("Hare", color: 0x121415),
			Haskell => info!("Haskell", color: 0x6144b3),
			Html => info!("HTML", color: 0xdf6e3c),
			Idris => info!("Idris", color: 0xc74350),
			Io => info!("Io", color: 0xa9188d),
			Jai => info!("Jai"),
			Jakt => info!("Jakt", color: [255, 0, 0]), // TODO: bad
			// Java => info!("Java", color: [205, 112, 42]),
			Java => info!("Java", color: [205, 55, 47]),
			JavaScript => info!("JavaScript", color: 0xf1e05a),
			Json => info!("JSON"),
			Julia => info!("Julia", color: 0xa270ba),
			Koka => info!("Koka"),
			// Kotlin => info!("Kotlin", color: 0x7f52ff),
			Kotlin => info!("Kotlin", color: 0xa97bff),
			Llvm => info!("LLVM IR"),
			Lua => info!("Lua", color: 0x000077),
			Make => info!("Make"),
			Markdown => info!("Markdown"),
			Nim => info!("Nim", color: 0xffc200),
			NuShell => info!("NuShell", color: 0x3aa675),
			Oak => info!("Oak"),
			ObjectiveC => info!("Objective-C"),
			ObjectiveCxx => info!("Objective-C++"),
			OCaml => info!("OCaml", color: 0xee6a1a),
			Odin => info!("Odin", color: 0x3882d2),
			Pascal => info!("Pascal"),
			Perl => info!("Perl", color: 0x0073a1),
			Php => info!("PHP", color: 0x4f5d95),
			Porth => info!("Porth"),
			PowerShell => info!("PowerShell"),
			Prolog => info!("Prolog"),
			PureScript => info!("PureScript"),
			Python => info!("Python", color: 0x3776ab),
			Turquoise => info!("Turquoise", color: 0x90eada),
			Racket => info!("Racket"),
			Raku => info!("Raku", color: 0xd0dd2b),
			Reason => info!("Reason", color: 0xdb4d3f),
			Ren => info!("Ren", color: 0xdd5e36),
			ReScript => info!("ReScript", color: 0xD55454),
			Ruby => info!("Ruby", color: 0xcc342d),
			Roc => info!("Roc", color: 0x7c59dd),
			Rust => info!("Rust", color: 0xa72145),
			Sass => info!("Sass", color: 0xcf649a),
			Scala => info!("Scala", color: 0xc6422f),
			Sql => info!("SQL", color: 0x336790),
			Svelte => info!("Svelte", color: 0xe44d26),
			Swift => info!("Swift", color: 0xf05138),
			Tcl => info!("Tcl"),
			Terraform => info!("Terraform", color: 0x844fba),
			Toml => info!("TOML"),
			TypeScript => info!("TypeScript", color: 0x3178c6),
			Unison => info!("Unison", color: [118, 207, 143]),
			V => info!("V"),
			Val => info!("Val", color: [0, 119, 179]),
			Vala => info!("Vala", color: 0x7239b3),
			Vale => info!("Vale"),
			VisualBasic => info!("Visual Basic"),
			Vue => info!("Vue", color: 0x41b883),
			WebAssembly => info!("WebAssembly", color: 0x654ff0),
			Wren => info!("Wren", color: 0x383838),
			Xml => info!("XML"),
			Yall => info!("Y'all", color: 0xff8f77),
			Yaml => info!("YAML"),
			YueScript => info!("YueScript", color: 0xb7ae8f),
			Zig => info!("Zig", color: [235, 168, 66]),
		}
	}
}

#[derive(Clone, Debug)]
pub struct LanguageSummary {
	pub language: Language,
	pub lines: usize,
}

impl Display for LanguageSummary {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		// We have to count this length by hand because, unfortunately, escape codes count
		let left_side_width = self.language.info().name.len() + 4; // circle + 2 leading spaces + 1 trailing space
		let right_side = format!("{}", self.lines);
		let width = f.width().unwrap_or(0) - left_side_width - (right_side.len() + 1);
		let inlay = format!("{:.>width$}", "", width = width)
			.bright_black()
			.to_string();
		write!(f, "{} {} {}", self.language, inlay, right_side)
	}
}

impl LanguageSummary {
	pub fn from(language: Language) -> Self {
		Self { language, lines: 0 }
	}
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Color {
	r: u8,
	g: u8,
	b: u8,
}

impl Color {
	pub fn color<T>(&self, t: T) -> String
	where
		T: Colorize,
	{
		t.truecolor(self.r, self.g, self.b).to_string()
	}

	pub fn on_color<T>(&self, t: T) -> String
	where
		T: Colorize,
	{
		t.on_truecolor(self.r, self.g, self.b).to_string()
	}
}

impl From<[u8; 3]> for Color {
	fn from(color: [u8; 3]) -> Self {
		Self {
			r: color[0],
			g: color[1],
			b: color[2],
		}
	}
}

impl From<u32> for Color {
	fn from(color: u32) -> Self {
		Self {
			r: ((color >> 16) & 0xff) as u8,
			g: ((color >> 8) & 0xff) as u8,
			b: (color & 0xff) as u8,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use Language::*;

	#[test]
	fn language_from_file_name() {
		let check = |inner| Language::from_file_name(OsStr::new(inner));

		assert_eq!(check(""), None);
		assert_eq!(check("CMakeLists.txt"), Some(CMake));
		assert_eq!(check("main.rs"), Some(Rust));
		assert_eq!(check("Makefile"), Some(Make));
		assert_eq!(check("NotCMake.txt"), None);
		assert_eq!(check("README.md"), Some(Markdown));
		assert_eq!(check("main.zig"), Some(Zig));
	}
}
