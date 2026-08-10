# Direct dependencies

Every dependency is code somebody else wrote that becomes part of the answer this
tool gives. That answer may be challenged, and the person defending it has to be
able to say what each piece of borrowed code does and why it is there.

This file is the list, and it is checked. `dependency_budget.rs` in
`crates/einschlag/tests/` reads the manifests in the workspace and compares the
direct dependencies it finds against the entries below, in both directions: a
dependency with no entry fails, and an entry naming a dependency nothing uses
fails. A stale entry is how a file like this stops being read.

It reads one manifest Cargo does not reach from the root as well. `fuzz/Cargo.toml`
declares its own workspace, for the reason written at the top of that file, and a
dependency added there was invisible to this check until it was named. Which
manifests are read is not a list a person keeps in step: the check refuses a
`Cargo.toml` in the tree that is neither the root, nor a member, nor named as one
of the manifests outside the workspace.

## The ceiling

The ceiling is 8 direct dependencies.

It is a number to argue past, not a wall. Raising it is a change to this line
with the reason in the pull request that raises it, and the point of the number
is that the argument has to happen at all. Eight rather than four because
milestone 5 needs sampling and linear algebra and milestone 6 needs a
serialisation format, and rather than sixteen because
`docs/decisions/0002-language-and-toolchain.md` already accepted writing more of
the mathematics here in exchange for a smaller surface.

The count the ceiling applies to is direct dependencies, which is what this
project chose. It says nothing about the transitive set, and a single direct
dependency can bring in fifty. Whoever adds one is expected to look, and nothing
here makes them.

## What an entry says

Four things: what the crate is used for, what doing without it would cost, its
licence, and the name of the crate as the heading.

The licence matters twice over. This repository has no LICENSE file of its own,
because the licence is an open maintainer decision on #1, and a dependency whose
terms conflict with whatever is chosen there is a problem discovered late.

## The direct dependencies

There are three. Two are in the workspace and one is in the fuzz crate, which is
its own workspace and is read by the check anyway; the entry says why.

### libm

**What it is used for.** Every transcendental function the numeric core
evaluates: the arcsine the ellipse relation is built on, the trigonometry that
turns a direction into a scene frame, and the exponentials and logarithms the
uncertainty model needs.
`docs/decisions/0013-platform-math-out-of-the-numeric-core.md` argues the choice
and `crates/einschlag/src/math.rs` is the one place it is called from.

**What doing without it would cost.** The alternative in reach is the standard
library, whose documentation attaches an "Unspecified precision" note to those
same functions, permitting the result to vary by platform, by compiler release,
and from one invocation to the next inside a single execution. The last of those
is inside the promise `docs/decisions/0009-determinism.md` makes rather than
inside its stated bound. The other alternative is writing the functions here,
which is a numerical-methods project this repository would then own and test
forever.

**Its licence.** MIT, read from the resolved package rather than from a
README:

```
$ cargo metadata --format-version 1 | python -c "import json,sys; d=json.load(sys.stdin); print([(p['name'], p['version'], p['license'], p['repository']) for p in d['packages'] if p['name']=='libm'])"
[('libm', '0.2.16', 'MIT', 'https://github.com/rust-lang/compiler-builtins')]
```

MIT code may be combined into a work distributed under AGPL-3.0, which is what
entry 1 of #1 decided and what `LICENSE` now carries. The obligation it brings is
that the notice and the permission text travel with any distribution, which is a
thing a release has to carry rather than a thing this file settles.

**The version requirement is exact.** `= 0.2.16` rather than a range. A later
release of this crate may return a different value in the last place, which is
the whole thing `docs/decisions/0013-platform-math-out-of-the-numeric-core.md`
pins, so a range would leave the promise resting on whatever the resolver felt
like.

`Cargo.lock` is tracked now, and the lock alone would hold the version for
anybody building from this repository. It would not hold it for anybody depending
on this crate as a library, because a lock file is ignored for a dependency, and
the record's promise is about the arithmetic rather than about who is building.
Relaxing the requirement is therefore a decision about that record and not a
tidying-up that follows from the lock, and it has not been taken.

**What it brings with it.** Nothing. It is the only package in the graph besides
this workspace's own two. The absolute path the first line printed is shortened
here to the part that is the same in every clone:

