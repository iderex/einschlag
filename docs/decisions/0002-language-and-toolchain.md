# 0002. The tool is written in Rust, against a pinned stable toolchain

## Status

Accepted.

## Date

2026-08-08.

## The question

Nothing in milestone 3 can start before this, and everything after milestone 3
inherits it. The question is not which language is pleasant. It is which one lets
this project keep three promises it has already made elsewhere in this directory.

An operator has to be able to run the tool. `../survey/field-practice.md` records
that half the intended audience works on a borrowed or personal laptop, at a scene
that was not preserved, sometimes without a network, on hardware that may be
searched. A tool that needs a package manager, a runtime install or administrator
rights is a tool that does not run for them at the moment they need it.

A result has to be reproducible. `0009-determinism.md` promises that the same input
file, the same seed and the same build produce byte-identical output, and states
what bounds that promise. A language whose dependency set moves under the same
source, or whose build cannot be pinned, cannot deliver it.

A claim has to be provable. This repository's rule is that an asserted fact carries
the command that produced it, and the tool's own honesty properties in #19 are
supposed to be refused by tests rather than promised in prose. A language in which
those properties cannot be expressed as something a machine refuses is a language
that fails the rule this project is built on.

## The options considered

**Rust.** A single compiled binary with no runtime to install, a committed
dependency lock file, a toolchain version that can be pinned in the tree, and a
test runner that needs no display. Cost: the numerical and statistical ecosystem is
thinner than Python's, so more of the mathematics is written and validated here.
Larger cost: the intended field reads Python, so a Rust codebase has a small pool of
people who can extend it, and that pool does not overlap much with forensic
scientists and open source investigators.

**Python.** The language the field actually reads, and the deepest numerical
ecosystem. Cost: it is the option that fails the first promise outright. Running it
means installing an interpreter and a dependency tree, and the audience described
above frequently cannot. Freezing it into a single file with a bundler is possible
and produces an artefact that is large, platform-specific, and harder to attest
than a compiled binary. Reproducibility is achievable with a lock file and is
weaker in practice because the numerical stack underneath is compiled code whose
build the lock does not fix.

**Go.** Single static binary, fast builds, a committed `go.sum`, a test runner that
needs no display, and a much larger contributor pool than Rust. Cost: the numerical
ecosystem is thinner than Rust's rather than richer, so the argument that pushed
against Rust pushes harder here. Its floating-point behaviour carries the same
platform question and its generics are younger, which matters for expressing the
result types `0011-degenerate-cases.md` requires the caller to be unable to
confuse.

**C or C++.** Maximum portability of the artefact and a complete numerical
ecosystem. Cost: no committed lock file in the ordinary case, a build that varies
by machine, and a memory-safety burden this project has no reason to take on. It
fails the reproducibility promise at the level of the build rather than at the level
of the arithmetic.

**A language with an installed runtime, Java or .NET.** Excellent numerics and
strong tooling. Cost: the same failure as Python on the first promise, in a
different shape. Self-contained deployment exists and produces a large artefact
with a runtime inside it.

## The option taken

**Rust, on the stable channel, against a toolchain version pinned in the tree.**

The minimum toolchain version is **1.97.0**, which is the version this decision was
written against. On the machine where it was written:

    $ rustc --version
    rustc 1.97.0 (2d8144b78 2026-07-07)
    $ cargo --version
    cargo 1.97.0 (c980f4866 2026-06-30)

That is a fact about one machine and not about the repository, because the
repository has no `Cargo.toml` yet. Landing the pin is #21.

**How the version is pinned.** Three things, and they are different mechanisms for
three different failures.

A `rust-toolchain.toml` at the root naming the exact stable version and the
components the build needs. This makes a fresh clone build with the same compiler
without anybody being told to, which is the failure a documented minimum version
does not prevent.

`rust-version` in `Cargo.toml`, so that a build with an older compiler fails with a
message naming the version rather than with an error inside the source.

A committed `Cargo.lock`, with the build and the test run passing `--locked`, so a
resolver that would silently pick a newer dependency fails instead. #26 carries that.

**The numeric core does not use the standard library's transcendental functions.**
This falls out of the reproducibility promise rather than out of taste, and the
reason is in the evidence section. #77 carries the mechanism.

## The reasons

**The single self-contained binary decided it.** Everything else in this record is
a comparison between second and third place. `../survey/field-practice.md`
establishes an audience that cannot install a runtime, sometimes cannot reach a
network, and works on hardware that may be searched. A compiled binary with no
runtime is not a convenience for them; it is the difference between the tool
existing and not existing. Python and the runtime languages fail there, and no
amount of ecosystem repairs it.

**Reproducibility chose Rust over Go, narrowly.** Both give a static binary and a
committed lock. Rust's lock covers the whole dependency graph including the
compiled parts, its version can be pinned in the tree so a fresh clone cannot drift,
and its type system expresses the two-shaped result in `0011-degenerate-cases.md` as
something a caller cannot read wrongly by forgetting, which that record requires by
name. Go would have been an acceptable answer and this record would not have been
embarrassing if it had said Go.

