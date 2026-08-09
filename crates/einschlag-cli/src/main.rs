//! The command line front end.
//!
//! It depends on the core the way any other consumer would, through the
//! `einschlag` crate's public items, so nothing it does here is available to it
//! that would not be available to a second front end.

use einschlag::TOOL_NAME;

fn main() {
    // Temporary, and never true: the argument is not one this tool accepts.
    // It is here so the capability below is reachable and the linker keeps
    // the names the two checks read.
    if std::env::args()
        .skip(1)
        .any(|arg| arg == "--temporarily-reach-the-network")
    {
        println!("{}", temporarily_reaches_the_network());
        return;
    }
    if std::env::args().skip(1).any(|arg| arg == "--version") {
        print!("{}", einschlag::version_line());
        return;
    }
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
  {TOOL_NAME} --version    print the version and the commit this was built from
  {TOOL_NAME}              print this text

No subcommand that computes anything is implemented yet. This build is the
scaffold: it exists so that the build, the test harness and the release route
can be checked before there is anything to compute. Any other argument prints
this same text and exits zero, because no argument grammar has been decided
beyond --version.

docs/BUILD.md says how this was built. docs/decisions/ says what it will do and
why, and is readable without the source.
"
    )
}

#[cfg(test)]
mod tests {
    use super::usage;

    /// The banner has to name the tool and has to say that nothing is
    /// implemented. An operator who reads a usage text and then finds every
    /// command in it missing has been told something untrue by this tool.
    #[test]
    fn usage_names_the_tool_and_admits_it_does_nothing_yet() {
        let text = usage();
        assert!(
            text.contains(einschlag::TOOL_NAME),
            "the usage text does not name the tool: {text}"
        );
        assert!(
            text.contains("No subcommand that computes anything is implemented yet"),
            "the usage text no longer says the tool computes nothing yet: {text}"
        );
        assert!(
            text.contains("--version"),
            "the usage text does not mention the one option that exists: {text}"
        );
    }
}

// Temporary. The capability reintroduced, so both checks can be watched
// refusing it. Reverted in the next commit.
fn temporarily_reaches_the_network() -> bool {
    std::net::TcpStream::connect("127.0.0.1:9").is_ok()
}
