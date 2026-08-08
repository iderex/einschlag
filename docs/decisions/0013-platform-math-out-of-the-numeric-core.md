# 0013. The numeric core evaluates every transcendental through one pinned implementation, and a mechanism refuses the platform's

## Status

Accepted.

## Date

2026-08-08.

## The question

`0009-determinism.md` promises that the same input file, the same seed and the
same build of the tool produce byte-identical output, and it bounds that promise
by the build and by the platform. It is the promise a second party relies on when
they re-run a reconstruction they disagree with, and that re-run is the strongest
defence this project has against a modified build.

The standard library of the language taken in `0002-language-and-toolchain.md`
does not support the unbounded half of it. The documentation of `f64` attaches an
"Unspecified precision" note to its transcendental methods:

> The precision of this function is non-deterministic. This means it varies by
> platform, Rust version, and can even differ within the same execution from one
> invocation to the next.

Read at `https://doc.rust-lang.org/std/primitive.f64.html` on 2026-08-08. It is
attached to, among others, `sin`, `cos`, `tan`, `asin`, `acos`, `atan`, `atan2`,
`exp`, `ln`, `log2`, `log10`, `powf`, `powi`, `hypot`, `cbrt`, `to_degrees` and
`to_radians`. Several of those are unavoidable here: the ellipse relation is an
arcsine, a direction in a scene frame is trigonometry, and every angle in the
input format arrives in degrees.

Two of the three clauses are already inside 0009's stated bound. Platform
variation is named there, and a change of build is named there. The third,
variation "within the same execution from one invocation to the next", is not
bounded by anything 0009 says: it would break byte-identical output on one
machine, with one build and one seed, which is the promise itself.

This record decides what the core calls instead, and whether anything refuses the
other route. It does not touch 0009. Records are added and superseded rather than
edited, and the measurement below did not force a supersession.

## The options considered

**Leave it. Call the standard library and rely on the note being a licence the
documentation reserves rather than a thing that happens.** Cost: the promise in
0009 rests on an assumption nobody measured, and the failure it admits is
invisible. A reconstruction that differs in the last place between two runs on one
machine does not look wrong, it looks like two reconstructions. The measurement
below found no instance of the clause biting, which is a reason to think the risk
is small and not a reason to depend on it: a negative result over one platform and
one build says nothing about the next one, and the documentation is the authority
on what is permitted rather than the observation.

**Call the standard library and write the promise down as bounded by it.** That
is, supersede 0009 with a record admitting the output may differ between two runs
on one machine. Cost: it gives up the property the tool exists to offer. A
reconstruction that cannot be reproduced exactly cannot be checked by the other
side, and `../survey/field-practice.md` reads the Minnesota Protocol defining
reliability as the stability of a result across observers, places and times.

**Take a math implementation that does not depend on the platform's C library,
at a pinned version, and route every call in the core through one module.** Cost:
a direct dependency, a version to keep an eye on, and a performance cost against
the platform's, which is often an intrinsic and is not measured here.

**Write the transcendental functions in this repository.** Cost: this becomes a
numerical-methods project. Correctly rounded elementary functions are a field with
its own literature, and a home-written arcsine that is wrong in the fourth place
is far worse than a borrowed one that differs in the sixteenth. Rejected on the
same standpoint reasoning that took a small surface elsewhere: this project is not
equipped to own that, and it would have to own it forever.

**Software floating point throughout, so that every operation is defined by this
project.** Already considered and rejected in 0009, where it is named as the cost
of promising determinism across all platforms. Nothing here changes that
judgement, and the option is repeated so that this record is not read as having
found a cheaper version of it.

## The option taken

The third. Every transcendental the numeric core evaluates goes through
`crates/einschlag/src/math.rs`, which calls the `libm` crate at an exact pinned
version, and `crates/einschlag/tests/platform_math_stays_out_of_the_core.rs`
refuses a call to the platform's version from anywhere under the core's `src`.

What that means precisely, so a reader can tell whether a piece of code complies:

- A function whose precision the standard library leaves unspecified is not
  called from the core by any spelling: not as a method on a float, and not as a
  qualified call naming the float type. The check reads both forms.
- The one route is the module named above. A function it does not carry yet is
  added there and nowhere else.
- The version requirement is exact rather than a range. `Cargo.lock` is untracked
  until #26, so a range would let a fresh clone resolve a newer release, and a
  newer release may return a different value in the last place. Once the lock is
  tracked, the exact requirement can be relaxed and the lock holds the version
  instead. That relaxation belongs to #26 and is not made here.
- Square root, fused multiply add and the ordinary arithmetic operators are not
  on the refused list and are called directly, because IEEE 754 specifies them to
  be correctly rounded and the platform has no latitude in them.
- Degrees to radians and back are supplied by the module as one multiplication by
  one constant each. The standard library's versions carry the same reservation
  as the transcendentals while being that same single multiplication, and a
  multiplication written out is exact.
- Integer powers are supplied by the module as repeated multiplication rather
  than by the borrowed library, which has no integer-exponent form. Every step is
  an IEEE multiplication, so the result depends on that function's shape and on
  nothing underneath it.

`libm` is the implementation taken. It describes itself as "libm in pure Rust",
its repository is `rust-lang/compiler-builtins`, and it is MIT licensed, which
combines into a work distributed under the AGPL-3.0 that entry 1 of #1 decided.
`../DEPENDENCIES.md` carries the entry with the commands behind those facts.

## The reasons

The clause that forced this is the third one, and it is the only one that reaches
inside 0009's promise rather than inside its bound. A project that promises
byte-identical output on one machine and then calls a function documented as
permitted to move between invocations has written a promise it does not control.
That is true whether or not the licence is ever exercised, and it is the same
distinction this repository draws everywhere else between what was measured and
what was assumed.

