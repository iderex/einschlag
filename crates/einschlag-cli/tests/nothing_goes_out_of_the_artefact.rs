//! Refuses a built binary that carries the machinery for opening a network
//! connection.
//!
//! `crates/einschlag/tests/nothing_goes_out.rs` reads the resolved dependency
//! graph and refuses a package outside a declared list and a package whose name
//! belongs to a network stack. Every claim it makes is about names, and #96 is
//! the issue that says so: code written directly in this repository can call the
//! standard library and open a socket without any package name changing, and
//! that check stays green, because `std` is not a package in the graph.
//!
//! This one reads the artefact. Not a name in a manifest, not a line of source:
//! the bytes Cargo produced, which are what an operator runs.
//!
//! **What it reads and what that is worth.** A linker records the name of every
//! symbol and every library a program reaches outside itself, and it records
//! them as text, so those names are in the file. This searches for the ones a
//! socket is opened through. It is a search rather than a parse: nothing here
//! reads an import table or a symbol table, because doing so needs a format
//! parser per platform, and `docs/DEPENDENCIES.md` is where the cost of the
//! crate that would supply one would have to be argued.
//!
//! **What it cannot do**, stated here and at the claim in `docs/PRIVACY.md`. It
//! reads the names of things outside the program, so a program that reaches a
//! socket through a syscall it makes by hand records no name for it and passes.
//! It cannot read the standard library's own type names, for the measured reason
//! written at the list below, so a type named and never linked to anything
//! outside the program passes too and is caught by the source-level half
//! instead. A name present for an unrelated reason fails it, which is the safe
//! direction and is why the failure prints what it found. And a run that opens a
//! socket is still not observed by anything: that is the second shape #96 names
//! and it is not taken here.

use std::fs;
use std::path::Path;

/// Names of things outside the program that a socket is opened through.
///
/// Compared case-insensitively over the raw bytes. Every one of them is a symbol
/// or a library the program imports rather than a name it merely contains, which
/// is the distinction that makes the list work and is not the list this file
/// started with.
///
/// **The standard library's own type names are deliberately absent, and that was
/// measured rather than reasoned.** The first version of this check listed
/// `TcpStream`, `TcpListener`, `UdpSocket`, `SocketAddr` and `socket`. On Windows
/// all five were absent from a clean build and the check passed. On Linux all
/// five were present in a clean build, because the standard library is linked in
/// statically and its type names survive in a debug binary whether or not any of
/// it is reached. A check that fails on every clean build of one platform is not
/// a check, so those five are out and the source-level half of this property, in
/// `crates/einschlag/tests/nothing_goes_out.rs`, is what catches a type name
/// written here.
///
/// What is left is measured absent from a clean build on both platforms and
/// measured present once the capability is reintroduced. The pull request that
/// landed this file quotes both runs.
const NETWORK_MARKERS: &[&str] = &[
    "ws2_32",
    "wsastartup",
    "wsasocket",
    "wsaconnect",
    "getaddrinfo",
    "freeaddrinfo",
    "gethostbyname",
];

/// The binary Cargo built for this crate, which is the thing an operator runs.
const ARTEFACT: &str = env!("CARGO_BIN_EXE_einschlag");

#[test]
fn the_built_artefact_carries_no_name_a_socket_is_opened_through() {
    let path = Path::new(ARTEFACT);
    let bytes = fs::read(path).unwrap_or_else(|why| {
        panic!(
            "cannot read the built artefact at {}: {why}",
            path.display()
        )
    });
    assert!(
        !bytes.is_empty(),
        "the artefact at {} is empty, so nothing was searched and nothing is established",
        path.display()
    );

    let found: Vec<&str> = NETWORK_MARKERS
        .iter()
        .copied()
        .filter(|marker| contains_ignoring_case(&bytes, marker.as_bytes()))
        .collect();

    assert!(
        found.is_empty(),
        "the built artefact at {} carries {found:?}. docs/PRIVACY.md states that \
         this tool sends nothing, and a name a socket is opened through is in the \
         binary an operator runs. Either something in this workspace reached the \
         networking part of the standard library, or a marker is present for an \
         unrelated reason and this list is too wide; the first is the case this \
         exists for.",
        path.display()
    );
}

/// The scan is looking at something.
///
/// Without this, a marker list that matched nothing because the search was
/// broken would read exactly like a clean binary.
#[test]
fn the_search_finds_what_is_there() {
    let bytes = fs::read(ARTEFACT).expect("the built artefact is readable");
    assert!(
        contains_ignoring_case(&bytes, b"einschlag"),
        "the artefact does not carry its own name, so the search is not reading it"
    );
    assert!(
        !contains_ignoring_case(&bytes, b"this string is not in any binary anywhere"),
        "the search matches text that is not there"
    );
    assert!(
        !NETWORK_MARKERS.is_empty(),
        "the marker list is empty, so the check above asserts nothing"
    );
}

/// Whether `needle` appears anywhere in `haystack`, comparing ASCII letters
/// without regard to case.
///
/// Written out rather than taken from a crate: it is one loop, and
/// `docs/DEPENDENCIES.md` requires a reason for every direct dependency.
fn contains_ignoring_case(haystack: &[u8], needle: &[u8]) -> bool {
    if needle.is_empty() || needle.len() > haystack.len() {
        return false;
    }
    haystack.windows(needle.len()).any(|window| {
        window
            .iter()
            .zip(needle)
            .all(|(a, b)| a.eq_ignore_ascii_case(b))
    })
}
