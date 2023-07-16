use std::ffi::OsStr;

pub fn default_ignore_rule(path: &ignore::DirEntry) -> bool {
	path.file_name() != OsStr::new("package-lock.json")
}
