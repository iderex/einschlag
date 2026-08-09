# Building

## The one command

```
cargo build --release --locked
```

Run it from the root of a clone. It builds every crate in the workspace and
leaves a binary at `target/release/einschlag`
(`target\release\einschlag.exe` on Windows).

`--locked` is part of the command rather than a flag for the build server.
`Cargo.lock` is tracked, and `--locked` refuses to build where it would have to
change, so a dependency added without the lock being committed stops here instead
of resolving quietly into a graph nobody reviewed. The one time to leave it off
is the moment a dependency is being added on purpose: run the command without it
once, commit the lock the resolver wrote, and put it back.

There is no configure step and no code generation. The only network access is
Cargo fetching the packages the lock names, which today is one package outside
this workspace, `libm`. `docs/DEPENDENCIES.md` says what it is for.

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
Cargo and the standard library, and neither the formatter nor the linter is in
that set. Both are named in the `components` list beside it, so rustup installs
them from the tree rather than from an instruction somebody has to be given, and
both are the build of them that ships with the pinned compiler. "The formatter"
and "the linter" below are those two, and nothing chooses a different one.

**A build that reaches `cargo` without rustup in front of it is not stopped by
this pin, and it is refused by a test.** A distribution-packaged toolchain, a
vendored build, an image that installed Cargo directly, or `cargo +1.96.0` typed
by hand will all use a compiler this file never chose. The manifest states no
minimum version, because #21 requires the version in exactly one tracked file and
Cargo cannot read a manifest key from another file. So the disagreement is
refused rather than prevented: `crates/einschlag/build.rs` records the compiler
Cargo actually used and `crates/einschlag/tests/toolchain_pin.rs` fails when it
differs from the channel above.

```
$ cargo +1.96.0 test -p einschlag --test toolchain_pin
test the_compiler_that_built_this_is_the_one_the_pin_names ... FAILED

thread 'the_compiler_that_built_this_is_the_one_the_pin_names' panicked at crates\einschlag\tests\toolchain_pin.rs:31:5:
assertion `left == right` failed: this build used rustc 1.96.0 and rust-toolchain.toml pins 1.97.0.
```

**What that does not do is stop the build.** It compiles, and the test says so
afterwards. A build made with `--release` and never tested still ships. The
mechanism `docs/decisions/0002-language-and-toolchain.md` asked for, a compiler
error naming the version, would have needed a second literal in the manifest, and
that is the trade #81 recorded and this took.

**The version is in one file that configures a build, and in two documents that
quote it.** `crates/einschlag/tests/toolchain_pin.rs` refuses a second
configuration point: another manifest, a source file or a workflow setting the
version fails. It does not refuse a document. This page and
`docs/decisions/0002-language-and-toolchain.md` both write the number out, each
inside a command transcript that shows where it came from, and a record is never
edited in place so the second one cannot be repaired even if it should be. #21's
body claimed the number appeared in exactly one tracked file, on a search that
excluded `docs/`; that claim held only under that exclusion and this paragraph is
the correction.

## The formatter and the linter

Two commands, and each one refuses rather than reports.

```
$ cargo fmt --all -- --check
$ echo $?
0
```

```
$ cargo clippy --workspace --all-targets --locked
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 8.58s
$ echo $?
0
```

`--all` and `--workspace` because a crate left out of the command is a crate the
rules do not reach, and `--all-targets` because the tests are most of the source
in this tree today. Both were run at the commit this paragraph landed on.

**Neither prints a warning and carries on.** `cargo fmt --check` writes the diff
it would have applied and exits non-zero, which is the formatter's own behaviour.
The linter's is this repository's: the lint levels are in the `[workspace.lints]`
table of the root `Cargo.toml`, not in a flag on a command line, so `deny` is what
the manifest says and every route that compiles this workspace carries it.
`cargo build` and `cargo test` refuse a warning too, and an editor running
`cargo check` shows it as an error at the moment it is written rather than at the
end of a pipeline.

