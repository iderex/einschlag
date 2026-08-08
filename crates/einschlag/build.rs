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
        Some(r) => match git(r, &["status", "--porcelain"]) {
            Some(out) if out.is_empty() => "clean",
            Some(_) => "modified",
            None => UNKNOWN,
        },
        None => UNKNOWN,
    };

    println!("cargo::rustc-env=EINSCHLAG_BUILD_COMMIT={commit}");
    println!("cargo::rustc-env=EINSCHLAG_BUILD_TREE_STATE={tree_state}");

    for path in rerun_triggers(root.as_deref()) {
        println!("cargo::rerun-if-changed={}", path.display());
    }
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
    let out = Command::new("git").current_dir(dir).args(args).output().ok()?;
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
/// This is not complete and the incompleteness is in `docs/BUILD.md` and in
/// issue #84: a change elsewhere in the workspace that never reaches the git
/// index leaves the previous marker in place until this crate is rebuilt.
fn rerun_triggers(root: Option<&Path>) -> Vec<PathBuf> {
    let here = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap_or_default());
    let mut paths = vec![here.join("src"), here.join("Cargo.toml"), here.join("build.rs")];

    let Some(root) = root else { return paths };
    let Some(git_dir) = git(root, &["rev-parse", "--absolute-git-dir"]).map(PathBuf::from) else {
        return paths;
    };

    // HEAD moves on a checkout, the ref file moves on a commit, and the index
    // is touched by add, commit and status.
    paths.push(git_dir.join("HEAD"));
    paths.push(git_dir.join("index"));
    if let Some(reference) = git(root, &["symbolic-ref", "--quiet", "HEAD"]) {
        paths.push(git_dir.join(reference));
    }
    paths
}
