//! Refuses a suite that has acquired a requirement for a display or for
//! elevated rights.
//!
//! Two of this project's stated users work on borrowed or restricted machines,
//! and `../../../docs/survey/field-practice.md` is where that is recorded. A
//! test that opens a window or asks for administrator rights excludes them, and
//! it excludes an unattended runner too, which means the suite that is reported
//! green is not the suite that ran.
//!
//! **What this can and cannot do is in `docs/TESTING.md` at the rule.** In
//! short: Rust has no declaration of what a test requires, so nothing here reads
//! one. What it reads is the source, for the names a display or an elevation
//! request is made through, and it is a floor rather than a guarantee. It holds
//! what somebody would actually write and it will not catch a route nobody has
//! written yet.

use std::fs;
use std::path::{Path, PathBuf};

/// Names a test would go through to reach a display server or a window system.
///
/// Compared as whole tokens and case-insensitively, so `sqrt` does not match
/// `qt` and `DISPLAY` matches `display`.
const DISPLAY_MARKERS: &[&str] = &[
    "display",
    "wayland_display",
    "x11",
    "xlib",
    "xcb",
    "winit",
    "gtk",
    "qt",
    "egui",
    "iced",
    "sdl2",
    "minifb",
    "user32",
    "createwindowexw",
    "messageboxw",
];

/// Names a test would go through to ask for rights it should not need.
const ELEVATION_MARKERS: &[&str] = &[
    "runas",
    "sudo",
    "setuid",
    "seteuid",
    "shellexecutew",
    "adjusttokenprivileges",
    "requireadministrator",
    "netsh",
    "sc.exe",
];

/// The one file allowed to carry the markers, because it is the list.
const EXEMPT: &str = "crates/einschlag/tests/headless_and_unprivileged.rs";

const SKIPPED_DIRECTORIES: [&str; 2] = [".git", "target"];

#[test]
fn no_source_in_the_workspace_reaches_for_a_display() {
    assert_no_marker(DISPLAY_MARKERS, "a display or a window system");
}

#[test]
fn no_source_in_the_workspace_asks_for_elevated_rights() {
    assert_no_marker(ELEVATION_MARKERS, "elevated rights");
}

#[test]
fn the_marker_lists_are_not_empty() {
    // Either list emptying would leave both tests above passing on nothing.
    assert!(
        !DISPLAY_MARKERS.is_empty(),
        "the display marker list is empty"
    );
    assert!(
        !ELEVATION_MARKERS.is_empty(),
        "the elevation marker list is empty"
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

fn assert_no_marker(markers: &[&str], what: &str) {
    let root = workspace_root();
    let mut offences = Vec::new();

    for path in source_files(&root) {
        let relative = relative_to(&root, &path);
        if relative == EXEMPT {
            continue;
        }
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        for (number, token) in markers_in(&text, markers) {
            offences.push(format!("{relative}:{number} names {token:?}"));
        }
    }

    assert!(
        offences.is_empty(),
        "the source reaches for {what}, which the default suite may not require:\n  {}\n\
         docs/TESTING.md states the rule and what this check does not cover.",
        offences.join("\n  ")
    );
}

/// Every marker in `text`, as the line it is on counting from one and the
/// token that matched.
///
/// Tokens are compared whole and case-insensitively, and a `.` is part of a
/// token rather than a separator, so `path.display()` is the token
/// `path.display` and matches nothing. That is why an ordinary call on a path
/// is not an offence here.
///
/// **One name is read as a form rather than as a token**, and it is the only
/// one: `display`. The standard library's formatting trait lowercases to it, so
/// every type in this workspace that wanted a readable form had to write a
/// plain method instead, and two of them did. A check that fires on true
/// statements is a check somebody switches off, which
/// `crates/einschlag/src/vocabulary.rs` argues at the place it chooses phrases
/// over bare words. Issue #111 is where that was fixed.
///
/// **What that costs, stated rather than hidden.** The exemption is the five
/// characters `fmt::` in front of the name, so `impl std::fmt::Display for X`
/// passes and a bare `Display` after a `use` does not. Writing the trait out in
/// full is the price of the check staying able to read the difference: a bare
/// `Display` and a window-system name are the same token, and nothing here can
/// tell them apart. The narrow exemption is deliberate and so is what it
/// refuses.
fn markers_in(text: &str, markers: &[&str]) -> Vec<(usize, String)> {
    let mut found = Vec::new();
    for (index, line) in text.lines().enumerate() {
        for (at, raw) in tokens(line) {
            let token = raw.trim_matches('.').to_ascii_lowercase();
            if !markers.contains(&token.as_str()) {
                continue;
            }
            if token == "display" && line[..at].ends_with("fmt::") {
                continue;
            }
            found.push((index + 1, token));
        }
    }
    found
}

/// The tokens of one line, each with the byte it starts at, so that what is in
/// front of a token can be read.
fn tokens(line: &str) -> Vec<(usize, &str)> {
    let is_token_byte = |c: char| c.is_ascii_alphanumeric() || c == '_' || c == '.';
    let mut found = Vec::new();
    let mut start = None;
    for (at, character) in line.char_indices() {
        match (is_token_byte(character), start) {
            (true, None) => start = Some(at),
            (false, Some(from)) => {
                found.push((from, &line[from..at]));
                start = None;
            }
            _ => {}
        }
    }
    if let Some(from) = start {
        found.push((from, &line[from..]));
    }
    found
}

/// The formatting trait is not a window system, and the check reads the
/// difference.
///
/// The direction that stops this being a hole: the same name without `fmt::`
/// in front of it is still an offence. A rule that only stopped refusing would
/// have stopped checking, and the two cannot be told apart by watching one of
/// them.
#[test]
fn the_standard_formatting_trait_is_not_read_as_a_window_system() {
    let allowed = "impl std::fmt::Display for Refusal {\n    fn fmt(&self) -> Result {}\n}\n";
    assert_eq!(
        markers_in(allowed, DISPLAY_MARKERS),
        Vec::new(),
        "an implementation of the standard formatting trait is refused"
    );

    let refused = "let display = open_the_display();\n";
    assert_eq!(
        markers_in(refused, DISPLAY_MARKERS),
        vec![(1, "display".to_owned())],
        "a bare reach for a display is not refused"
    );
}

/// The reach for a window system that is not spelled `display` is unmoved by
/// the exemption above, which reaches one name and no others.
#[test]
fn the_other_window_system_names_are_untouched_by_the_exemption() {
    let refused = "use fmt::x11 as w;\nlet w = winit::Window::new();\n";
    assert_eq!(
        markers_in(refused, DISPLAY_MARKERS),
        vec![(1, "x11".to_owned()), (2, "winit".to_owned())],
        "the exemption reached a name it was not written for"
    );
}

/// An ordinary call on a path is not a reach for a window system, and this is
/// the property the tokeniser has always had rather than a new one. It is
/// asserted here because the tokeniser was rewritten under it.
#[test]
fn a_path_being_printed_is_not_a_reach_for_a_window_system() {
    let ordinary = "panic!(\"cannot read {}: {why}\", directory.display());\n";
    assert_eq!(markers_in(ordinary, DISPLAY_MARKERS), Vec::new());
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

/// Rust source and manifests. What the suite is made of and what it pulls in.
///
/// Documentation is deliberately not read. A document naming a display variable
/// is describing something rather than requiring it, and refusing that would
/// make `docs/TESTING.md` unable to state its own rule.
fn source_files(root: &Path) -> Vec<PathBuf> {
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
            } else if name.ends_with(".rs") || name == "Cargo.toml" {
                found.push(path);
            }
        }
    }
    found
}
