use std::path::Path;
use std::process::Command;

mod setup;

const EXE: &str = "./build/release/kc";

#[test]
fn self_check() {
	setup::before();

	let result = Command::new(EXE).output().unwrap();
	assert!(result.status.success());
	let stdout = String::from_utf8_lossy(&result.stdout);

	assert!(stdout.contains("Rust"));
	assert!(stdout.contains("Markdown"));
	assert!(stdout.contains("TOML"));

	// There's technically a bit of each in the codebase, but we shouldn't report it,
	// because it's all in testdata/ which is ignored by the .ignore file at the repo root.
	assert!(!stdout.contains("Gleam"));
	assert!(!stdout.contains("TypeScript"));
	// There's Yaml in the codebase as well, but only in .github/, which is hidden
	assert!(!stdout.contains("YAML"));

	let result = Command::new(EXE).arg("-a").output().unwrap();
	assert!(result.status.success());
	let stdout = String::from_utf8_lossy(&result.stdout);
	// We're including hidden stuff now!
	assert!(stdout.contains("YAML"));
	// ...but not ignored stuff
	assert!(!stdout.contains("Gleam"));

	let result = Command::new(EXE).arg("-A").output().unwrap();
	assert!(result.status.success());
	let stdout = String::from_utf8_lossy(&result.stdout);
	// We're including ignored stuff now!
	assert!(stdout.contains("Gleam"));
	// ...but not hidden stuff
	assert!(!stdout.contains("YAML"));

	let result = Command::new(EXE).arg("-aa").output().unwrap();
	assert!(result.status.success());
	let stdout = String::from_utf8_lossy(&result.stdout);
	// We're including ignored and hidden stuff now!
	assert!(stdout.contains("Gleam"));
	assert!(stdout.contains("YAML"));
}

#[test]
fn self_check_exclude() {
	setup::before();

	let result = Command::new(EXE).args(["-x", "toml,md"]).output().unwrap();
	assert!(result.status.success());
	let stdout = String::from_utf8_lossy(&result.stdout);

	assert!(stdout.contains("Rust"));
	assert!(!stdout.contains("Markdown"));
	assert!(!stdout.contains("TOML"));
}

#[test]
fn self_check_only() {
	setup::before();

	let result = Command::new(EXE).args(["-o", "rs"]).output().unwrap();
	assert!(result.status.success());
	let stdout = String::from_utf8_lossy(&result.stdout);

	assert!(stdout.contains("Rust"));
	assert!(!stdout.contains("Markdown"));
	assert!(!stdout.contains("TOML"));
}

#[test]
fn self_check_lines() {
	setup::before();

	let result = Command::new(EXE).args(["-l"]).output().unwrap();
	assert!(result.status.success());
	let stdout = String::from_utf8_lossy(&result.stdout);

	assert!(
		stdout
			.trim()
			.parse::<isize>()
			.expect("program output should only be an integer")
			> 1000,
		"output: {}",
		stdout
	);
}

#[test]
fn self_check_blame() {
	setup::before();

	let result = Command::new(EXE).args(["-blame"]).output().unwrap();
	assert!(result.status.success());
	let stdout = String::from_utf8_lossy(&result.stdout);

	assert!(stdout.contains("Rust"));
	assert!(stdout.contains("./src/main.rs"));
}

#[test]
fn scan_nonexistent() {
	setup::before();

	let nonexistent = "./tests/testdata/nonexistent";
	let nonexistent_path = Path::new(&nonexistent);
	assert!(!nonexistent_path.exists()); // this is kind of a silly check, but why not check anyway

	let result = Command::new(EXE).arg(nonexistent).output().unwrap();
	assert!(!result.status.success());
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
	assert!(!result.status.success());
	let stderr = String::from_utf8_lossy(&result.stderr);

	assert!(stderr.contains("is not a directory"));
}

#[test]
fn scan_empty() {
	setup::before();

	let result = Command::new(EXE)
		.arg("tests/testdata/empty/")
		.output()
		.unwrap();
	assert!(result.status.success());
	let stderr = String::from_utf8_lossy(&result.stderr);

	assert!(stderr.contains("no code found in"));
}

#[test]
fn scan_rust() {
	setup::before();

	let result = Command::new(EXE)
		.arg("tests/testdata/rust/")
		.output()
		.unwrap();
	assert!(result.status.success());
	let stdout = String::from_utf8_lossy(&result.stdout);

	assert!(stdout.contains("Rust"));
	assert!(stdout.contains("6"));
}

