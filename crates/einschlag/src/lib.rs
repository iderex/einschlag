//! The geometry and uncertainty core.
//!
//! Everything this project computes belongs here, and the front end reaches it
//! only through the items this file marks `pub`. That boundary is the compiler's
//! rather than a convention: a front end that reads the core's internals is a
//! front end the core cannot be replaced underneath, and milestones 4 to 7
//! assume it can.
//!
//! The core is empty today. What fills it is decided in `docs/decisions/` and
//! carried by milestones 4 and 5.

pub mod math;
pub mod sampling;
pub mod vocabulary;

/// The name the tool reports itself as.
///
/// It lives here rather than in the front end so that a second front end, or a
/// script driving the tool, reports the same name as the first one.
pub const TOOL_NAME: &str = "einschlag";

/// The released version of the tool, read from the one manifest that states it.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The commit this binary was built from, or `unknown` where the build was made
/// somewhere `git` could not answer, such as an unpacked source archive.
///
/// Derived by `build.rs` at build time rather than written into a file, so it
/// cannot be a statement about the last time somebody remembered to update it.
pub const BUILD_COMMIT: &str = env!("EINSCHLAG_BUILD_COMMIT");

/// `clean` if the working tree matched [`BUILD_COMMIT`] when the build was made,
/// `modified` if it did not, `unknown` where `git` could not answer.
///
/// A build from a modified tree says so here rather than reporting the commit as
/// though the source matched it.
pub const BUILD_TREE_STATE: &str = env!("EINSCHLAG_BUILD_TREE_STATE");

/// The release version of the compiler Cargo used for this build, or `unknown`.
///
/// It is the compiler Cargo named rather than whatever a shell would find, and
/// `crates/einschlag/tests/toolchain_pin.rs` refuses a build where it disagrees
/// with the version pinned in `rust-toolchain.toml`.
pub const BUILD_RUSTC_VERSION: &str = env!("EINSCHLAG_BUILD_RUSTC_VERSION");

/// What the tool says about its own provenance, on one line per fact.
///
/// An output artefact may be read years later, next to a report, by somebody
/// deciding whether to trust it, and the first question is which code produced
/// it. This is the answer to that question and nothing else.
pub fn version_line() -> String {
    let tree = match BUILD_TREE_STATE {
        "clean" => "working tree matched this commit".to_owned(),
        "modified" => "working tree had uncommitted changes at build time".to_owned(),
        other => format!("working tree state {other}"),
    };
    format!("{TOOL_NAME} {VERSION}\ncommit {BUILD_COMMIT}, {tree}\n")
}

#[cfg(test)]
mod tests {
    /// Temporary. An assertion that is false, so the `test` check can be watched
    /// going red on its own while `build`, `fmt` and `lint` stay green. This
    /// compiles cleanly and is formatted, so nothing but the run itself fails.
    #[test]
    fn temporarily_failing() {
        assert_eq!(
            1 + 1,
            3,
            "a temporary assertion, here to be watched failing"
        );
    }

    use super::{BUILD_COMMIT, BUILD_TREE_STATE, TOOL_NAME, VERSION, version_line};

    /// The name is written into a usage banner and typed at a shell. A name
    /// carrying whitespace or a byte outside ASCII would have to be quoted to be
    /// typed, and `docs/decisions/0007-input-format.md` puts this project's
    /// text where a person can check it line by line.
    #[test]
    fn tool_name_can_be_typed_at_a_shell() {
        assert!(!TOOL_NAME.is_empty(), "the tool has no name");
        assert!(
            !TOOL_NAME.chars().any(char::is_whitespace),
            "the name carries whitespace and would need quoting: {TOOL_NAME:?}"
        );
        assert!(TOOL_NAME.is_ascii(), "the name is not ASCII: {TOOL_NAME:?}");
    }

    /// The failure this refuses is a build configuration that drops the field.
    /// `env!` would then expand to an empty string and every artefact would
    /// carry a commit line with nothing in it, which reads as a commit nobody
    /// recorded rather than as a build that lost its provenance.
    #[test]
    fn the_commit_field_is_an_object_name_or_says_it_is_unknown() {
        assert!(!BUILD_COMMIT.is_empty(), "the commit field is empty");
        let is_object_name =
            BUILD_COMMIT.len() == 40 && BUILD_COMMIT.bytes().all(|b| b.is_ascii_hexdigit());
        assert!(
            is_object_name || BUILD_COMMIT == "unknown",
            "the commit field is neither an object name nor unknown: {BUILD_COMMIT:?}"
        );
    }

    #[test]
    fn the_tree_state_is_one_of_the_three_words_that_mean_something() {
        assert!(
            matches!(BUILD_TREE_STATE, "clean" | "modified" | "unknown"),
            "the tree state is not a word this project defined: {BUILD_TREE_STATE:?}"
        );
    }

    #[test]
    fn the_version_line_is_not_empty_and_carries_the_commit() {
        let line = version_line();
        assert!(!line.trim().is_empty(), "the version line is empty");
        // Checked before the `contains` below, which an empty commit would
        // satisfy vacuously. Watched happening: with the field dropped, this
        // test passed and only the object-name test went red.
        assert!(
            !BUILD_COMMIT.is_empty(),
            "the commit field is empty, so the check below would pass on nothing"
        );
        assert!(
            line.contains(VERSION),
            "the version line does not carry the version: {line:?}"
        );
        assert!(
            line.contains(BUILD_COMMIT),
            "the version line does not carry the commit: {line:?}"
        );
        assert!(
            line.contains("commit "),
            "the commit is not labelled, so a reader cannot tell what the hex is: {line:?}"
        );
    }
}
