//! Refuses a direct dependency that `docs/DEPENDENCIES.md` does not account for,
//! and an entry in that document that no manifest asks for.
//!
//! Both directions, because a stale entry is how a file like this stops being
//! read, and a reader who trusts it then defends code nobody wrote a reason for.
//!
//! The manifests are read as text rather than through a TOML library. A library
//! would be the first direct dependency, entered in the document it exists to
//! check, and the forms these manifests use are a handful. The cost of reading
//! text is that a form the reader does not know could be misread as nothing, so
//! the reader refuses what it does not recognise instead of returning an empty
//! answer. Every panic below is that refusal.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

/// Sections whose entries name a crate. A section name ending in
/// `dependencies` that is not one of these is refused rather than skipped.
const DEPENDENCY_SECTIONS: [&str; 4] = [
    "dependencies",
    "dev-dependencies",
    "build-dependencies",
    "workspace.dependencies",
];

/// Manifests this repository authors that Cargo does not reach from the root.
///
/// `fuzz/Cargo.toml` declares its own `[workspace]`, because a libFuzzer target
/// cannot be built by the pinned stable compiler and a member would stop the
/// default suite compiling. The manifest is still one this repository wrote, and
/// until this list existed a dependency added there entered the tree with no
/// entry in `docs/DEPENDENCIES.md` and nothing saying so.
///
/// The list cannot go stale in the other direction either: the test below
/// refuses a manifest on disc that is neither the root, nor a member, nor here.
const MANIFESTS_OUTSIDE_THE_WORKSPACE: [&str; 1] = ["fuzz/Cargo.toml"];

#[test]
fn every_direct_dependency_has_an_entry_and_every_entry_has_a_dependency() {
    let root = workspace_root();
    let used = direct_dependencies(&root);
    let documented = documented_entries(&root);

    let undocumented: Vec<_> = used.difference(&documented).cloned().collect();
    assert!(
        undocumented.is_empty(),
        "a manifest asks for {undocumented:?} and docs/DEPENDENCIES.md has no entry for it. \
         Add one saying what it is for, what doing without it would cost, and its licence."
    );

    let unused: Vec<_> = documented.difference(&used).cloned().collect();
    assert!(
        unused.is_empty(),
        "docs/DEPENDENCIES.md has an entry for {unused:?} and no manifest asks for it. \
         Remove the entry; a stale one is how this file stops being read."
    );
}

#[test]
fn the_number_of_direct_dependencies_is_under_the_stated_ceiling() {
    let root = workspace_root();
    let used = direct_dependencies(&root);
    let ceiling = stated_ceiling(&root);
    assert!(
        used.len() <= ceiling,
        "{} direct dependencies against a ceiling of {ceiling}: {used:?}. \
         The ceiling is a number to argue past, so raise it in docs/DEPENDENCIES.md \
         with the reason, or drop a dependency.",
        used.len()
    );
}

#[test]
fn the_reader_finds_every_manifest_in_the_workspace() {
    let root = workspace_root();
    let declared: BTreeSet<PathBuf> = member_manifests(&root).into_iter().collect();
    let on_disk: BTreeSet<PathBuf> = fs::read_dir(root.join("crates"))
        .expect("the crates directory is readable")
        .map(|entry| entry.expect("the directory entry is readable").path())
        .filter(|path| path.is_dir())
        .map(|path| path.join("Cargo.toml"))
        .filter(|path| path.is_file())
        .collect();
    assert_eq!(
        declared, on_disk,
        "a crate exists that the workspace does not list, or the workspace lists one \
         that is not there. Either way the dependency check would not read it."
    );
}

#[test]
fn every_manifest_in_the_tree_is_one_the_dependency_check_reads() {
    let root = workspace_root();

    let mut read: BTreeSet<PathBuf> = member_manifests(&root).into_iter().collect();
    read.insert(root.join("Cargo.toml"));
    read.extend(outside_manifests(&root));

    let missed: Vec<String> = authored_manifests(&root)
        .difference(&read)
        .map(|path| relative(&root, path))
        .collect();

    assert!(
        missed.is_empty(),
        "the manifests {missed:?} are in this tree and the dependency check does not \
         read them, so a crate asked for there would arrive with no entry in \
         docs/DEPENDENCIES.md. Add each one to MANIFESTS_OUTSIDE_THE_WORKSPACE, or \
         make it a workspace member."
    );
}

/// A path as this repository writes it, from the root and with forward slashes.
fn relative(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .expect("every walked path is under the root")
        .to_string_lossy()
        .replace('\\', "/")
}

/// Every manifest on [`MANIFESTS_OUTSIDE_THE_WORKSPACE`], refusing one that is
/// named and is not there.
fn outside_manifests(root: &Path) -> Vec<PathBuf> {
    MANIFESTS_OUTSIDE_THE_WORKSPACE
        .iter()
        .map(|relative| {
            let manifest = root.join(relative);
            assert!(
                manifest.is_file(),
                "{relative} is named as a manifest outside the workspace and there \
                 is no file there"
            );
            manifest
        })
        .collect()
}

/// Every `Cargo.toml` this repository authors, wherever it sits.
///
/// Build output is skipped by directory name rather than by path, so the build
/// directory of the fuzz crate is skipped by the same rule as the root one.
fn authored_manifests(root: &Path) -> BTreeSet<PathBuf> {
    const SKIPPED: [&str; 2] = [".git", "target"];

    let mut found = BTreeSet::new();
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
                if !SKIPPED.contains(&name.as_str()) {
                    pending.push(path);
                }
            } else if name == "Cargo.toml" {
                found.insert(path);
            }
        }
    }
    found
}

