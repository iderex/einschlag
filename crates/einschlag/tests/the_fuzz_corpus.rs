//! Refuses a fuzz corpus that has drifted from the fixtures it was seeded with.
//!
//! Issue #58 asks for a corpus seeded from the fixtures in #33 and tracked, so
//! that a later run starts from what earlier runs learned. Seeding is a thing
//! somebody does once and the drift arrives afterwards: a fixture is added, or
//! one is edited to prove a different refusal, and the corpus goes on holding
//! the file as it was. A fuzzer starting from a stale seed spends its budget
//! rediscovering the shape of a file the tree already has.
//!
//! Nothing here runs the fuzzer. `cargo fuzz` needs a nightly compiler and an
//! instrumented build, which `fuzz/Cargo.toml` says why the default suite cannot
//! carry, so what this test judges is the material a run starts from and never
//! the run. `docs/TESTING.md` states that bound at the section on fuzzing.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Where the fixtures the corpus is seeded from live.
const FIXTURES: &str = "fixtures/scene";

/// Where the seeded corpus lives, which is the directory `cargo fuzz run` reads.
const CORPUS: &str = "fuzz/corpus/input_parser";

#[test]
fn every_fixture_is_in_the_corpus_with_the_bytes_it_has_today() {
    let root = workspace_root();
    let fixtures = fixture_files(&root.join(FIXTURES), &root);
    assert!(
        !fixtures.is_empty(),
        "no fixture was found under {FIXTURES}, so this check would pass on nothing"
    );

    let corpus = root.join(CORPUS);
    let mut missing = Vec::new();
    let mut differing = Vec::new();

    for (relative, path) in &fixtures {
        let seeded = corpus.join(corpus_name(relative));
        let Ok(bytes) = fs::read(&seeded) else {
            missing.push(corpus_name(relative));
            continue;
        };
        if bytes != fs::read(path).expect("the fixture is readable") {
            differing.push(corpus_name(relative));
        }
    }

    assert!(
        missing.is_empty(),
        "the fuzz corpus has no entry for {missing:?}. A fixture is a file somebody \
         wrote to reach one refusal, which is the cheapest seed a run can start \
         from. Copy it into {CORPUS} under that name."
    );
    assert!(
        differing.is_empty(),
        "the fuzz corpus entry for {differing:?} is not the fixture it was seeded \
         from. Either the fixture was edited and the copy was not, or the copy was \
         edited, and a seed nobody can trace back to a fixture is a seed nobody \
         maintains."
    );
}

#[test]
fn the_corpus_holds_no_entry_named_after_a_fixture_that_is_gone() {
    let root = workspace_root();
    let expected: BTreeMap<String, ()> = fixture_files(&root.join(FIXTURES), &root)
        .into_keys()
        .map(|relative| (corpus_name(&relative), ()))
        .collect();

    let corpus = root.join(CORPUS);
    let entries = fs::read_dir(&corpus)
        .unwrap_or_else(|why| panic!("cannot read {}: {why}", corpus.display()));

    let mut orphaned = Vec::new();
    for entry in entries {
        let name = entry
            .expect("the directory entry is readable")
            .file_name()
            .to_string_lossy()
            .into_owned();
        // An entry a run added is not named after a fixture and is left alone.
        // That is the half of the corpus this check deliberately does not judge:
        // what a run learned is not derivable from the tree.
        if name.starts_with("scene-") && !expected.contains_key(&name) {
            orphaned.push(name);
        }
    }

    assert!(
        orphaned.is_empty(),
        "the fuzz corpus holds {orphaned:?}, named after a fixture that is not \
         under {FIXTURES} any more. A seed carrying a refusal the parser no longer \
         has reads as a case somebody is still proving."
    );
}

/// The name a fixture is seeded under, which is its path with the separators
/// flattened. libFuzzer reads a directory rather than a tree, so a fixture in a
/// subdirectory has to arrive as one file with a name that still says where it
/// came from.
fn corpus_name(relative: &str) -> String {
    let inside = relative
        .strip_prefix(FIXTURES)
        .and_then(|rest| rest.strip_prefix('/'))
        .unwrap_or(relative);
    format!("scene-{}", inside.replace('/', "-"))
}

/// Every TOML file under the fixtures directory, keyed by its path from the
/// workspace root with forward slashes.
fn fixture_files(directory: &Path, root: &Path) -> BTreeMap<String, PathBuf> {
    let mut found = BTreeMap::new();
    let mut pending = vec![directory.to_path_buf()];

    while let Some(next) = pending.pop() {
        let entries = fs::read_dir(&next)
            .unwrap_or_else(|why| panic!("cannot read {}: {why}", next.display()));
        for entry in entries {
            let path = entry.expect("the directory entry is readable").path();
            if path.is_dir() {
                pending.push(path);
            } else if path.extension().is_some_and(|suffix| suffix == "toml") {
                let relative = path
                    .strip_prefix(root)
                    .expect("every walked path is under the root")
                    .to_string_lossy()
                    .replace('\\', "/");
                found.insert(relative, path);
            }
        }
    }
    found
}

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