A flag would have held only where the flag was typed, which is the failure this
avoids: a contributor whose own machine was quiet learning about the rule from a
red build on work that is already finished.

What is denied, and it is a list this document does not restate because the
manifest is the authority for it:

```
$ sed -n '/^\[workspace.lints/,$p' Cargo.toml
```

Two things about that table are worth reading before changing it. The rustc half
denies the whole `warnings` group, which is safe only because `rust-toolchain.toml`
pins an exact release, so the set cannot grow underneath the tree. The clippy half
takes the default group and then names a short list out of `clippy::pedantic`
individually: the casts between integers and floats, and exact comparison of
floats. Those are what this project is for. The rest of `pedantic` is not
enabled, and most of what it would add here is `#[must_use]` on functions that
have no caller yet.

**The levels are `deny` rather than `forbid`, so a site with a reason can carry
an `#[expect]`.** Where they are, and each one says in place why it is there:

```
$ git grep -c 'clippy::' -- 'crates/**/*.rs'
crates/einschlag/src/math.rs:1
crates/einschlag/src/sampling.rs:3
```

`#[expect]` rather than `#[allow]` on purpose. An `expect` that stops being
needed is itself a warning, so a suppression outliving the code it was written
for is refused rather than sitting there.

**What neither command judges is whether the code is right.** A formatter reads
whitespace and a linter reads patterns. `docs/TESTING.md` is the suite, and the
two are not substitutes for each other.

Both of them run on a push and on a pull request as well, under the names the
next section is about.

## What runs on a push, and under which names

`.github/workflows/ci.yml` builds this workspace, runs the suite, and runs the
two commands above, on every pull request and on every push to `main`. It is the
first workflow in this repository that compiles the code; the five beside it read
the tree, the history and each other.

**It produces four check runs, and their names are these four words:**

```
build
test
fmt
lint
```

**Those names are matched literally elsewhere and are not free to change.** A
branch protection rule requires a check by name, so a rename that nobody notices
removes a requirement without removing anything a reader would see: the rule goes
on naming a check that no longer exists, the pull request goes on being
mergeable, and nothing says so. Renaming one is a deliberate act that has to move
whatever names it. This paragraph is the only place in this document that writes
the four out, and the workflow file carries the same sentence at the top.

**Nothing requires them today.** The ruleset on this repository requires no
status check at all, so a pull request whose `build` is red can still be merged.
`CONTRIBUTING.md` quotes the ruleset with the command that produced it, and
`docs/QUALITY-PARITY.md` compares it against a repository that does require its
checks. Requiring them is entry 3 of issue #1 and is a repository setting rather
than a change anybody can make in a pull request.

**The workflow types what this document and `docs/TESTING.md` type.** Not a
variant, not a superset:

```
$ grep -n 'run: cargo' .github/workflows/ci.yml
```

Two procedures that drift apart is how a green build stops meaning anything. The
lint job in particular carries no `-D warnings` on its command line, because the
levels are in the manifest and a flag there would be a second place to set them.

**No job installs a toolchain.** `rust-toolchain.toml` is the one tracked file
that carries the version, and the `rustup` already on the runner reads it and
fetches the pinned compiler with the components named beside it. An action taking
a version as an input would be a second place to set it, which
`crates/einschlag/tests/toolchain_pin.rs` refuses by name.

**No cache.** Each job compiles from nothing, which costs minutes on a workspace
this size and removes a class of failure where a job passes on something a clone
would not produce. That trade is worth re-running when the build gets long enough
to notice, and it has not been measured here.

**What runs there is Linux, and nothing else.** `runs-on: ubuntu-latest` on all
four jobs, so a break that only appears on Windows or macOS is not caught by any
of this. The determinism promise in
`docs/decisions/0009-determinism.md` is the reason that matters, and no issue
holds a second platform today.

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

