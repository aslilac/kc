use std::ffi::OsStr;
use std::fmt;
use std::fmt::Display;

use colored::Colorize;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Language {
    Assembly,
    Astro,
    Bash,
    C,
    Co,
    Cxx,
    CoffeeScript,
    Crystal,
    CSharp,
    D,
    Dart,
    Elm,
    Elixir,
    Erlang,
    Fortran,
    FSharp,
    Gleam,
    Gn,
    Go,
    Grain,
    Gren,
    Hare,
    Haskell,
    Idris,
    Jai,
    Jakt,
    Java,
    JavaScript,
    Julia,
    Kotlin,
    Nim,
    NuShell,
    ObjectiveC,
    OCaml,
    Odin,
    Pascal,
    Php,
    Perl,
    Porth,
    Python,
    Turquoise,
    Ren,
    Ruby,
    Roc,
    Rust,
    Scala,
    Sql,
    Svelte,
    Swift,
    Toml,
    TypeScript,
    Unison,
    V,
    Vale,
    VisualBasic,
    Vue,
    WebAssembly,
    Zig,
}

impl Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let info = LanguageInfo::from(self);

        let name = f
            .width()
            .map(|width| format!("{:width$}", &info.name, width = width))
            .unwrap_or(info.name.clone());

        write!(
            f,
            "{}",
            info.color
                .map(|color| color.color(&*name))
                .unwrap_or(name.to_string())
        )
    }
}

