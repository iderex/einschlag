//! Refuses a build made with a compiler the pin did not choose.
//!
//! `docs/decisions/0002-language-and-toolchain.md` names three mechanisms for
//! three different failures and `rust-toolchain.toml` is the first of them: a
//! clone with rustup in front of it gets the pinned compiler without anybody
//! being told to. The failure it does not cover is a build that reaches Cargo
//! some other way. A distribution-packaged toolchain, an image that installed
//! Cargo directly, or `cargo +1.96.0` typed by hand. On that route an older
//! compiler builds the tree and nothing says so.
//!
//! The second mechanism the record names is `rust-version` in `Cargo.toml`, and
//! #21 requires the version to appear in exactly one tracked file, which a
//! second literal in a manifest is not. This is the third shape #81 offered:
//! keep the one literal, and refuse a disagreement between it and the compiler
//! Cargo actually used.

use std::fs;
use std::path::{Path, PathBuf};

use einschlag::BUILD_RUSTC_VERSION;

#[test]
fn the_compiler_that_built_this_is_the_one_the_pin_names() {
    let pinned = pinned_channel();
    assert_ne!(
        BUILD_RUSTC_VERSION, "unknown",
        "the compiler version could not be derived, so nothing here can judge \
         whether the pin was honoured. This fails rather than passing on an \
         unanswered question."
    );
    assert_eq!(
        BUILD_RUSTC_VERSION, pinned,
        "this build used rustc {BUILD_RUSTC_VERSION} and rust-toolchain.toml pins \
         {pinned}. Either rustup was not in front of this build, or a toolchain \
         override was given. The pin is the authority; a build made with anything \
         else has not honoured it."
    );
}

#[test]
fn the_pin_names_an_exact_release_rather_than_a_channel() {
    let pinned = pinned_channel();
    let parts: Vec<&str> = pinned.split('.').collect();
    let is_release = parts.len() == 3
        && parts
            .iter()
            .all(|part| !part.is_empty() && part.bytes().all(|b| b.is_ascii_digit()));
    assert!(
        is_release,
        "rust-toolchain.toml pins {pinned:?}, which is not an exact release. \
         docs/decisions/0009-determinism.md promises byte-identical output from \
         the same build, and a moving channel is a different build under the same \
         source."
    );
}

#[test]
fn no_other_file_configures_the_toolchain_version() {
    // #21's Done-when, narrowed to what it can be about, and the narrowing was
    // forced by running the wider version first.
    //
    // The wider check refused three files: docs/BUILD.md and
    // docs/decisions/0002-language-and-toolchain.md, both of which quote the
    // number as the output of a command they also show, and this crate's
    // build.rs, whose example line has since been de-versioned. A record is
    // never edited in place, so the wider property would have left the tree
    // permanently red with no legal repair.
    //
    // So the property is that nothing except rust-toolchain.toml CONFIGURES the
    // version: no manifest, no source file, no workflow. A document may quote
    // it, and docs/BUILD.md names the two that do.
    let root = workspace_root();
    let pinned = pinned_channel();
    let mut carrying = Vec::new();

    for path in candidate_files(&root) {
        let relative = relative(&root, &path);
        if relative == "rust-toolchain.toml" {
            continue;
        }
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        if text.contains(&pinned) {
            carrying.push(relative);
        }
    }

    assert!(
        carrying.is_empty(),
        "the pinned version {pinned} is configured in {carrying:?} as well as in \
         rust-toolchain.toml. A second place to set it is a second place to forget \
         it; point at rust-toolchain.toml and give the command that prints it."
    );
}

/// The channel `rust-toolchain.toml` names.
fn pinned_channel() -> String {
    const KEY: &str = "channel";
    let path = workspace_root().join("rust-toolchain.toml");
    let text = fs::read_to_string(&path)
        .unwrap_or_else(|why| panic!("cannot read {}: {why}", path.display()));

    let line = text
        .lines()
        .map(str::trim)
        .find(|line| line.starts_with(KEY))
        .unwrap_or_else(|| panic!("rust-toolchain.toml names no channel"));
    let value = line
        .split_once('=')
        .unwrap_or_else(|| panic!("the channel line is not key = value: {line}"))
        .1
        .trim();
    let unquoted = value.trim_matches('"');
    assert!(
        !unquoted.is_empty() && !unquoted.contains('"'),
        "the channel value is not a plain quoted string: {value}"
    );
    unquoted.to_owned()
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("this crate sits two levels under the workspace root")
        .to_path_buf()
}

fn relative(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .expect("every walked path is under the root")
        .to_string_lossy()
        .replace('\\', "/")
}

/// The files that can configure a build: source, manifests and workflows.
///
/// `docs/` is not among them, and that exclusion is the narrowing argued at the
/// test above rather than a convenience. The git directory, build output and the
/// untracked lock file are skipped because nothing authors them here.
fn candidate_files(root: &Path) -> Vec<PathBuf> {
    const SKIPPED_DIRECTORIES: [&str; 3] = [".git", "target", "docs"];
    const SKIPPED_FILES: [&str; 1] = ["Cargo.lock"];
    const CONFIGURING_SUFFIXES: [&str; 4] = [".rs", ".toml", ".yml", ".yaml"];

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
            } else if !SKIPPED_FILES.contains(&name.as_str())
                && CONFIGURING_SUFFIXES
                    .iter()
                    .any(|suffix| name.ends_with(suffix))
            {
                found.push(path);
            }
        }
    }
    found
}
