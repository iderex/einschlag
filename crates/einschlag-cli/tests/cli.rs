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
    // Not an endorsement. `docs/BUILD.md` and the usage text both say that
    // arguments are read and ignored because no argument grammar has been
    // decided. This test exists so that the day #29 lands one, it fails and
    // has to be rewritten rather than the change going in unnoticed.
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
