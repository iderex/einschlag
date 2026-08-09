//! Refuses a second source of draws anywhere in the workspace.
//!
//! `docs/decisions/0009-determinism.md` promises that the same input, the same
//! seed and the same build give byte-identical output, and it holds that promise
//! by requiring every draw to come from one explicitly seeded generator.
//! `crates/einschlag/src/sampling.rs` is that generator and
//! `docs/decisions/0014-the-sampling-generator.md` argues why the arithmetic for
//! it is written in this repository.
//!
//! The failure this refuses is not somebody rejecting that design. It is one
//! call, written in a hurry, to whatever the language or a crate offers for a
//! number nobody wanted to think about: a jitter, a tie-break, a shuffled order.
//! A run carrying one of those is reproducible in everything except the part
//! that moved, and it fails in the way this project can least afford, because
//! the two outputs both look like reconstructions.
//!
//! **This is a floor and not the property.** It reads names in source, so it
//! holds what somebody would actually write and it will not recognise a route
//! nobody has written yet. It cannot see a crate that arrives with its own
//! source of entropy inside it either; what stands there is the allowed list in
//! `nothing_goes_out.rs`, which refuses any package in the resolved graph that
//! nobody named, and `docs/DEPENDENCIES.md`, which refuses a direct dependency
//! nobody wrote a reason for.

use std::fs;
use std::path::{Path, PathBuf};

/// Names a draw or a source of entropy is reached through.
///
/// Compared as whole tokens and case-insensitively. `rand` is on the list and
/// `random` is a different token, so both are named rather than one being
/// assumed to cover the other.
const SOURCES: &[&str] = &[
    "rand",
    "rands",
    "random",
    "randomize",
    "rng",
    "thread_rng",
    "small_rng",
    "os_rng",
    "getrandom",
    "from_entropy",
    "entropy",
    "randomstate",
    "defaulthasher",
    "randomsource",
];

/// Names a seed would be taken from where nobody supplied one.
///
/// A clock is not a source of draws and it is on a list of its own for that
/// reason. It is here because seeding from one is the ordinary way a run stops
/// being repeatable while every other rule in this file is obeyed: the generator
/// is the project's, the reduction order is fixed, and the sequence still
/// differs on every run.
///
/// `now` is deliberately absent, and its absence is the shape of the whole
/// list. It is the method a clock is read through and it is also an ordinary
/// English word: it appears in a message string in
/// `crates/einschlag-cli/tests/cli.rs` today, and refusing that would be a check
/// firing on a true statement, which is a check somebody switches off. What
/// catches a clock read is the type it goes through, which is on the list.
const IMPLICIT_SEEDS: &[&str] = &["systemtime", "unix_epoch", "instant", "elapsed"];

/// The one file allowed to carry the names, because it is the lists.
const EXEMPT: &str = "crates/einschlag/tests/one_seeded_generator.rs";

const SKIPPED_DIRECTORIES: [&str; 2] = [".git", "target"];

#[test]
fn nothing_in_the_workspace_reaches_for_a_draw_outside_the_seeded_generator() {
    assert_no_name(
        SOURCES,
        "a source of draws that is not the seeded generator",
    );
}

#[test]
fn nothing_in_the_workspace_seeds_itself_from_the_machine() {
    assert_no_name(
        IMPLICIT_SEEDS,
        "a clock, which is how a run seeds itself and stops being repeatable",
    );
}

#[test]
fn the_lists_are_not_empty() {
    // Either list emptying would leave the test above it passing on nothing.
    assert!(!SOURCES.is_empty(), "the source list is empty");
    assert!(
        !IMPLICIT_SEEDS.is_empty(),
        "the implicit seed list is empty"
    );
}

#[test]
fn the_exemption_names_a_file_that_is_there() {
    let root = workspace_root();
    assert!(
        root.join(EXEMPT).is_file(),
        "the exemption names {EXEMPT}, which is not in the tree, so it exempts \
         nothing and hides the next file that takes that name"
    );
}

/// The reader is only worth anything if it is looking at files. A change that
/// moved the sources would otherwise leave this walking an empty tree and
/// reporting nothing, which reads exactly like a clean one.
#[test]
fn the_reader_found_the_sources_and_the_generator() {
    let root = workspace_root();
    let sources = rust_sources(&root);
    assert!(
        sources.len() > 5,
        "the reader found {} Rust files in the workspace, which is fewer than \
         there are",
        sources.len()
    );
    assert!(
        root.join("crates/einschlag/src/sampling.rs").is_file(),
        "the module this check exists to make the only route is not in the tree"
    );
}

fn assert_no_name(names: &[&str], what: &str) {
    let root = workspace_root();
    let mut offences = Vec::new();

    for path in rust_sources(&root) {
        let relative = relative_to(&root, &path);
        if relative == EXEMPT {
            continue;
        }
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        for (number, raw) in text.lines().enumerate() {
            // Comments are stripped rather than read. A doc comment saying what
            // this module refuses is describing it, and a check that fires on
            // the description would stop the source explaining itself.
            let line = strip_line_comment(raw);
            // One offence per line rather than one per name. A single line can
            // carry two of these, and reporting it twice reads as two places to
            // repair. Watched happening: `SystemTime::UNIX_EPOCH` matched both
            // entries of the seed list and printed itself twice.
            if line
                .split(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
                .any(|token| names.contains(&token.to_ascii_lowercase().as_str()))
            {
                offences.push(format!("{relative}:{}: {}", number + 1, raw.trim()));
            }
        }
    }

    assert!(
        offences.is_empty(),
        "the workspace reaches for {what}:\n  {}\n\
         Every draw comes from crates/einschlag/src/sampling.rs, seeded by a value \
         the caller supplied, which is what docs/decisions/0009-determinism.md \
         requires and what makes a run repeatable by somebody who disagrees with it.",
        offences.join("\n  ")
    );
}

fn strip_line_comment(line: &str) -> &str {
    match line.find("//") {
        Some(at) => &line[..at],
        None => line,
    }
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("this crate sits two levels under the workspace root")
        .to_path_buf()
}

fn relative_to(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .expect("every walked path is under the root")
        .to_string_lossy()
        .replace('\\', "/")
}

fn rust_sources(root: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let mut pending = vec![root.to_path_buf()];

    while let Some(directory) = pending.pop() {
        let entries = fs::read_dir(&directory)
            .unwrap_or_else(|why| panic!("cannot read {directory:?}: {why}"));
        for entry in entries {
            let path = entry.expect("the directory entry is readable").path();
            let name = path
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_default();
            if path.is_dir() {
                if !SKIPPED_DIRECTORIES.contains(&name.as_str()) {
                    pending.push(path);
                }
            } else if name.ends_with(".rs") {
                found.push(path);
            }
        }
    }
    found
}
