//! Refuses a build in which anything able to open a network connection has
//! arrived in the workspace.
//!
//! `docs/PRIVACY.md` states that the tool sends nothing, and that the absence is
//! by design rather than a feature not yet written. It is the claim an adversary
//! would most want to be false, for an audience that may be working on a machine
//! that will be searched, and until this file existed it was held by the document
//! and by whoever read the source.
//!
//! The judgement is made against the resolved dependency graph rather than
//! against the source, which is what #63 asked for. Reading the source catches a
//! call somebody wrote here and misses a transitive crate that brought a network
//! stack in behind a dependency nobody looked past. The graph catches both,
//! because a socket in a crate five levels down is still a package in the graph.
//!
//! **This is a floor and not the property.** What it establishes is that no
//! package outside a declared list is in the build. It does not establish that
//! the packages on that list open no socket, and nothing here reads a syscall, a
//! symbol table or a running process. `docs/PRIVACY.md` says which of those is
//! checked at the place it makes the claim, and #96 holds the stronger
//! mechanism.
//!
//! The graph is read from `Cargo.lock` as text. Cargo writes that file before it
//! builds anything, so it exists whenever this test runs, and reading it needs no
//! crate to parse it and no subprocess. `Cargo.lock` is untracked until #26,
//! which is why this reads the resolved file rather than a tracked one, and why
//! the check refuses a file it cannot read rather than treating one as absent.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

/// Every package allowed in the resolved graph, with the reason it is there.
///
/// The list is the mechanism. Anything not on it fails, whether it arrived as a
/// direct dependency or five levels down behind one, so a network stack cannot
/// enter without somebody adding its name here and reading this comment.
const ALLOWED: &[(&str, &str)] = &[
    ("einschlag", "this workspace's own core"),
    ("einschlag-cli", "this workspace's own front end"),
    (
        "einschlag-hardware-harness",
        "this workspace's own runs that need equipment, kept out of the default \
         test suite by issue #53. It depends on the core and on nothing else",
    ),
    (
        "libm",
        "the pinned transcendental implementation the numeric core calls, \
         argued in docs/decisions/0013-platform-math-out-of-the-numeric-core.md. \
         It computes floating-point functions and has no dependencies of its own",
    ),
];

/// Name components that belong to something able to open a socket or negotiate
/// a connection over one.
///
/// This is the second layer and it exists for one case: a package added to the
/// list above without the reason being thought about. It is a floor of the
/// weakest kind, since it holds names rather than behaviour and it will not
/// recognise a crate nobody has heard of. Each entry is matched against the whole
/// package name and against its hyphen or underscore separated components, so
/// `tokio-rustls` is caught by two of them and `windows-sys` is caught by
/// neither.
const NETWORK_NAMES: &[&str] = &[
    "async-std",
    "reqwest",
    "hyper",
    "tokio",
    "smol",
    "mio",
    "socket2",
    "ureq",
    "curl",
    "isahc",
    "surf",
    "attohttpc",
    "minreq",
    "openssl",
    "rustls",
    "webpki",
    "quinn",
    "tonic",
    "tungstenite",
    "websocket",
    "trust",
    "hickory",
    "http",
    "h2",
    "h3",
];

#[test]
fn nothing_in_the_resolved_graph_is_outside_the_declared_list() {
    let resolved = resolved_packages();
    let allowed: BTreeSet<&str> = ALLOWED.iter().map(|(name, _)| *name).collect();

    let uninvited: Vec<&String> = resolved
        .iter()
        .filter(|name| !allowed.contains(name.as_str()))
        .collect();
    assert!(
        uninvited.is_empty(),
        "the resolved dependency graph carries {uninvited:?}, which nothing in \
         crates/einschlag/tests/nothing_goes_out.rs declares. docs/PRIVACY.md says \
         this tool sends nothing, and a package nobody looked at is where a network \
         stack arrives. Add it to ALLOWED with the reason it is there and what it \
         does, or take it out of the build."
    );

    let stale: Vec<&str> = allowed
        .iter()
        .filter(|name| !resolved.iter().any(|resolved| resolved == *name))
        .copied()
        .collect();
    assert!(
        stale.is_empty(),
        "the list in crates/einschlag/tests/nothing_goes_out.rs declares {stale:?} \
         and the resolved graph does not have it. A stale entry is a name somebody \
         can reuse without the reason beside it being read."
    );
}

#[test]
fn no_package_name_in_the_graph_belongs_to_a_network_stack() {
    let resolved = resolved_packages();
    let mut offences = Vec::new();

    for name in &resolved {
        if names_a_network_stack(name) {
            offences.push(name.clone());
        }
    }

    assert!(
        offences.is_empty(),
        "the resolved dependency graph carries {offences:?}, whose name says it \
         opens connections. docs/PRIVACY.md states that this tool sends nothing and \
         that the absence is by design rather than a thing not yet written."
    );
}

/// The reader has to be reading something. A graph of one package, or of none,
/// would pass both checks above while establishing nothing, and that is exactly
/// what a broken path or a renamed file would produce.
#[test]
fn the_reader_actually_read_the_resolved_graph() {
    let resolved = resolved_packages();
    assert!(
        resolved.len() >= 2,
        "the resolved graph reads as {resolved:?}, which is fewer packages than \
         this workspace has crates. The checks above would pass on nothing."
    );
    for own in ["einschlag", "einschlag-cli"] {
        assert!(
            resolved.iter().any(|name| name == own),
            "the resolved graph does not carry {own:?}, so it is not this \
             workspace's graph: {resolved:?}"
        );
    }
}

