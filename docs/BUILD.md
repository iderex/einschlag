# Building

## The one command

```
cargo build --release
```

Run it from the root of a clone. It builds both crates and leaves a binary at
`target/release/einschlag` (`target\release\einschlag.exe` on Windows).

There is no configure step, no code generation and no network access beyond what
Cargo needs for dependencies, of which there are none outside this workspace
today.

There is one build script, `crates/einschlag/build.rs`. It runs `git` twice to
derive the commit the build was made from, writes two environment values for the
compiler, and does nothing else. A build where `git` is missing still succeeds
and reports the commit as `unknown`. What that field means and where it is weak
is under "Which code produced an artefact" below.

## The pinned toolchain

The version is pinned in `rust-toolchain.toml`, and that is the only tracked file
that carries it. This document does not restate the number, because a number
written in two places drifts and the drift is invisible. Print it:

```
$ grep '^channel' rust-toolchain.toml
channel = "1.97.0"
```

`rustup` reads that file and selects the matching compiler on its own. Nobody has
to be told to switch, and a clone that has a different default toolchain still
builds with the pinned one. Confirm which compiler a build will use:

```
$ rustc --version
rustc 1.97.0 (2d8144b78 2026-07-07)
$ cargo --version
cargo 1.97.0 (c980f4866 2026-06-30)
```

`profile = "minimal"` in the same file keeps a fresh install to the compiler,
Cargo and the standard library. The components a formatter and a linter need are
added by the issue that lands them, which is #22, rather than being installed
here for a build that does not use them.

**A build that reaches `cargo` without rustup in front of it is not covered by
this pin.** A distribution-packaged toolchain, a vendored build or an image that
installed Cargo directly will use whatever compiler it has, and this tree carries
nothing that refuses it. The manifest states no minimum version, and the reason
is that #21's Done-when requires the version in exactly one tracked file while
Cargo has no way to read a manifest key from another file. Issue #81 holds the
mechanism. Until it lands, an older compiler on that route fails inside the
source rather than with a message naming a version.

## What a clean clone needs

`rustup`, and nothing else. Everything the build reads is in the tree.

Verified by cloning the pushed commit into a fresh directory and building there;
the commands and their output are in the body of the pull request that landed
this file, run against the pushed commit rather than against a working tree.

**Not verified: a machine that has only the pinned toolchain installed and no
other.** The machine this was checked on has several toolchains installed
alongside 1.97.0, and `rustup` selecting the pinned one is what was observed. A
machine holding 1.97.0 and nothing else was not available, so that half of #21's
Done-when is a claim rather than a measurement. What was measured is that the
clone needs nothing from outside itself except rustup.

## Running it

```
$ ./target/release/einschlag
einschlag: shooting-scene reconstruction that states what it cannot exclude.

usage:
  einschlag --version    print the version and the commit this was built from
  einschlag              print this text

No subcommand that computes anything is implemented yet. This build is the
scaffold: it exists so that the build, the test harness and the release route
can be checked before there is anything to compute. Any other argument prints
this same text and exits zero, because no argument grammar has been decided
beyond --version.

docs/BUILD.md says how this was built. docs/decisions/ says what it will do and
why, and is readable without the source.
```

It exits zero. There is nothing else it does yet.

## Which code produced an artefact

An output read years later, next to a report, has to be traceable to the code
that produced it. The one command that answers that:

```
$ ./target/release/einschlag --version
einschlag 0.0.0
commit 78750c2e79ecc1f756172c9f2a74b425bc1d5834, working tree matched this commit
```

The version comes from the manifest. The commit is derived by
`crates/einschlag/build.rs` at build time by running `git`, not written into a
file, because a literal is a statement about the last time somebody remembered to
update it.

The third field is whether the source the build was made from matched that
commit. It reads `working tree matched this commit`, `working tree had
uncommitted changes at build time`, or `working tree state unknown`. A build made
where `git` could not answer, from an unpacked source archive for instance,
reports the commit as `unknown` rather than guessing.

**The third field can be one build behind, and a modified tree can therefore
report as matching.** The marker is derived when the build script runs, and a
build script does not run on every build. It reruns when the core crate's `src`,
`Cargo.toml` or `build.rs` changes, or when `HEAD`, the branch ref or the git
index changes. An edit anywhere else in the workspace, built with nothing having
touched the git index in between, leaves the previous marker in place. Reproduced
on Windows and intermittent, because unrelated git commands rewrite the index and
hide it. Issue #84 holds the measurement and the mechanism, and quotes the runs.

**Until #26 lands the lock file, a real clone reports `working tree had
uncommitted changes at build time` on every build**, because `Cargo.lock` is
untracked and Cargo writes it before the build script runs. The clean reading
quoted above came from a scratch copy with `Cargo.lock` in `.git/info/exclude`,
which is a local exclusion and not something in the tree.

## The layout

`crates/einschlag` is the library: the geometry, the sampling and the uncertainty
propagation, everything the tool computes. It is empty today.

`crates/einschlag-cli` is the command line front end. It depends on the library
as an ordinary path dependency, so the compiler refuses it access to anything the
library has not marked `pub`. That is the point of the split rather than a side
effect of it: `docs/decisions/0002-language-and-toolchain.md` commits this project
to being drivable from outside by a documented input format, a documented output
artefact and a command line contract, and a front end wired into the library's
internals is a front end no second one can be written beside.

`fixtures/` is the test material. `docs/TESTING.md` says what goes there, and it
holds nothing yet.

## What is not here

**No lock file.** `Cargo.lock` is not tracked, so a resolver that picked a newer
dependency would not be refused. There are no dependencies outside the workspace
today, which bounds what that can currently do rather than removing it. Issue #26
commits the lock and adds `--locked` to the routes that build and test.

**The test command is not here.** `cargo test` and what it prints are
`docs/TESTING.md`.

**No formatter, no linter, no continuous integration that builds anything.** The
workflows in `.github/workflows/` at this commit check sign-off, dependencies,
Unicode and the workflow files themselves. None of them compiles this code, so
nothing outside a developer's own machine has built it. Issues #22 and #24.

**No release artefact and no cross-compilation.** Issue #69.

**No reproducible-build claim.** `docs/decisions/0009-determinism.md` promises
that the same input, the same seed and the same build produce byte-identical
output. Nothing here measures that, and the scaffold produces no output to
measure. Issue #77 carries the part of it that this language makes hard.
