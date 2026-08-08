//! What the operator actually gets: the built binary, started as a process.
//!
//! A unit test inside the front end can only check a function. These start the
//! artefact, so the manifest, the linkage and the exit status are part of what
//! is being checked.

use std::path::Path;
use std::process::Command;

/// Cargo builds the binary before this test runs and substitutes its path here.
/// The name in the macro is the `[[bin]]` name in `Cargo.toml`, so a rename of
/// the binary stops this file compiling rather than leaving a test that quietly
/// runs something else.
const BIN: &str = env!("CARGO_BIN_EXE_einschlag");

#[test]
fn the_binary_is_named_what_the_core_reports() {
    let stem = Path::new(BIN)
        .file_stem()
        .expect("the binary path has a file name")
        .to_str()
        .expect("the binary path is UTF-8");
    assert_eq!(
        stem,
        einschlag::TOOL_NAME,
        "the built binary and the name the core reports have drifted apart"
    );
}

#[test]
fn no_arguments_prints_usage_and_exits_zero() {
    let run = Command::new(BIN).output().expect("the binary starts");
    assert!(
        run.status.success(),
        "running with no arguments failed: {:?}",
        run.status
    );
    let stdout = String::from_utf8(run.stdout).expect("stdout is UTF-8");
    assert!(
        stdout.contains("usage:"),
        "no usage section in the output: {stdout}"
    );
    assert!(
        stdout.contains(einschlag::TOOL_NAME),
        "the output does not name the tool: {stdout}"
    );
    assert!(
        run.stderr.is_empty(),
        "the run with no arguments wrote to stderr: {:?}",
        String::from_utf8_lossy(&run.stderr)
    );
}

#[test]
fn an_unknown_argument_is_accepted_today_and_this_test_records_that() {
    // Not an endorsement. The usage text says that any argument other than
    // --version prints usage and exits zero, because no argument grammar has
    // been decided beyond that one. This test exists so that the day a grammar
    // lands, it fails and has to be rewritten rather than the change going in
    // unnoticed.
    let run = Command::new(BIN)
        .arg("--no-such-option")
        .output()
        .expect("the binary starts");
    assert!(
        run.status.success(),
        "an unknown argument now fails, which is a change of contract: {:?}",
        run.status
    );
}

#[test]
fn version_reports_the_commit_the_artefact_was_built_from() {
    let run = Command::new(BIN)
        .arg("--version")
        .output()
        .expect("the binary starts");
    assert!(
        run.status.success(),
        "--version failed: {:?}",
        run.status
    );
    let stdout = String::from_utf8(run.stdout).expect("stdout is UTF-8");
    assert!(
        stdout.contains(einschlag::VERSION),
        "--version does not print the version: {stdout}"
    );
    assert!(
        stdout.contains(einschlag::BUILD_COMMIT),
        "--version does not print the commit: {stdout}"
    );
    assert!(
        stdout.contains("commit "),
        "the commit is not labelled in the output: {stdout}"
    );
    assert!(
        run.stderr.is_empty(),
        "--version wrote to stderr: {:?}",
        String::from_utf8_lossy(&run.stderr)
    );
}

#[test]
fn version_says_whether_the_tree_it_was_built_from_was_modified() {
    let run = Command::new(BIN)
        .arg("--version")
        .output()
        .expect("the binary starts");
    let stdout = String::from_utf8(run.stdout).expect("stdout is UTF-8");
    let says_something_about_the_tree = stdout.contains("working tree matched this commit")
        || stdout.contains("working tree had uncommitted changes at build time")
        || stdout.contains("working tree state unknown");
    assert!(
        says_something_about_the_tree,
        "--version reports a commit without saying whether the source matched it: {stdout}"
    );
}