impl Language {
    pub fn from_extension(ext: &OsStr) -> Option<Self> {
        use Language::*;

        match ext.to_str()? {
            "asm" => Some(Assembly),
            "astro" => Some(Astro),
            "c" => Some(C),
            "cc" => Some(Cxx),
            "co" => Some(Co),
            "coffee" => Some(CoffeeScript),
            "cpp" => Some(Cxx),
            "cr" => Some(Crystal),
            "cs" => Some(CSharp),
            "cxx" => Some(Cxx),
            "d" => Some(D),
            "dart" => Some(Dart),
            "elm" => Some(Elm),
            "erl" => Some(Erlang),
            "ex" => Some(Elixir),
            "fs" => Some(FSharp),
            "f90" => Some(Fortran),
            "gleam" => Some(Gleam),
            "gn" => Some(Gn),
            "go" => Some(Go),
            "gr" => Some(Grain),
            "gren" => Some(Gren),
            "ha" => Some(Hare),
            "hs" => Some(Haskell),
            "idr" => Some(Idris),
            "jai" => Some(Jai),
            "jakt" => Some(Jakt),
            "java" => Some(Java),
            "jl" => Some(Julia),
            "js" => Some(JavaScript),
            "jsx" => Some(JavaScript),
            "cjs" => Some(JavaScript),
            "mjs" => Some(JavaScript),
            "kt" => Some(Kotlin),
            "m" => Some(ObjectiveC),
            "ml" => Some(OCaml),
            "nim" => Some(Nim),
            "nu" => Some(NuShell),
            "odin" => Some(Odin),
            "pas" => Some(Pascal),
            "php" => Some(Php),
            "pl" => Some(Perl),
            "pm" => Some(Perl),
            "porth" => Some(Porth),
            "py" => Some(Python),
            "q" => Some(Turquoise),
            "ren" => Some(Ren),
            "rb" => Some(Ruby),
            "roc" => Some(Roc),
            "rs" => Some(Rust),
            "s" => Some(Assembly),
            "scala" => Some(Scala),
            "sh" => Some(Bash),
            "sql" => Some(Sql),
            "svelte" => Some(Svelte),
            "swift" => Some(Swift),
            "toml" => Some(Toml),
            "ts" => Some(TypeScript),
            "tsx" => Some(TypeScript),
            "cts" => Some(TypeScript),
            "mts" => Some(TypeScript),
            "u" => Some(Unison),
            "v" => Some(V),
            "vale" => Some(Vale),
            "vb" => Some(VisualBasic),
            "vue" => Some(Vue),
            "wat" => Some(WebAssembly),
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

impl LanguageInfo {
    pub fn from(lang: &Language) -> Self {
        use Language::*;

        // "c" => Some("C".truecolor(163, 176, 240).to_string()),
        // "c" => Some("C".truecolor(163, 176, 240).to_string()),
        // "java" => Some("Java".truecolor(205, 112, 42).to_string()),
        // "kt" => Some("Kotlin".truecolor(0x7f52ff).to_string()),
        match lang {
            Assembly => LanguageInfo {
                name: "Assembly".to_string(),
                color: None,
            },
            Astro => LanguageInfo {
                name: "Astro".to_string(),
                color: None,
            },
            Bash => LanguageInfo {
                name: "Bash".to_string(),
                color: Some([50, 50, 50].into()),
            },
            C => LanguageInfo {
                name: "C".to_string(),
                color: Some([40, 48, 126].into()),
            },
            Co => LanguageInfo {
                name: "Co".to_string(),
                color: None,
            },
            CoffeeScript => LanguageInfo {
                name: "CoffeeScript".to_string(),
                color: Some(0x3e2723.into()),
            },
            Crystal => LanguageInfo {
                name: "Crystal".to_string(),
                color: None,
            },
            CSharp => LanguageInfo {
                name: "C#".to_string(),
                color: None,
            },
            Cxx => LanguageInfo {
                name: "C++".to_string(),
                color: Some([25, 65, 122].into()),
            },
            D => LanguageInfo {
                name: "D".to_string(),
                color: None,
            },
            Dart => LanguageInfo {
                name: "Dart".to_string(),
                color: None,
            },
            Elm => LanguageInfo {
                name: "Elm".to_string(),
                color: Some(0x60b5cc.into()),
            },
            Elixir => LanguageInfo {
                name: "Elixir".to_string(),
                color: Some(0x4e2a8e.into()),
            },
            Erlang => LanguageInfo {
                name: "Erlang".to_string(),
                color: Some(0xa2003e.into()),
            },
            FSharp => LanguageInfo {
                name: "F#".to_string(),
                color: None,
            },
            Fortran => LanguageInfo {
                name: "Fortran".to_string(),
                color: None,
            },
            Gleam => LanguageInfo {
                name: "Gleam".to_string(),
                color: Some(0xffaff3.into()),
            },
            Gn => LanguageInfo {
                name: "gn".to_string(),
                color: None,
            },
            Go => LanguageInfo {
                name: "Go".to_string(),
                color: Some(0x00add8.into()),
            },
            Grain => LanguageInfo {
                name: "Grain".to_string(),
                color: Some([255, 133, 14].into()),
            },
            Gren => LanguageInfo {
                name: "Gren".to_string(),
                color: Some(0xff6600.into()),
            },
            Hare => LanguageInfo {
                name: "Hare".to_string(),
                color: Some(0x121415.into()),
            },
            Haskell => LanguageInfo {
                name: "Haskell".to_string(),
                color: Some(0x6144b3.into()),
            },
            Idris => LanguageInfo {
                name: "Idris".to_string(),
                color: None,
            },
            Jai => LanguageInfo {
                name: "Jai".to_string(),
                color: None,
            },
            Jakt => LanguageInfo {
                name: "Jakt".to_string(),
                color: Some([255, 0, 0].into()), // TODO: bad
            },
            Java => LanguageInfo {
                name: "Java".to_string(),
                color: Some([205, 55, 47].into()),
            },
            JavaScript => LanguageInfo {
                name: "JavaScript".to_string(),
                color: Some(0xf1e05a.into()),
            },
            Julia => LanguageInfo {
                name: "Julia".to_string(),
                color: None,
            },
            Kotlin => LanguageInfo {
                name: "Kotlin".to_string(),
                color: Some(0xa97bff.into()),
            },
            Nim => LanguageInfo {
                name: "Nim".to_string(),
                color: None,
            },
            NuShell => LanguageInfo {
                name: "NuShell".to_string(),
                color: Some(0x3aa675.into()),
            },
            ObjectiveC => LanguageInfo {
                name: "Objective-C".to_string(),
                color: None,
            },
            OCaml => LanguageInfo {
                name: "OCaml".to_string(),
                color: None,
            },
            Odin => LanguageInfo {
                name: "Odin".to_string(),
                color: None,
            },
            Pascal => LanguageInfo {
                name: "Pascal".to_string(),
                color: None,
            },
            Php => LanguageInfo {
                name: "PHP".to_string(),
                color: None,
            },
            Perl => LanguageInfo {
                name: "Perl".to_string(),
                color: None,
            },
            Porth => LanguageInfo {
                name: "Porth".to_string(),
                color: None,
            },
            Python => LanguageInfo {
                name: "Python".to_string(),
                color: Some(0x3776ab.into()),
            },
            Turquoise => LanguageInfo {
                name: "Turquoise".to_string(),
                color: Some(0x90eada.into()),
            },
            Ren => LanguageInfo {
                name: "Ren".to_string(),
                color: None,
            },
            Ruby => LanguageInfo {
                name: "Ruby".to_string(),
                color: Some(0xcc342d.into()),
            },
            Roc => LanguageInfo {
                name: "Roc".to_string(),
                color: None,
            },
            Rust => LanguageInfo {
                name: "Rust".to_string(),
                color: Some(0xa72145.into()),
            },
            Scala => LanguageInfo {
                name: "Scala".to_string(),
                color: None,
            },
            Sql => LanguageInfo {
                name: "SQL".to_string(),
                color: None,
            },
            Svelte => LanguageInfo {
                name: "Svelte".to_string(),
                color: None,
            },
            Swift => LanguageInfo {
                name: "Swift".to_string(),
                color: Some(0xf05138.into()),
            },
            Toml => LanguageInfo {
                name: "TOML".to_string(),
                color: None,
            },
            TypeScript => LanguageInfo {
                name: "TypeScript".to_string(),
                color: Some(0x3178c6.into()),
            },
            Unison => LanguageInfo {
                name: "Unison".to_string(),
                color: Some([118, 207, 143].into()),
            },
            V => LanguageInfo {
                name: "V".to_string(),
                color: None,
            },
            Vale => LanguageInfo {
                name: "Vale".to_string(),
                color: None,
            },
            VisualBasic => LanguageInfo {
                name: "Visual Basic".to_string(),
                color: None,
            },
            Vue => LanguageInfo {
                name: "Vue".to_string(),
                color: None,
            },
            WebAssembly => LanguageInfo {
                name: "WebAssembly".to_string(),
                color: None,
            },
            Zig => LanguageInfo {
                name: "Zig".to_string(),
                color: Some([235, 168, 66].into()),
            },
        }
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

impl From<usize> for Color {
    fn from(color: usize) -> Self {
        Self {
            r: ((color >> 16) & 0xff) as u8,
            g: ((color >> 8) & 0xff) as u8,
            b: (color & 0xff) as u8,
        }
    }
}
