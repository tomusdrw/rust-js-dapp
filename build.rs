use std::fmt;
use std::process::Command;

#[cfg(not(windows))]
mod platform {
	use std::process::Command;

	pub static NPM_CMD: &'static str = "npm";
	pub fn handle_fd(cmd: &mut Command) -> &mut Command {
		cmd
	}
}

#[cfg(windows)]
mod platform {
	use std::process::{Command, Stdio};

	pub static NPM_CMD: &'static str = "npm.cmd";
	// NOTE [ToDr] For some reason on windows
	// We cannot have any file descriptors open when running a child process
	// during build phase.
	pub fn handle_fd(cmd: &mut Command) -> &mut Command {
		cmd.stdin(Stdio::null())
			.stdout(Stdio::null())
			.stderr(Stdio::null())
	}
}

fn die<T : fmt::Debug>(s: &'static str, e: T) -> ! {
	panic!("Error: {}: {:?}", s, e);
}

fn main() {
    let path = env!("CARGO_MANIFEST_DIR");
    let dest = "build";

	let child = platform::handle_fd(&mut Command::new(platform::NPM_CMD))
		.arg("install")
		.arg("--no-progress")
		.current_dir(path)
		.status()
		.unwrap_or_else(|e| die("Installing node.js dependencies with npm", e));
	assert!(child.success(), "There was an error installing dependencies.");

	let child = platform::handle_fd(&mut Command::new(platform::NPM_CMD))
		.arg("run")
		.arg("build")
		.env("NODE_ENV", "production")
		.env("BUILD_DEST", dest)
		.current_dir(path)
		.status()
		.unwrap_or_else(|e| die("Building JS code", e));
	assert!(child.success(), "There was an error build JS code.");
}