**The thin numerical ecosystem is smaller than it looks, for this project.** The
mathematics here is small: some linear algebra, sampling, and geometry.
`0004-uncertainty-model.md` is where the method is decided, and whichever way it
goes the operations are elementary. The crates that exist are enough to check the
shape of the answer against, and the parts this project writes are parts it has to
validate anyway, because the calibration work in milestone 7 measures the whole
pipeline rather than a library inside it. Read from `https://crates.io/api/v1/crates/`
on 2026-08-08: `rand` 0.10.2, `nalgebra` 0.35.0, `statrs` 0.19.0. Whether any of
them is taken is #27's question and is not decided here.

**The contributor argument was answered with an interface rather than with a
promise.** This is the point that argues hardest against Rust and it is not
dismissed. Forensic scientists and open source investigators read Python, and a
tool nobody in the field can extend has one contributor. The answer this project
takes is that the field should be able to build on the tool without writing in its
language: a documented input format, which `0007-input-format.md` already fixes as
text a person can check line by line; a documented output artefact, which #43 owes;
and a command line contract. Those three make the tool a component that a Python
script can drive and read, which is what most of the field actually wants to do
with it. A promise to add bindings later would be the weaker answer and is not made
here.

## What this costs

**The field cannot read the source.** The interface above lets the field use the
tool. It does not let a forensic scientist who suspects the geometry is wrong open
the file and check. That cost falls on exactly the user this project most wants
scrutiny from, and nothing in this record removes it. It is reduced only by the
documents in `docs/decisions/` being readable without the code, which is why they
are written the way they are.

**A smaller pool of people who can fix a bug.** This falls on the project and it is
worse than it looks for a one-committer repository, because it also shrinks the
pool of people who could take it over.

**More mathematics written here.** Each piece written here is a piece that has to
be validated here, and milestone 7 is already the largest unfinished thing in the
plan.

**A compile step between a change and a result.** Where a Python prototype would let
somebody test a geometric idea in a minute, this will not. That cost falls on
development speed and, more importantly, on the willingness of a domain expert to
try something.

**Cross-compiling for every platform an operator might have.** A single binary is
only an advantage once it exists for the machine in front of them. Producing and
attesting those artefacts is #69 and it is work this record creates.

## What would falsify this

An operator population that turns out to install a runtime without difficulty. If
the field-practice reading is wrong about that, the strongest reason in this record
goes, and Python's ecosystem and contributor pool would win on what is left.

The numerical work turning out not to be small. If the uncertainty model in
`0004-uncertainty-model.md` needs a component that exists only in the Python or R
ecosystem and reimplementing it here is a research project rather than a week, this
record is wrong and the boundary should move, most likely to a compiled core with a
different language at the edge.

The reproducibility promise failing inside the language. #77 is open on exactly
this. If the standard library's floating-point behaviour cannot be pinned down even
with a platform-independent math implementation, then Rust bought less than this
record credits it with, and the comparison against Go turns on nothing.

Nobody outside this project ever driving the tool through its documented interface.
That is the mitigation the contributor cost was answered with. If, a year after the
first release, the interface has no users, the mitigation did not work and the
choice should be re-argued rather than defended.

## Evidence

**The transcendental functions are not specified, and this is the finding that most
affects the choice.** The Rust documentation for `f64`, read at
`https://doc.rust-lang.org/std/primitive.f64.html`, carries this note on `sin`,
`cos`, `tan`, `asin`, `acos`, `atan`, `atan2`, `exp`, `ln`, `powf`, `powi`, `hypot`,
`to_degrees` and `to_radians`, among others:

> Unspecified precision
>
> The precision of this function is non-deterministic. This means it varies by
> platform, Rust version, and can even differ within the same execution from one
> invocation to the next.

Every one of those functions is unavoidable here: the ellipse relation is an
arcsine, a direction in a scene frame is trigonometry, and angles arrive in degrees.
The platform clause sits inside the bound `0009-determinism.md` already states. The
clause about one execution does not, and it is the promise itself rather than the
residual. Whether it is reachable for the calls this project makes **has not been
measured**, and saying so is the state rather than a hedge. #77 carries the
measurement and the mechanism.

The mitigation named there is the `libm` crate, described on crates.io as "libm in
pure Rust" with repository `rust-lang/compiler-builtins`, version 0.2.16 read from
`https://crates.io/api/v1/crates/libm` on 2026-08-08. It is named as the candidate,
not as the decision.

**The toolchain versions quoted above are from one machine**, not from the
repository, which carries no manifest yet. Every other number in this record is a
crates.io version read on the date given, and none of them is a measurement of this
project.

**Nothing here was benchmarked.** No compile time, no run time, no binary size and
no memory figure was measured for any option. Where this record prefers one option
over another it is on the three promises in the question, never on speed, and any
sentence that reads as a performance claim is not one.