```
$ cargo tree -p einschlag --charset ascii
einschlag v0.0.0 (crates\einschlag)
`-- libm v0.2.16
```

### toml_parser

**What it is used for.** Reading the operator's input file.
`docs/decisions/0007-input-format.md` fixes that file as TOML and names the cost
of the choice in one line, "a TOML dependency in whatever language decision 0002
chooses". This is that dependency. `crates/einschlag/src/input/document.rs` is
the one place it is called from, and what that file does with the events is
organise them into the four containers the format has; the grammar is not
reimplemented there.

**What doing without it would cost.** A TOML implementation written and owned
here. The record that chose the format refuses that route by name, and says a
record whose language has no maintained implementation is superseded rather than
worked around with a hand-written parser. The input file is also what issue #58
will fuzz, and a grammar this project wrote is a grammar this project would have
to harden alone.

**Its licence.** MIT or Apache-2.0, at the taker's choice, read from the
resolved package rather than from a README:

```
$ cargo metadata --format-version 1 | python -c "import json,sys; d=json.load(sys.stdin); print([(p['name'], p['version'], p['license'], p['repository']) for p in d['packages'] if p['name'] in ('toml_parser','winnow')])"
[('toml_parser', '1.1.3+spec-1.1.0', 'MIT OR Apache-2.0', 'https://github.com/toml-rs/toml'), ('winnow', '1.0.4', 'MIT', 'https://github.com/winnow-rs/winnow')]
```

Both may be combined into a work distributed under AGPL-3.0, which is what entry
1 of #1 decided. The obligation is the same one `libm` brings: the notice and the
permission text travel with any distribution, which a release has to carry rather
than a thing this file settles.

**The version requirement is exact.** `= 1.1.3` rather than a range, for a
different reason from `libm`'s. Nothing about arithmetic rests on it. What rests
on it is that the refusals in `crates/einschlag/tests/the_input_parser.rs` name
lines, and a release that recovered from a malformed file differently would move
which line a refusal is reported at without moving anything a reader would
notice.

**What it brings with it.** One package, `winnow`, which is the parser combinator
library it is built on.

```
$ grep -c '^name = ' Cargo.lock
6
$ git show main:Cargo.lock | grep -c '^name = '
4
```

It was taken over `toml_edit`, which is the document model above it in the same
family and which the same author maintains. That one was resolved first, on this
branch, and the lock it produced was counted the same way before the change was
undone: eighteen packages rather than six, adding `serde_core`, `serde_derive`,
`syn`, `proc-macro2`, `quote`, `unicode-ident`, `memchr`, `indexmap`,
`hashbrown`, `equivalent` and `toml_datetime` beside these two. That count is a
measurement of a tree that no longer exists and no command here reproduces it;
what reproduces it is `cargo add --package einschlag toml_edit@=0.25.13
--no-default-features --features parse` in a scratch clone, and it is written
here as the reason for the choice rather than as a figure to be quoted.

The four proc-macro packages in that list are not compiled under the features
that would have been on, and they would still each need a line in the list
`crates/einschlag/tests/nothing_goes_out.rs` holds, which is what somebody checks
when they want to know that nothing in this build can open a socket. Two names to
read rather than fourteen is what the choice cost, and what it bought is the
document layer sitting in this tree instead of in a dependency.

```
$ cargo tree -p einschlag --charset ascii
einschlag v0.0.0 (crates\einschlag)
|-- libm v0.2.16
`-- toml_parser v1.1.3+spec-1.1.0
    `-- winnow v1.0.4
