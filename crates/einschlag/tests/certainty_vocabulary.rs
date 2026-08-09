//! Refuses a build in which any text this project ships or emits carries a
//! phrase courts have been told never to permit.
//!
//! `docs/decisions/0012-certainty-vocabulary.md` argues the scope. In short: the
//! constraint is on what this tool puts in an operator's hands with its name on
//! it, which is every string it can emit and every text file it ships, and it is
//! not on what the operator writes.
//!
//! Four files have to hold the phrases in order to refuse them, and they are
//! named below one at a time. The register fails in both directions, so an
//! exemption cannot be a place to hide a fourth file: a name with no file behind
//! it is dangling, and a file that no longer carries a phrase is stale.
//!
//! This file is not among them. It quotes no phrase, and the fixtures that do
//! are the unit tests inside the module that holds the list.

use std::fs;
use std::path::{Path, PathBuf};

use einschlag::vocabulary::{REFUSED, first_refused};

/// The files allowed to carry a refused phrase, each with the reason.
///
/// Paths are relative to the workspace root and use forward slashes.
const EXEMPT: &[(&str, &str)] = &[
    (
        "crates/einschlag/src/vocabulary.rs",
        "holds the list; it has to carry each literal in order to refuse it",
    ),
    (
        "docs/decisions/0012-certainty-vocabulary.md",
        "quotes the PCAST sentence the list is derived from; a record that names \
         its source without quoting it cannot be checked against that source",
    ),
    (
        "docs/decisions/0010-honesty-rule.md",
        "declares the property this check enforces, and names the phrase in \
         order to say what property 6 is about; records are never edited in place",
    ),
    (
        "docs/survey/challenges.md",
        "the survey the phrases were read out of, quoting what courts were told \
         to refuse; removing the quotation would remove the evidence",
    ),
];

/// Directories never walked. Neither is shipped and neither is emitted.
const SKIPPED_DIRECTORIES: [&str; 2] = [".git", "target"];

#[test]
fn no_shipped_text_carries_a_refused_phrase() {
    let root = workspace_root();
    let mut offences = Vec::new();

    for path in text_files(&root) {
        let relative = relative_to(&root, &path);
        if EXEMPT.iter().any(|(exempt, _)| *exempt == relative) {
            continue;
        }
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        if let Some(phrase) = first_refused(&text) {
            offences.push(format!("{relative} carries {:?}", phrase.text));
        }
    }

    assert!(
        offences.is_empty(),
        "text this project ships carries a phrase courts have been told never to permit:\n  {}\n\
         The list and its provenance are docs/decisions/0012-certainty-vocabulary.md.",
        offences.join("\n  ")
    );
}

#[test]
fn every_exemption_names_a_file_that_is_there() {
    let root = workspace_root();
    for (path, reason) in EXEMPT {
        assert!(
            root.join(path).is_file(),
            "the exemption for {path} names no file in the tree, so it exempts nothing \
             and hides the next file that takes its name. Reason given: {reason}"
        );
    }
}

#[test]
fn every_exemption_is_still_earning_it() {
    let root = workspace_root();
    for (path, reason) in EXEMPT {
        let text = fs::read_to_string(root.join(path))
            .unwrap_or_else(|why| panic!("cannot read the exempt file {path}: {why}"));
        assert!(
            first_refused(&text).is_some(),
            "{path} is exempt and carries no refused phrase, so the exemption is stale. \
             Remove it rather than leaving a file outside the check. Reason given: {reason}"
        );
    }
}

#[test]
fn what_the_tool_emits_today_is_clean() {
    // The scan above reaches these strings as source text. This reaches them as
    // the values the tool actually produces, which is the thing the property is
    // about and which will diverge from the source the moment a string is built
    // rather than written.
    let emitted = einschlag::version_line();
    assert!(
        first_refused(&emitted).is_none(),
        "the version line carries a refused phrase: {emitted}"
    );
}

#[test]
fn the_list_is_not_empty() {
    // A list that emptied would leave every test above passing on nothing.
    assert!(!REFUSED.is_empty(), "the refused vocabulary is empty");
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

/// Every regular file under the root, minus the directories nothing ships.
fn text_files(root: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let mut pending = vec![root.to_path_buf()];

    while let Some(directory) = pending.pop() {
        let entries = fs::read_dir(&directory)
            .unwrap_or_else(|why| panic!("cannot read {}: {why}", directory.display()));
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
            } else {
                found.push(path);
            }
        }
    }
    found
}