#[test]
fn scan_mixed() {
	setup::before();

	let result = Command::new(EXE)
		.arg("tests/testdata/mixed")
		.output()
		.unwrap();
	assert!(result.status.success());
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

	let result = Command::new(EXE)
		.args(["-l", "tests/testdata/mixed/"])
		.output()
		.unwrap();
	assert!(result.status.success());
	let stdout = String::from_utf8_lossy(&result.stdout);
	assert_eq!(stdout, "17\n");

	let result = Command::new(EXE)
		.args(["-l", "tests/testdata/mixed/", "-t", "2"])
		.output()
		.unwrap();
	assert!(result.status.success());
	let stdout = String::from_utf8_lossy(&result.stdout);
	assert_eq!(stdout, "11\n");

	let result = Command::new(EXE)
		.args(["-l", "tests/testdata/mixed/", "-x", "rust,gleam"])
		.output()
		.unwrap();
	assert!(result.status.success());
	let stdout = String::from_utf8_lossy(&result.stdout);
	assert_eq!(stdout, "6\n");
}

// Update snapshot by running `cargo run -- -reporter html tests/testdata/mixed/ > tests/testdata/mixed.html`
#[test]
fn scan_mixed_with_html_reporter() {
	setup::before();

	let result = Command::new(EXE)
		.args(["-reporter", "html", "tests/testdata/mixed/"])
		.output()
		.unwrap();
	assert!(result.status.success());
	let stdout = String::from_utf8_lossy(&result.stdout);

	assert_eq!(include_str!("./testdata/mixed.html"), stdout);
}

#[test]
fn scan_hidden() {
	setup::before();

	// Scan without including hidden files should fail
	let result = Command::new(EXE)
		.arg("tests/testdata/hidden")
		.output()
		.unwrap();
	assert!(result.status.success());
	let stderr = String::from_utf8_lossy(&result.stderr);
	assert!(stderr.contains("no code found"));

	// Scan *with* hidden files included should report the hidden files
	let result = Command::new(EXE)
		.arg("-a")
		.arg("tests/testdata/hidden")
		.output()
		.unwrap();
	assert!(result.status.success());
	let stdout = String::from_utf8_lossy(&result.stdout);
	let mut lines = stdout.lines().skip(1); // skip blank leading line;

	// This test is less than ideal. The order in which these appear isn't stable, so we
	// just check that both lines include either one. Technically they could both say JSON,
	// and that would be a bug, but we wouldn't catch it.
	let line = lines.next().unwrap();
	assert!(line.contains("TOML") || line.contains("JSON"));
	assert!(line.contains("0"));
	let line = lines.next().unwrap();
	assert!(line.contains("TOML") || line.contains("JSON"));
	assert!(line.contains("0"));
}

#[test]
fn respects_ignore_file() {
	setup::before();

	let result = Command::new(EXE)
		.arg("tests/testdata/ignore/")
		.output()
		.unwrap();
	assert!(result.status.success());
	let stdout = String::from_utf8_lossy(&result.stdout);
	assert!(stdout.contains("Kotlin"));
	assert!(!stdout.contains("Go"));
	assert!(!stdout.contains("Rust"));
}

#[test]
fn ignores_ignore_file() {
	setup::before();

	let result = Command::new(EXE)
		.args(["tests/testdata/ignore/", "-A"])
		.output()
		.unwrap();
	assert!(result.status.success());
	let stdout = String::from_utf8_lossy(&result.stdout);
	assert!(stdout.contains("Kotlin"));
	assert!(stdout.contains("Go"));
	assert!(stdout.contains("Rust"));
}

#[test]
fn respects_gitignore_file() {
	setup::before();

	let result = Command::new(EXE)
		.arg("tests/testdata/gitignore/")
		.output()
		.unwrap();
	assert!(result.status.success());
	let stdout = String::from_utf8_lossy(&result.stdout);
	assert!(stdout.contains("Kotlin"));
	assert!(!stdout.contains("Go"));
	assert!(!stdout.contains("Rust"));
}

#[test]
fn ignores_gitignore_file() {
	setup::before();

	let result = Command::new(EXE)
		.args(["tests/testdata/gitignore/", "-A"])
		.output()
		.unwrap();
	assert!(result.status.success());
	let stdout = String::from_utf8_lossy(&result.stdout);
	assert!(stdout.contains("Kotlin"));
	assert!(stdout.contains("Go"));
	assert!(stdout.contains("Rust"));
}