The measurement did not find the clause biting, and that did not change the
decision. It changed what this record can claim: the risk is not observed on this
platform with this build, and the reason the promise holds is now written down
rather than assumed, which is what #77 asked for either way. A negative result
across one platform is not a property.

Pinning was chosen over merely depending because the difference between two
implementations is measurable and is exactly the size that matters. The platform
and `libm` disagree in the last place on a good fraction of the sampled points,
which is the evidence in the section below. If a clone can resolve a different
release of the borrowed library, then two clones of one commit produce different
answers, and that is the same defect arriving through the dependency instead of
through the platform.

The mechanism is a check rather than a convention because the mistake is one
character. The platform's arcsine is a method on the number, reachable by typing a
dot, and the wrong call compiles, runs, and returns a value that is right to
within a last place. There is nothing for a reviewer to see. This is the shape of
defect a guard exists for.

It is landed now, before the geometry, because it is a birth requirement. A guard
introduced after the calls exist is a guard whose first act is to refuse working
code, and it is then weakened to let that code through.

## What this costs

A direct dependency, and it is the project's first. `../DEPENDENCIES.md` states
the ceiling this counts against and what the entry has to say. It is code
somebody else wrote that is now part of every answer this tool gives, and the
person defending a reconstruction has one more thing to be able to explain.

An exact version requirement, which is the strictest form and will conflict with
another crate that wants a different release of the same library. Nothing else in
the tree depends on it today, and the day something does is the day this line is
argued again.

Performance against the platform's implementation, which on many targets is an
intrinsic or a vendor-tuned routine. This has not been measured. No figure is
quoted here and none should be inferred from the choice.

A second place a reader has to look. Code in the core says `math::asin(ratio)`
rather than the form every Rust reader already knows, and somebody arriving from
outside will find that unfamiliar before they find the reason.

The guard costs a list that has to keep up with the standard library. The refused
names were read from the documentation on one day, and a function that gains the
note later is a function this check does not know about. Nothing detects that.

## What would falsify this

A measurement showing the platform's functions returning different bits for the
same argument inside one execution. That would not falsify the decision, it would
confirm the reason for it, and it is named here so that a reader does not mistake
the negative result below for the argument.

`libm` turning out to vary across its own releases in a way the exact pin does
not hold, or across platforms for the same release. The pin holds the version;
it does not make the implementation platform-independent by itself, and this
record asserts only what the pin holds. That the borrowed library gives identical
results on two architectures has **not been measured here** and is not claimed.

A measured cross-platform difference large enough to move a reported region at a
stated level. That falsifies 0009's bound rather than this record, and 0009
already names it.

The disagreement between the two implementations turning out to be zero
everywhere, which would mean the choice is not observable and the pin buys
nothing. `the_platform_and_the_pinned_implementation_do_not_agree_bit_for_bit` in
`crates/einschlag/tests/transcendental_stability.rs` fails if that becomes true,
which is the one place this record checks its own premise rather than asserting
it.

A future where the standard library specifies the precision of these functions,
at which point the reason for the dependency is gone and this record is
superseded by one that says so.

## Evidence

The measurement #77 asked for, run at the commit this record lands on:

```
$ cargo test --offline --test transcendental_stability -- --nocapture
within one execution: 0 of 1792000 repeated invocations returned different bits
```

Sixty-four arguments, fourteen functions, two thousand repetitions each, every
argument passed through an optimisation barrier so that the compiler could not
answer once and reuse the answer. **No instance of the within-execution clause was
observed.** That is a negative result over one platform, one build and one
compiler release, and it establishes nothing about any other. The command re-runs
it.

Across platforms: **not measured.** Only one target was available, and no figure
for a second one is quoted or estimated. This is the half of #77's measurement
that this record does not carry.

That the choice of implementation is observable in the answer, from the same
command:

```
sin    differs at  1 of 64 grid points, largest difference 1 in the last place
cos    differs at  1 of 64 grid points, largest difference 1 in the last place
tan    differs at  5 of 64 grid points, largest difference -1 in the last place
asin   differs at  1 of 64 grid points, largest difference 1 in the last place
acos   differs at  0 of 64 grid points, largest difference 0 in the last place
atan   differs at  0 of 64 grid points, largest difference 0 in the last place
atan2  differs at  8 of 64 grid points, largest difference -1 in the last place
exp    differs at  4 of 64 grid points, largest difference 1 in the last place
ln     differs at  5 of 64 grid points, largest difference 1 in the last place
powf   differs at 12 of 64 grid points, largest difference 1 in the last place
hypot  differs at  5 of 64 grid points, largest difference 1 in the last place
cbrt   differs at 14 of 64 grid points, largest difference 1 in the last place
```

Every disagreement found was one representable value wide. That is small, and it
is not nothing: two builds of this tool that differ only in which implementation
they called would produce outputs that are not byte-identical, and byte-identical
is what 0009 promises.

The identity, version and licence of the borrowed library:

```
$ cargo metadata --format-version 1 | python -c "import json,sys; d=json.load(sys.stdin); print([(p['name'], p['version'], p['license'], p['repository']) for p in d['packages'] if p['name']=='libm'])"
[('libm', '0.2.16', 'MIT', 'https://github.com/rust-lang/compiler-builtins')]
```

The quotation from the standard library documentation, and the list of methods it
is attached to, were read from `https://doc.rust-lang.org/std/primitive.f64.html`
on 2026-08-08. That is a reading of a web page rather than a command run against
this tree, and it is cited as one.
