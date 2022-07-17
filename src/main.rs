#![feature(let_else)]

use std::ffi::OsStr;
use std::fmt;
use std::fmt::Display;
use std::fs::read;
use std::fs::read_dir;
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
struct FileContent {
    pub path: PathBuf,
    language: Option<String>,
    lines: usize,
    // comments: usize,
    // code: usize,
    // blank: usize,
}

impl Display for FileContent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(language) = &self.language {
            write!(
                f,
                "File: {:70}   Language: {:10}   Lines: {:>10}",
                format!("{:?}", self.path.as_os_str()),
                language,
                self.lines,
            )
        } else {
            // write!(f, "File: {:?}", self.path.as_os_str())
            Ok(())
        }
    }
}

impl FileContent {
    pub fn new(path: PathBuf) -> Self {
        let language = path.extension().map(lang_from_extension).flatten();

        Self {
            path,
            language,
            lines: 0,
            // comments: 0,
            // code: 0,
            // blank: 0,
        }
    }
}

fn lang_from_extension(ext: &OsStr) -> Option<String> {
    use colored::Colorize;

    match ext.to_str()? {
        "asm" => Some("Assembly".to_string()),
        "astro" => Some("Astro".to_string()),
        "c" => Some("C".to_string()),
        "cc" => Some("C++".truecolor(25, 65, 122).to_string()),
        "co" => Some("Co".to_string()),
        "coffee" => Some("CoffeeScript".to_string()),
        "cpp" => Some("C++".truecolor(25, 65, 122).to_string()),
        "cr" => Some("Crystal".to_string()),
        "cs" => Some("C#".to_string()),
        "cxx" => Some("C++".truecolor(25, 65, 122).to_string()),
        "d" => Some("D".to_string()),
        "dart" => Some("Dart".to_string()),
        "elm" => Some("Elm".truecolor(0x60, 0xB5, 0xCC).to_string()),
        "ex" => Some("Elixir".truecolor(0x4e, 0x2a, 0x8e).to_string()),
        "fs" => Some("F#".to_string()),
        "f90" => Some("Fortran".to_string()),
        "gleam" => Some("Gleam".truecolor(0xff, 0xaf, 0xf3).to_string()),
        "gn" => Some("gn".to_string()),
        "go" => Some("Go".truecolor(0x00, 0xAD, 0xD8).to_string()),
        "gr" => Some("Grain".truecolor(255, 133, 14).to_string()),
        "gren" => Some("Gren".to_string()),
        "ha" => Some("Hare".to_string()),
        "hs" => Some("Haskell".truecolor(0x61, 0x44, 0xb3).to_string()),
        "idr" => Some("Idris".to_string()),
        "jai" => Some("Jai".to_string()),
        "jakt" => Some("Jakt".to_string()),
        "java" => Some("Java".to_string()),
        "jl" => Some("Julia".to_string()),
        "js" => Some("JavaScript".truecolor(0xf1, 0xe0, 0x5a).to_string()),
        "jsx" => Some("JavaScript".truecolor(0xf1, 0xe0, 0x5a).to_string()),
        "cjs" => Some("JavaScript".truecolor(0xf1, 0xe0, 0x5a).to_string()),
        "mjs" => Some("JavaScript".truecolor(0xf1, 0xe0, 0x5a).to_string()),
        // "kt" => Some("Kotlin".truecolor(0x7f, 0x52, 0xff).to_string()),
        "kt" => Some("Kotlin".truecolor(0xa9, 0x7b, 0xff).to_string()),
        "m" => Some("Objective-C".to_string()),
        "ml" => Some("OCaml".to_string()),
        "nim" => Some("Nim".to_string()),
        "nu" => Some("NuShell".truecolor(0x3a, 0xa6, 0x75).to_string()),
        "odin" => Some("Odin".to_string()),
        "pas" => Some("Pascal".to_string()),
        "php" => Some("PHP".to_string()),
        "pl" => Some("Perl".to_string()),
        "pm" => Some("Perl".to_string()),
        "porth" => Some("Porth".to_string()),
        "py" => Some("Python".to_string()),
        "q" => Some("Turquoise".truecolor(0x90, 0xEA, 0xDA).to_string()),
        "ren" => Some("Ren".to_string()),
        "rb" => Some("Ruby".to_string()),
        "roc" => Some("Roc".to_string()),
        "rs" => Some("Rust".truecolor(0xa7, 0x21, 0x45).to_string()),
        "s" => Some("Assembly".to_string()),
        "scala" => Some("Scala".to_string()),
        "sh" => Some("Bash".to_string()),
        "sql" => Some("SQL".to_string()),
        "svelte" => Some("Svelte".to_string()),
        "swift" => Some("Swift".truecolor(0xF0, 0x51, 0x38).to_string()),
        "toml" => Some("TOML".to_string()),
        "ts" => Some("TypeScript".truecolor(0x31, 0x78, 0xc6).to_string()),
        "tsx" => Some("TypeScript".truecolor(0x31, 0x78, 0xc6).to_string()),
        "cts" => Some("TypeScript".truecolor(0x31, 0x78, 0xc6).to_string()),
        "mts" => Some("TypeScript".truecolor(0x31, 0x78, 0xc6).to_string()),
        "u" => Some("Unison".to_string()),
        "v" => Some("V".to_string()),
        "vale" => Some("Vale".to_string()),
        "vb" => Some("Visual Basic".to_string()),
        "vue" => Some("Vue".to_string()),
        "wat" => Some("WebAssembly".to_string()),
        "zig" => Some("Zig".truecolor(235, 168, 66).to_string()),
        _ => None,
    }
}

fn scan_dir(dir: &str) -> io::Result<()> {
    for entry in read_dir(dir)?.flatten() {
        let path = entry.path();

        if path.file_name() == Some(OsStr::new(".git")) {
            continue;
        }

        if path.file_name() == Some(OsStr::new("build")) {
            continue;
        }

        if path.file_name() == Some(OsStr::new("node_modules")) {
            continue;
        }

        if path.file_name() == Some(OsStr::new("target")) {
            continue;
        }

        if path.file_name() == Some(OsStr::new("vendor")) {
            continue;
        }

        if path.is_file() {
            let text = String::from_utf8(read(&path)?);
            let mut content = FileContent::new(path);

            // meh
            let Ok(text) = text else {
                continue;
            };

            content.lines = text.lines().count();
            println!("{}", content);
        } else if path.is_dir() {
            scan_dir(&path.to_str().unwrap())?;
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    use colored::Colorize;
    // let dir_contents = read_dir(".").unwrap();

    if let Ok(x) = read("./.gitignore").map(|buf| String::from_utf8(buf)) {
        println!("gitignore: {:?}", x);
    }

    scan_dir(".")?;
    println!("\n {}\n", " ".repeat(80).on_truecolor(0xa7, 0x21, 0x45));

    // for each in dir_contents.flatten() {
    //     println!("dir has file {:?}", each);

    //     let type_ = each.file_type()?;
    //     println!("{:?}", type_);
    //     // type_.is_dir()

    //     if type_.is_file() {
    //     }
    // }

    Ok(())
}