/// The offence the checks are written against, so the matcher is known to
/// recognise the shape rather than only the shapes absent from this tree.
#[test]
fn the_matcher_recognises_the_names_it_exists_for() {
    for name in [
        "reqwest",
        "tokio-rustls",
        "hyper-util",
        "async-std",
        "socket2",
    ] {
        assert!(
            names_a_network_stack(name),
            "the matcher does not recognise {name:?}"
        );
    }

    for name in ["windows-sys", "libm", "einschlag-cli", "serde", "bitflags"] {
        assert!(
            !names_a_network_stack(name),
            "the matcher refuses {name:?}, which it should not"
        );
    }
}

/// Whether a package name says the package opens connections.
///
/// The whole name and each of its components are compared, because a crate is
/// named either way round: `reqwest` is one word and `tokio-rustls` is two.
fn names_a_network_stack(name: &str) -> bool {
    NETWORK_NAMES.contains(&name)
        || name
            .split(['-', '_'])
            .any(|component| NETWORK_NAMES.contains(&component))
}

/// Every package name in the resolved lock file.
///
/// The file is read as text. A TOML library would be a direct dependency, which
/// would then be the first thing this check had to account for, and the form is
/// one key on its own line inside a `[[package]]` table. The reader refuses what
/// it does not recognise rather than returning an empty answer, because an empty
/// answer here reads exactly like a clean graph.
fn resolved_packages() -> Vec<String> {
    const TABLE: &str = "[[package]]";
    const KEY: &str = "name = ";

    let path = workspace_root().join("Cargo.lock");
    let text = fs::read_to_string(&path).unwrap_or_else(|why| {
        panic!(
            "cannot read {}: {why}. Cargo writes this file before it builds, so a \
             test run without it is a state this check does not know how to judge, \
             and it fails rather than reporting an empty graph.",
            path.display()
        )
    });

    let mut names = Vec::new();
    let mut inside = false;
    for raw in text.lines() {
        let line = raw.trim();
        if line.starts_with('[') {
            inside = line == TABLE;
            continue;
        }
        if !inside || !line.starts_with(KEY) {
            continue;
        }
        let value = line[KEY.len()..].trim();
        let name = value.trim_matches('"');
        assert!(
            !name.is_empty() && !name.contains('"') && value.starts_with('"'),
            "{}: the name line {line:?} is not a plain quoted string, and this \
             reader refuses what it cannot read rather than counting it as nothing",
            path.display()
        );
        names.push(name.to_owned());
        inside = false;
    }

    names.sort();
    names.dedup();
    names
}

fn workspace_root() -> PathBuf {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
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

/// Names in the standard library through which a socket is opened.
///
/// The graph check above cannot see any of these. `std` is not a package in
/// `Cargo.lock`, so a call written directly in this repository opens a
/// connection without a single package name changing, and #96 is the issue that
/// says so. This is the third of the three shapes that issue names: weaker than
/// reading the built artefact, and it catches the case the graph cannot see at
/// all.
///
/// Compared as substrings of the source text, which is enough for a name that
/// has to be written out to be used and is not enough for one reached through a
/// re-export somebody wrote to hide it. That is the bound.
const STANDARD_LIBRARY_NETWORKING: &[&str] = &[
    "std::net",
    "TcpStream",
    "TcpListener",
    "UdpSocket",
    "ToSocketAddrs",
];

/// The two files allowed to carry those names, because they are the lists.
const EXEMPT: [&str; 2] = [
    "crates/einschlag/tests/nothing_goes_out.rs",
    "crates/einschlag-cli/tests/nothing_goes_out_of_the_artefact.rs",
];

#[test]
fn no_source_file_in_this_workspace_reaches_the_networking_standard_library() {
    let root = workspace_root();
    let mut offending = Vec::new();

    for path in rust_sources(&root) {
        let relative = path
            .strip_prefix(&root)
            .expect("every walked path is under the root")
            .to_string_lossy()
            .replace('\\', "/");
        if EXEMPT.contains(&relative.as_str()) {
            continue;
        }
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        for name in STANDARD_LIBRARY_NETWORKING {
            if text.contains(name) {
                offending.push(format!("{relative}: {name}"));
            }
        }
    }

    assert!(
        offending.is_empty(),
        "{offending:?}. docs/PRIVACY.md states that this tool sends nothing, and \
         the dependency graph cannot see a call to the standard library, because \
         std is not a package in it."
    );
}

/// Every `.rs` file in the workspace, skipping the git directory and the build
/// output because nothing here authors either.
fn rust_sources(root: &Path) -> Vec<PathBuf> {
    const SKIPPED: [&str; 2] = [".git", "target"];
    let mut found = Vec::new();
    let mut pending = vec![root.to_path_buf()];

    while let Some(directory) = pending.pop() {
        let entries = fs::read_dir(&directory)
            .unwrap_or_else(|why| panic!("cannot read {}: {why}", directory.display()));
        for entry in entries {
            let path = entry.expect("the directory entry is readable").path();
            let name = path
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default();
            if path.is_dir() {
                if !SKIPPED.contains(&name.as_str()) {
                    pending.push(path);
                }
            } else if name.ends_with(".rs") {
                found.push(path);
            }
        }
    }
    found
}

/// The walker is looking at something, and the matcher matches what it exists
/// for.
#[test]
fn the_source_walker_and_its_matcher_are_both_working() {
    let root = workspace_root();
    let sources = rust_sources(&root);
    assert!(
        sources.len() > 5,
        "the walker found {} Rust files, which is fewer than this workspace has",
        sources.len()
    );
    for name in STANDARD_LIBRARY_NETWORKING {
        let synthetic = format!("let s = {name}::nothing();");
        assert!(
            synthetic.contains(name),
            "the matcher does not recognise {name}, which it exists for"
        );
    }
}
