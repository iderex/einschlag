//! Derives the commit this build was made from, and whether the tree it was
//! made from differed from that commit.
//!
//! It is derived rather than written down. A commit typed into a file is a
//! statement about the last time somebody remembered to type it, and an output
//! artefact read years later next to a report has to be traceable to the code
//! that produced it rather than to a stale literal.
//!
//! No dependency is used. `git` is invoked as a subprocess, which is the forced
//! means here: nothing else can read a repository this project does not carry a
//! parser for, and the surface is two commands whose output is checked before it
//! is believed.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// What the tool reports when `git` was not available or the source was not a
/// repository. A build from an unpacked source archive is the ordinary case.
const UNKNOWN: &str = "unknown";

fn main() {
    let root = repository_root();

    let commit = root
        .as_deref()
        .and_then(|r| git(r, &["rev-parse", "HEAD"]))
        .filter(|c| is_object_name(c))
        .unwrap_or_else(|| UNKNOWN.to_owned());

    let tree_state = match root.as_deref() {
        // `--no-optional-locks` so that reading the state does not write the
        // index. Without it this script rewrites a path it watches, so a build
        // causes the next build to rerun the script, and the behaviour reads as
        // intermittent rather than as this line missing.
        Some(r) => match git(r, &["--no-optional-locks", "status", "--porcelain"]) {
            Some(out) if out.is_empty() => "clean",
            Some(_) => "modified",
            None => UNKNOWN,
        },
        None => UNKNOWN,
    };

    println!("cargo::rustc-env=EINSCHLAG_BUILD_COMMIT={commit}");
    println!("cargo::rustc-env=EINSCHLAG_BUILD_TREE_STATE={tree_state}");
    println!(
        "cargo::rustc-env=EINSCHLAG_BUILD_RUSTC_VERSION={}",
        compiler_version()
    );

    for path in rerun_triggers(root.as_deref()) {
        println!("cargo::rerun-if-changed={}", path.display());
    }
}

/// The version of the compiler Cargo is actually using for this build.
///
/// Cargo names it in `RUSTC`, so this is the compiler that will build the crate
/// rather than whatever `rustc` a shell would find. That distinction is the
/// whole point: a build reaching Cargo with a toolchain override, or without
/// rustup in front of it, uses a compiler `rust-toolchain.toml` never chose, and
/// nothing else in the tree would notice.
fn compiler_version() -> String {
    let Some(rustc) = std::env::var_os("RUSTC") else {
        return UNKNOWN.to_owned();
    };
    let Ok(out) = Command::new(rustc).arg("--version").output() else {
        return UNKNOWN.to_owned();
    };
    if !out.status.success() {
        return UNKNOWN.to_owned();
    }
    let Ok(text) = String::from_utf8(out.stdout) else {
        return UNKNOWN.to_owned();
    };
    // "rustc 1.2.3 (0000000 1970-01-01)" -> "1.2.3". The example carries no
    // real version on purpose; the pinned one lives in rust-toolchain.toml and
    // nowhere else that configures a build.
    text.split_whitespace()
        .nth(1)
        .filter(|version| is_release_version(version))
        .unwrap_or(UNKNOWN)
        .to_owned()
}

/// Three dot-separated numbers and nothing else. A nightly or beta compiler
/// reports something this refuses, which is correct: this project pins a
/// release and a build made with anything else has not honoured the pin.
fn is_release_version(candidate: &str) -> bool {
    let parts: Vec<&str> = candidate.split('.').collect();
    parts.len() == 3
        && parts
            .iter()
            .all(|part| !part.is_empty() && part.bytes().all(|b| b.is_ascii_digit()))
}

/// The top of the working tree, or `None` when this is not a repository.
fn repository_root() -> Option<PathBuf> {
    let here = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR")?);
    let out = git(&here, &["rev-parse", "--show-toplevel"])?;
    let root = PathBuf::from(out);
    root.is_dir().then_some(root)
}

/// Runs git and returns its trimmed standard output, or `None` if git is
/// missing, fails, or writes something that is not UTF-8.
fn git(dir: &Path, args: &[&str]) -> Option<String> {
    let out = Command::new("git")
        .current_dir(dir)
        .args(args)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let text = String::from_utf8(out.stdout).ok()?;
    Some(text.trim().to_owned())
}

/// A full object name, checked rather than assumed, so that a future git that
/// answers differently produces `unknown` instead of a string that looks like a
/// commit and is not one.
fn is_object_name(s: &str) -> bool {
    s.len() == 40 && s.bytes().all(|b| b.is_ascii_hexdigit())
}

/// What has to change for the two values above to be derived again.
///
/// **Everything the two values are about.** The commit is about `HEAD` and the
/// ref it points at; the tree state is about every path `git status` reads,
/// which is the whole working tree. So the list is every entry at the top of the
/// working tree except the two that are not part of it, plus the three files
/// inside the git directory that move when a commit or a checkout does.
///
/// Emitting any `rerun-if-changed` turns off Cargo's default rule that a change
/// inside the package reruns the script, so a list that named only this package
/// left every other tracked file unwatched. An edit to the front end, a document
/// or a workflow, built with nothing having touched the git index in between,
/// then produced an artefact claiming its tree matched a commit it did not
/// match. Issue #84 measured that and this is the repair.
///
/// A directory in the list is scanned in full, which is Cargo's documented
/// behaviour for a `rerun-if-changed` path that is a directory rather than
/// something observed to work. `docs/BUILD.md` carries what that leaves.
fn rerun_triggers(root: Option<&Path>) -> Vec<PathBuf> {
    let here = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap_or_default());
    let package_inputs = vec![
        here.join("src"),
        here.join("Cargo.toml"),
        here.join("build.rs"),
    ];

    // Not a repository, so there is no tree state to be wrong about and the
    // commit is already `unknown`. Watching this package is what is left.
    let Some(root) = root else {
        return package_inputs;
    };

    let Ok(entries) = fs::read_dir(root) else {
        return package_inputs;
    };
    let mut paths = Vec::new();
    for entry in entries.flatten() {
        let name = entry.file_name();
        // `target` is this script's own output and watching it would make every
        // build rerun the one before it. `.git` is not part of the working tree
        // and changes on almost every git command, and the three files inside it
        // that matter are named below.
        if name == "target" || name == ".git" {
            continue;
        }
        paths.push(entry.path());
    }
    if paths.is_empty() {
        return package_inputs;
    }

    let Some(git_dir) = git(root, &["rev-parse", "--absolute-git-dir"]).map(PathBuf::from) else {
        return paths;
    };

    // HEAD moves on a checkout and the ref file moves on a commit, neither of
    // which changes a byte in the working tree while both change the answer. The
    // index is here for the same reason: staging a file changes what `git
    // status` reports without changing the file.
    paths.push(git_dir.join("HEAD"));
    paths.push(git_dir.join("index"));
    if let Some(reference) = git(root, &["symbolic-ref", "--quiet", "HEAD"]) {
        paths.push(git_dir.join(reference));
    }
    paths
}