**The third field used to be able to sit one build behind, so a modified tree
could report as matching.** The marker is derived when the build script runs, a
build script does not run on every build, and emitting any `rerun-if-changed`
turns off Cargo's rule that a change inside the package reruns the script. The
list named the core crate and three files in the git directory, so an edit to the
front end, a document or a workflow, built with nothing having touched the git
index in between, left the previous marker in place. #84 measured it on Windows
and it read as intermittent, because unrelated git commands rewrite the index and
hide it.

**What the script watches now is everything the two fields are about.** Every
entry at the top of the working tree, minus `target`, which is the build's own
output, and minus `.git`, which is not part of the working tree; then `HEAD`, the
branch ref and the index, which move without a byte of the working tree moving.
Print it, rather than reading this paragraph for it:

```
$ grep -h 'rerun-if-changed' target/release/build/einschlag-*/output
```

That file is where Cargo keeps what the script last said, so the command answers
after any build rather than only after one that reran the script.

A directory in that list is scanned in full, which is Cargo's documented
behaviour for a `rerun-if-changed` path that is a directory. That is the reason
this shape was taken and the reason the two alternatives were not: emitting the
workspace root would have included `target` and made every build rerun the one
before it, and relying on a path that does not exist to force a rerun every time
is widely used and is not a documented guarantee. A provenance field on a tool
whose output may be read in court should not rest on an observed side effect.

**What that still leaves.** Cargo compares modification times, so a file whose
contents change without its timestamp moving is not seen; restoring an older copy
over a newer one is the way that happens. A change under `target` is invisible on
purpose. And the script reports what `git status` said at the moment it ran: an
edit made while the compiler is still running is not in the answer, which is a
race no ordering fixes.

**The script no longer writes a path it watches.** `git status` refreshes the
index as a side effect, the index is in the watch list, so each run of the script
caused the next build to run it again. It is invoked as `git --no-optional-locks
status --porcelain`, which is the documented way to ask git not to take that
lock, and the extra rebuild is gone with it. That second-order effect is most of
why the original behaviour looked inconsistent rather than simply wrong.

**A real clone used to report `working tree had uncommitted changes at build
time` on every build**, because `Cargo.lock` was untracked and Cargo wrote it
before the build script ran. The lock is tracked now, so an unmodified clone
reaches a clean tree and the reading above is one an ordinary build produces
rather than one a local exclusion had to be arranged for.

## The layout

`crates/einschlag` is the library: the geometry, the sampling and the uncertainty
propagation, everything the tool computes. It is empty today.

`crates/einschlag-hardware-harness` holds the runs that need equipment. It is a
workspace member, so it is compiled by every build here and never executed by
one; `docs/TESTING.md` is where that arrangement is argued.

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

**Nothing refuses a lock that was updated on purpose and badly.** `--locked`
refuses a build where the lock would have to change. It says nothing about a
commit that changes the lock and the manifest together, which is exactly what
adding a dependency looks like, so a version bump nobody reviewed passes every
route here. What stands against that is `docs/DEPENDENCIES.md`, the entry it
requires per direct dependency, and the reader of the diff.

**Nothing verifies what a package contains.** The lock records a checksum, and
Cargo refuses a package whose bytes do not match it, so a registry serving
different content for a version that was already resolved is caught. A first
resolution of a new version is trusted at the moment it happens.
`.github/workflows/dependency-review.yml` refuses a newly added dependency with a
known vulnerability, which is a different question again.

**The test command is not here.** `cargo test --locked` and what it prints are
`docs/TESTING.md`.

**Nothing that makes a check run a precondition of a merge.** The four check
runs exist and report; the ruleset requires none of them, so a red pull request
is mergeable. That is a repository setting and entry 3 of issue #1, and the
section above is where it is set out.

**No release artefact and no cross-compilation.** Issue #69.

**No reproducible-build claim.** `docs/decisions/0009-determinism.md` promises
that the same input, the same seed and the same build produce byte-identical
output. Nothing here measures that, and the scaffold produces no output to
measure. Issue #77 carries the part of it that this language makes hard.
