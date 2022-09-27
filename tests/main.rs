use std::path::Path;
use std::process::Command;
use std::sync::Once;

const EXE: &str = "./build/release/kc";

static BUILD: Once = Once::new();

fn setup() {
	BUILD.call_once(|| {
		Command::new("cargo")
			.args(&["build", "--release"])
			.status()
			.expect("failed to build test binary");
	});
}

#[test]
fn self_check() {
	setup();

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
	setup();

	let result = Command::new(EXE).args(["-x", "toml,md"]).output().unwrap();
	let stdout = String::from_utf8_lossy(&result.stdout);

	assert!(stdout.contains("Rust"));
	assert!(!stdout.contains("Markdown"));
	assert!(!stdout.contains("TOML"));
}

#[test]
fn scan_nonexistent() {
	setup();

	let nonexistent = "./tests/testdata/nonexistent";
	let nonexistent_path = Path::new(&nonexistent);
	assert!(!nonexistent_path.exists()); // this is kind of a silly check, but why not check anyway

	let result = Command::new(EXE).arg(nonexistent).output().unwrap();
	let stderr = String::from_utf8_lossy(&result.stderr);

	assert!(stderr.contains("is not a directory"));
}

#[test]
fn scan_file() {
	setup();

	let result = Command::new(EXE)
		.arg("./tests/testdata/file")
		.output()
		.unwrap();
	let stderr = String::from_utf8_lossy(&result.stderr);

	assert!(stderr.contains("is not a directory"));
}

#[test]
fn scan_empty() {
	setup();

	let result = Command::new(EXE)
		.arg("./tests/testdata/empty")
		.output()
		.unwrap();
	let stderr = String::from_utf8_lossy(&result.stderr);

	assert!(stderr.contains("no code found in"));
}

#[test]
fn scan_rust() {
	setup();

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
	setup();

	let result = Command::new(EXE)
		.arg("./tests/testdata/mixed")
		.output()
		.unwrap();
	let stdout = String::from_utf8_lossy(&result.stdout);

	assert!(stdout.contains("Rust"));
	assert!(stdout.contains("6"));
	assert!(stdout.contains("Gleam"));
	assert!(stdout.contains("5"));
	assert!(stdout.contains("Make"));
	assert!(stdout.contains("4"));
	assert!(stdout.contains("TypeScript"));
	assert!(stdout.contains("2"));
}
