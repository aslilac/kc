use std::process::Command;
use std::sync::Once;

static BUILD: Once = Once::new();

pub fn before() {
	BUILD.call_once(|| {
		Command::new("cargo")
			.args(&["build", "--release"])
			.status()
			.expect("failed to build test binary");
	});
}