```

**Default features are off**, and `std` is asked for by name. The crate builds
without the standard library and this project does not need it to; what `std`
carries here is the allocation the reader uses to hold events and decoded text.
Nothing else in the feature list is on.

The workspace has three crates and `einschlag-cli` depends on `einschlag` by path.
That is one part of this project depending on another part of it, not code
somebody else wrote, and the check does not count it. A path dependency pointing
outside the workspace would be counted, because that is somebody else's code
arriving by a different route.

The provenance the tool reports comes from `git` invoked as a subprocess in a
build script rather than from a crate, which `docs/BUILD.md` records and the pull
request that landed #29 argues.

`docs/decisions/0002-language-and-toolchain.md` names `rand`, `nalgebra`,
`statrs` and `libm` as candidates and decides none of them. Two have been taken
since. #77 took `libm`, in
`docs/decisions/0013-platform-math-out-of-the-numeric-core.md`. #37 took `rand`
and declined it, in `docs/decisions/0014-the-sampling-generator.md`: the draw
sequence is arithmetic written in this repository, so nothing here supplies
randomness and the count above is unmoved. `nalgebra` and `statrs` are still
undecided, and #43 is the issue that would take one for the output artefact. The
TOML implementation `docs/decisions/0007-input-format.md` named as the cost of
the input format is the third taken, by #33.

### libfuzzer-sys

**What it is used for.** The one fuzz target, in `fuzz/fuzz_targets/`. It is the
Rust binding to libFuzzer, which is the engine that generates the inputs, keeps
the ones that reach new code, and reports the one that failed. Issue #58 is where
the target is asked for, and `docs/TESTING.md` states the time bound a run is
given and what a run of that length does and does not cover.

**What doing without it would cost.** A generator written here, driving the
parser with inputs this project invented. That is a different thing and a weaker
one: libFuzzer chooses its next input from the coverage the last one reached, so
it walks into branches nobody thought of, and a generator written from a reading
of the parser can only reach the branches its author already knew about. The
parser is the surface that reads a file arriving from outside, which is the whole
reason #58 exists, and a hand-written generator would leave the interesting half
of it unvisited.

**Its licence.** `(MIT OR Apache-2.0) AND NCSA`, read from the resolved package:

```
$ cd fuzz && cargo +nightly metadata --format-version 1 | python -c "import json,sys; d=json.load(sys.stdin); print([(p['name'], p['version'], p['license']) for p in d['packages'] if p['name']=='libfuzzer-sys'])"
[('libfuzzer-sys', '0.4.10', '(MIT OR Apache-2.0) AND NCSA')]
```

The NCSA term is there because the crate carries a copy of libFuzzer's own C++
source and builds it. It is a permissive licence and the combination raises no
question against the AGPL-3.0 this repository carries, and the question is
narrower than for the other two anyway: **nothing in this crate is distributed**.
It is a test harness that runs on a developer's machine and on a runner, and it
is not linked into the binary a release would carry.

**The version requirement is exact.** `= 0.4.10` rather than a range, for the same
reason as `toml_parser`: a run whose engine changed under it is a different run,
and a finding attributed to a fuzz job should be reproducible from the version the
job used. `fuzz/Cargo.lock` is tracked beside it.

**What it brings with it.** `arbitrary`, and a build-time chain under `cc` that
compiles the vendored C++:

```
$ cd fuzz && cargo +nightly tree -p einschlag-fuzz --charset ascii
einschlag-fuzz v0.0.0 (fuzz)
|-- einschlag v0.0.0 (crates\einschlag)
|   |-- libm v0.2.16
|   `-- toml_parser v1.1.3+spec-1.1.0
|       `-- winnow v1.0.4
`-- libfuzzer-sys v0.4.10
    `-- arbitrary v1.4.2
    [build-dependencies]
    `-- cc v1.4.2
        |-- find-msvc-tools v0.1.10
        |-- jobserver v0.1.35
        |   `-- getrandom v0.4.3
        |       `-- cfg-if v1.0.4
        `-- shlex v2.0.1
```

**Those packages are not judged by the socket check**, and that is the residual
this entry is here to state rather than to soften.
`crates/einschlag/tests/nothing_goes_out.rs` reads the root `Cargo.lock`, which
this graph is not in, so `getrandom` and the rest of the build chain above are
outside what refuses a network stack in this tree. What holds instead is that
none of it reaches the tool: the fuzz crate is not a workspace member, nothing in
the workspace depends on it, and a release carries none of it. Whether that check
should reach a second lock file is not decided here and no issue holds it today.

## What the check does not do

**It reads the manifests, not the resolved graph.** `Cargo.lock` is tracked and
`--locked` is on the build, test and lint jobs, so a resolver that pulled a
different version can no longer do it quietly; what this check reads is still the
manifests. It answers which crates this project asks for, not which ones it got,
and a package arriving only through the lock is invisible to it.

**It counts nothing transitive.** See the ceiling section above.

**It does not read the licence.** The licence in an entry is written by a person
and compared against nothing. A crate that changes its licence between versions
leaves this file saying the old one. No mechanism is owed by #27 for that and
none exists.

**It does not judge whether an entry's reasoning is any good.** Whether the cost
of doing without a crate was honestly stated is what a reader is for.
