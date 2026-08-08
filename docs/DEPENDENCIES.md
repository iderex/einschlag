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

There are none.

The workspace has two crates and `einschlag-cli` depends on `einschlag` by path.
That is one part of this project depending on another part of it, not code
somebody else wrote, and the check does not count it. A path dependency pointing
outside the workspace would be counted, because that is somebody else's code
arriving by a different route.

Nothing else is used. The provenance the tool reports comes from `git` invoked as
a subprocess in a build script rather than from a crate, which
`docs/BUILD.md` records and the pull request that landed #29 argues.

This section is expected to stop being empty. `docs/decisions/0002-language-and-toolchain.md`
names `rand`, `nalgebra`, `statrs` and `libm` as candidates and decides none of
them, and the issues that would take one are #37 for sampling, #77 for the
platform mathematics and #43 for the output artefact.

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