/// The root of the workspace, from this crate's manifest directory.
fn workspace_root() -> PathBuf {
    let here = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let root = here
        .parent()
        .and_then(Path::parent)
        .expect("this crate sits two levels under the workspace root")
        .to_path_buf();
    assert!(
        root.join("Cargo.toml").is_file(),
        "no workspace manifest at {}",
        root.display()
    );
    root
}

/// Every manifest the workspace declares as a member, refusing a members list
/// that names something that is not there.
fn member_manifests(root: &Path) -> Vec<PathBuf> {
    let text = read(&root.join("Cargo.toml"));
    let line = text
        .lines()
        .map(str::trim)
        .find(|line| line.starts_with("members"))
        .unwrap_or_else(|| panic!("the workspace manifest has no members line"));
    let inside = line
        .split_once('[')
        .and_then(|(_, rest)| rest.rsplit_once(']'))
        .map(|(inside, _)| inside)
        .unwrap_or_else(|| {
            panic!("the members line is not a single-line array and this reader only knows that form: {line}")
        });

    inside
        .split(',')
        .map(str::trim)
        .filter(|piece| !piece.is_empty())
        .map(|piece| {
            let member = piece.trim_matches('"');
            assert!(
                !member.contains('"'),
                "a members entry is not a plain quoted string: {piece}"
            );
            let manifest = root.join(member).join("Cargo.toml");
            assert!(
                manifest.is_file(),
                "the workspace lists {member} and there is no manifest at {}",
                manifest.display()
            );
            manifest
        })
        .collect()
}

/// Every crate named in a dependency section of any manifest in the workspace,
/// minus the ones that are parts of this workspace.
fn direct_dependencies(root: &Path) -> BTreeSet<String> {
    let mut manifests = vec![root.join("Cargo.toml")];
    manifests.extend(member_manifests(root));
    manifests.extend(outside_manifests(root));

    let mut found = BTreeSet::new();
    for manifest in &manifests {
        for (name, value) in dependency_lines(manifest) {
            if is_inside_the_workspace(root, manifest, &value) {
                continue;
            }
            found.insert(name);
        }
    }
    found
}

/// The `name = value` pairs under every dependency section of one manifest.
fn dependency_lines(manifest: &Path) -> Vec<(String, String)> {
    let text = read(manifest);
    let mut section = String::new();
    let mut pairs = Vec::new();

    for raw in text.lines() {
        let line = strip_comment(raw).trim().to_owned();
        if line.is_empty() {
            continue;
        }

        if let Some(header) = line
            .strip_prefix('[')
            .and_then(|rest| rest.strip_suffix(']'))
        {
            let header = header.trim();
            assert!(
                !(header.contains("dependencies") && !DEPENDENCY_SECTIONS.contains(&header)),
                "{}: the section [{header}] names dependencies and this reader does not know it. \
                 Teach the reader that form rather than letting it report nothing.",
                manifest.display()
            );
            section = header.to_owned();
            continue;
        }

        if !DEPENDENCY_SECTIONS.contains(&section.as_str()) {
            continue;
        }

        let (name, value) = line.split_once('=').unwrap_or_else(|| {
            panic!(
                "{}: the line {line:?} is inside [{section}] and is not name = value. \
                 This reader refuses what it cannot read rather than counting it as nothing.",
                manifest.display()
            )
        });
        let name = name.trim();
        assert!(
            !name.is_empty() && !name.contains(char::is_whitespace),
            "{}: {name:?} is not a crate name",
            manifest.display()
        );
        pairs.push((name.to_owned(), value.trim().to_owned()));
    }

    pairs
}

/// Whether a dependency value points at another crate in this workspace.
///
/// A path dependency reaching outside the workspace is somebody else's code
/// arriving by a different route, and is counted.
fn is_inside_the_workspace(root: &Path, manifest: &Path, value: &str) -> bool {
    let Some(after) = value.split("path").nth(1) else {
        return false;
    };
    let Some(quoted) = after.split('"').nth(1) else {
        return false;
    };
    let base = manifest.parent().expect("a manifest has a directory");
    let Ok(target) = base.join(quoted).canonicalize() else {
        return false;
    };
    let Ok(root) = root.canonicalize() else {
        return false;
    };
    target.starts_with(root)
}

/// The crate names `docs/DEPENDENCIES.md` has an entry for.
fn documented_entries(root: &Path) -> BTreeSet<String> {
    read(&root.join("docs/DEPENDENCIES.md"))
        .lines()
        .filter_map(|line| line.strip_prefix("### "))
        .map(|name| name.trim().to_owned())
        .collect()
}

/// The ceiling the document states, refusing a document that states none.
fn stated_ceiling(root: &Path) -> usize {
    const OPENING: &str = "The ceiling is ";
    const CLOSING: &str = " direct dependencies.";

    let text = read(&root.join("docs/DEPENDENCIES.md"));
    let line = text
        .lines()
        .map(str::trim)
        .find(|line| line.starts_with(OPENING) && line.ends_with(CLOSING))
        .unwrap_or_else(|| {
            panic!(
                "docs/DEPENDENCIES.md states no ceiling. It needs a line reading \
                 {OPENING}<number>{CLOSING}"
            )
        });

    line[OPENING.len()..line.len() - CLOSING.len()]
        .trim()
        .parse()
        .unwrap_or_else(|_| panic!("the ceiling in {line:?} is not a number"))
}

fn strip_comment(line: &str) -> &str {
    match line.find('#') {
        // A quote before the hash means the hash may be inside a string, and
        // this reader does not try to tell. It leaves the line alone, which
        // sends it to the name = value check rather than silently truncating.
        Some(at) if !line[..at].contains('"') => &line[..at],
        _ => line,
    }
}

fn read(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|why| panic!("cannot read {}: {why}", path.display()))
}
