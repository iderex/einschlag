//! The command line front end.
//!
//! It depends on the core the way any other consumer would, through the
//! `einschlag` crate's public items, so nothing it does here is available to it
//! that would not be available to a second front end.

use einschlag::TOOL_NAME;

fn main() {
    print!("{}", usage());
}

/// The text printed when the tool is run.
///
/// It says what the tool does not do yet rather than listing commands that do
/// not exist. An operator who runs this and reads a list of subcommands would
/// then find every one of them missing.
fn usage() -> String {
    format!(
        "{TOOL_NAME}: shooting-scene reconstruction that states what it cannot exclude.

usage:
  {TOOL_NAME}

No subcommand is implemented yet. This build is the scaffold: it exists so that
the build, the test harness and the release route can be checked before there is
anything to compute. Running it with arguments prints this same text and exits
zero, because no argument grammar has been decided; issue #29 lands the first
one, with the version and the commit it was built from.

docs/BUILD.md says how this was built. docs/decisions/ says what it will do and
why, and is readable without the source.
"
    )
}
