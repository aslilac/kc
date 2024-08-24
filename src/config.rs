use std::ffi::OsStr;
use std::path::Component;
use std::sync::LazyLock;

static NODE_MODULES: LazyLock<Component> =
	LazyLock::new(|| Component::Normal(OsStr::new("node_modules")));
static PACKAGE_LOCK_JSON: LazyLock<&OsStr> = LazyLock::new(|| OsStr::new("package-lock.json"));

pub fn default_ignore_rule(path: &ignore::DirEntry) -> bool {
	path.path().components().all(|c| c != *NODE_MODULES) && path.file_name() != *PACKAGE_LOCK_JSON
}
