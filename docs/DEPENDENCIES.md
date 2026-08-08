# Direct dependencies

Every dependency is code somebody else wrote that becomes part of the answer this
tool gives. That answer may be challenged, and the person defending it has to be
able to say what each piece of borrowed code does and why it is there.

This file is the list, and it is checked. `dependency_budget.rs` in
`crates/einschlag/tests/` reads the manifests in the workspace and compares the
direct dependencies it finds against the entries below, in both directions: a
dependency with no entry fails, and an entry naming a dependency nothing uses
fails. A stale entry is how a file like this stops being read.

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

There is one.

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

**The version requirement is exact.** `= 0.2.16` rather than a range, because
`Cargo.lock` is untracked until #26 and a fresh clone would otherwise resolve
whatever is newest. A later release of this crate may return a different value in
the last place, which is the whole thing the record pins. Once the lock is
tracked, the exact requirement can be relaxed to a range and the lock can hold
the version instead.

**What it brings with it.** Nothing. It is the only package in the graph besides
this workspace's own two. The absolute path the first line printed is shortened
here to the part that is the same in every clone:

```
$ cargo tree -p einschlag --charset ascii
einschlag v0.0.0 (crates\einschlag)
`-- libm v0.2.16
```

The workspace has two crates and `einschlag-cli` depends on `einschlag` by path.
That is one part of this project depending on another part of it, not code
somebody else wrote, and the check does not count it. A path dependency pointing
outside the workspace would be counted, because that is somebody else's code
arriving by a different route.

The provenance the tool reports comes from `git` invoked as a subprocess in a
build script rather than from a crate, which `docs/BUILD.md` records and the pull
request that landed #29 argues.

`docs/decisions/0002-language-and-toolchain.md` names `rand`, `nalgebra`,
`statrs` and `libm` as candidates and decides none of them. #77 took the last of
those; the others are still undecided, and the issues that would take one are #37
for sampling and #43 for the output artefact.

## What the check does not do

**It reads the manifests, not the resolved graph.** `Cargo.lock` is untracked
until #26, and a resolver that pulled a different version of a named dependency
would not be visible here. The check answers which crates this project asks for,
not which ones it got.

**It counts nothing transitive.** See the ceiling section above.

**It does not read the licence.** The licence in an entry is written by a person
and compared against nothing. A crate that changes its licence between versions
leaves this file saying the old one. No mechanism is owed by #27 for that and
none exists.

**It does not judge whether an entry's reasoning is any good.** Whether the cost
of doing without a crate was honestly stated is what a reader is for.
