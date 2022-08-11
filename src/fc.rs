use ::std::fs::read_to_string;
use ::std::io;
use ::std::path::PathBuf;

use crate::langs::Language;

#[derive(Clone, Debug)]
pub struct FileContent {
    pub path: PathBuf,
    pub language: Language,
    pub lines: usize,
}

impl FileContent {
    pub fn new(path: PathBuf) -> Result<Self, io::Error> {
        let language = path
            .extension()
            .map(Language::from_extension)
            .flatten()
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Unable to determine language for {:?}", path),
                )
            })?;

        let text = read_to_string(&path)?;
        let lines = text.lines().count();

        Ok(Self {
            path,
            language,
            lines,
        })
    }
}
