use std::fmt;
use std::fmt::Display;
use std::path::PathBuf;

use crate::langs::Language;

#[derive(Clone, Debug)]
pub struct FileContent {
    pub path: PathBuf,
    pub language: Option<Language>,
    pub lines: usize,
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
        let language = path.extension().map(Language::from_extension).flatten();

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
