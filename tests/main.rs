use std::path::Path;
use std::process::Command;

mod setup;

const EXE: &str = "./build/release/kc";

#[test]
fn self_check() {
	setup::before();

	let result = Command::new(EXE).output().unwrap();
	let stdout = String::from_utf8_lossy(&result.stdout);

	assert!(stdout.contains("Rust"));
	assert!(stdout.contains("Markdown"));
	assert!(stdout.contains("TOML"));

	// There's technically a bit of each in the codebase, but we shouldn't report it,
	// because it's all in testdata/ which is ignored by the .ignore file at the repo root.
	assert!(!stdout.contains("Gleam"));
	assert!(!stdout.contains("TypeScript"));
}

#[test]
fn self_check_exclude() {
	setup::before();

	let result = Command::new(EXE).args(["-x", "toml,md"]).output().unwrap();
	let stdout = String::from_utf8_lossy(&result.stdout);

	assert!(stdout.contains("Rust"));
	assert!(!stdout.contains("Markdown"));
	assert!(!stdout.contains("TOML"));
}

#[test]
fn scan_nonexistent() {
	setup::before();

	let nonexistent = "./tests/testdata/nonexistent";
	let nonexistent_path = Path::new(&nonexistent);
	assert!(!nonexistent_path.exists()); // this is kind of a silly check, but why not check anyway

	let result = Command::new(EXE).arg(nonexistent).output().unwrap();
	let stderr = String::from_utf8_lossy(&result.stderr);

	assert!(stderr.contains("is not a directory"));
}

#[test]
fn scan_file() {
	setup::before();

	let result = Command::new(EXE)
		.arg("./tests/testdata/file")
		.output()
		.unwrap();
	let stderr = String::from_utf8_lossy(&result.stderr);

	assert!(stderr.contains("is not a directory"));
}

#[test]
fn scan_empty() {
	setup::before();

	let result = Command::new(EXE)
		.arg("./tests/testdata/empty")
		.output()
		.unwrap();
	let stderr = String::from_utf8_lossy(&result.stderr);

	assert!(stderr.contains("no code found in"));
}

#[test]
fn scan_rust() {
	setup::before();

	let result = Command::new(EXE)
		.arg("./tests/testdata/rust")
		.output()
		.unwrap();
	let stdout = String::from_utf8_lossy(&result.stdout);

	assert!(stdout.contains("Rust"));
	assert!(stdout.contains("6"));
}

#[test]
fn scan_mixed() {
	setup::before();

	let result = Command::new(EXE)
		.arg("./tests/testdata/mixed")
		.output()
		.unwrap();
	let stdout = String::from_utf8_lossy(&result.stdout);
	let mut lines = stdout.lines().skip(1); // skip blank leading line

	let line = lines.next().unwrap();
	assert!(line.contains("Rust"));
	assert!(line.contains("6"));
	let line = lines.next().unwrap();
	assert!(line.contains("Gleam"));
	assert!(line.contains("5"));
	let line = lines.next().unwrap();
	assert!(line.contains("Make"));
	assert!(line.contains("4"));
	let line = lines.next().unwrap();
	assert!(line.contains("TypeScript"));
	assert!(line.contains("2"));
}

#[test]
fn scan_head() {
	setup::before();

	let result = Command::new(EXE)
		.arg("./tests/testdata/mixed")
		.args(["-h", "3"])
		.output()
		.unwrap();
	let stdout = String::from_utf8_lossy(&result.stdout);

	assert!(stdout.contains("Rust"));
	assert!(stdout.contains("Gleam"));
	assert!(stdout.contains("Make"));
	assert!(!stdout.contains("TypeScript"));
}
