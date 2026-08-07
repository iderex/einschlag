# 0009. The same input and seed give the same output on the same build, and every run writes a manifest that says what produced it

## Status

Accepted.

## Date

2026-08-07.

## The question

If the uncertainty model samples, the output depends on a random seed, and a
result that cannot be reproduced exactly is a result that cannot be checked by the
other side.

That is not a preference of this project. `../survey/field-practice.md` reads the
Minnesota Protocol's own glossary, which defines reliability as the stability of a
result "when the test is undertaken by different observers in different places at
different times", and reviewability as the aim that another practitioner at
another time can independently reach their own conclusions. A reconstruction that
gives a different answer on a different machine fails the first definition on its
face.

Three things are decided here. What the reproducibility promise is and what bounds
it. What a run records about itself so that an output found on its own can be
traced back to the code and the input that produced it. And what happens when the
sample count is too low for the regions being reported to be stable.

## The options considered

**No promise. Seed from the system and let the numbers land where they land.**
Costs: the output cannot be checked by anyone. Rejected.

**Determinism promised across all platforms and all versions.** The strongest
claim and the one a reader most wants. Cost: it is not deliverable. Floating-point
results depend on the order of operations, on library implementations of
transcendental functions, and on compiler and hardware choices this project does
not control. Promising it means either shipping a claim that quietly fails, or
building a software floating-point layer and paying for it in every run.

**Determinism promised for the same input, seed and build, with the residual
dependence stated rather than denied, and reduced where reduction is cheap.**
Cost: the promise has a bound a reader has to understand, and a result reproduced
on a different platform may differ in the last places. The promise is weaker and
it is true.

**Determinism achieved by making the model non-sampling, so no seed exists.**
Cost: this is a decision about the uncertainty model rather than about
reproducibility, it is not this record's to make, and it does not remove the
problem in any case, since a deterministic numerical method still carries the
platform dependence above.

## The option taken

The same input file, the same seed and the same build of the tool produce
byte-identical output. This is the promise, and the tool states it in those terms.

The promise is bounded by the build. A different version of the tool may produce a
different answer, and that is a change worth making when the model improves, so
the manifest records the version and the commit and the release notes say when a
reported region could move.

The promise is bounded by the platform to the extent that floating-point
behaviour is not fully specified. What is done about that, rather than claimed
about it:

- every sampling draw comes from one explicitly seeded generator whose algorithm
  is fixed by this project and not taken from a platform default, so the sequence
  of random numbers is identical everywhere;
- all reductions over samples are performed in a fixed order that does not depend
  on iteration order, thread count or scheduling;
- no parallelism is allowed to change the order in which results are combined;
- the manifest records the platform, so that two outputs that differ can be told
  apart from two outputs that should not have differed.

What is not done: no software floating-point layer, and no claim that the last
digit is identical across architectures. Where a difference is observed across
platforms it is a fact to be measured and reported, not a bug to be denied.

The seed is written by the operator or generated and recorded. It is never left
implicit. A run that generated its own seed records the generated value, so the
run can be repeated exactly.

## The reasons

The bounded promise was taken because it is the strongest promise that can be kept
and because an unkept promise here is worse than a weaker one. The audience for
this tool includes people whose work will be attacked, and a reproducibility claim
that fails under cross-examination costs more than a claim that was modest and
held.

Fixing the generator rather than using the platform's is what makes the promise
worth anything in practice. Most of the difference between two machines is the
random sequence, not the arithmetic, and that part is entirely within this
project's control.

Fixing the reduction order is the other half. A parallel sum over samples is the
most likely way for two runs on one machine to differ, and it differs for a reason
that has nothing to do with the physics.

## What this costs

Performance. A fixed reduction order forecloses some parallel implementations, and
a project-owned generator will be slower than a platform intrinsic. Neither cost
has been measured; when it is, the number goes in this record's successor rather
than being estimated here.

A reader has to understand a bounded promise rather than an absolute one, and the
bound has to appear in the output rather than only in this file, or it will not be
read.

## What would falsify this

Two runs with the same input, seed and build producing different output on one
machine. That is the promise failing rather than the bound being reached, and it
is a defect.

A measured cross-platform difference large enough to move a reported region at a
stated level. The bound above treats platform dependence as affecting the last
places; if it affects the answer, this record is wrong and something stronger is
needed, up to and including the software floating-point layer rejected above.

The manifest turning out not to be enough to reproduce a run. If a run cannot be
repeated from the manifest and the input, the field list below is incomplete and
the record is superseded with the missing field named.

## The run manifest

Every run writes a manifest, and it travels with the output rather than being kept
beside it. An artefact found on its own has to be traceable to the code and the
input that produced it, so the manifest is inside the output artefact and not a
separate file that can be lost.

The fields:

- the tool version, and the commit it was built from;
- the seed, whether supplied or generated, and which of those it was;
- the number of samples drawn;
- the input file name as given, and a cryptographic digest of its bytes;
- the digest algorithm, named rather than assumed;
- the format version declared by the input;
- the identifiers of the decision records in force for this build;
- every default value that was applied because the input did not state it;
- the confidence levels at which regions were reported;
- the platform: operating system, architecture, and the toolchain version the
  build was made with;
- the start time of the run, in UTC, and its duration;
- every assumption that narrowed the answer, which is the subject of its own issue
  and is named here because the manifest is where it lives.

The digest is of the input bytes, not of a parsed and re-serialised form. A
digest that changes when the tool's formatter changes is a digest that proves
nothing about the evidence.

The time fields are the one part of the manifest that is not reproducible, and
they are deliberately outside the byte-identity promise: the promise is over the
reconstruction result, and a manifest comparison for reproduction purposes
excludes the timestamps. The tool says which fields those are rather than leaving
a reader to discover that two identical runs differ.

## When the sample count is too low

Reporting a region from too few samples is a precision claim the run cannot
support, and rounding it off is the failure this project exists against.

The tool computes, for each region it was asked to report and at each level, the
sample count needed for that region to be stable to a stated tolerance. Where the
run has fewer samples than that, the region is not reported. The tool says which
region at which level was refused, how many samples the run had, and how many it
would need.

This is a refusal and not a warning. A warning printed beside a number is read as
a number, and the region would be quoted without it.

The run does not silently increase its own sample count to meet the requirement,
because a run that quietly does more work than it was asked to is a run whose cost
cannot be predicted. It reports what it would need and stops, and the operator
re-runs with that count.

What the tolerance is, and how the required count is computed from it, is not
fixed here. It depends on how regions are constructed, which is not this record's
subject. What is fixed here is that the check exists, that it is evaluated per
region and per level rather than once for the run, and that its outcome is a
refusal.

## Evidence

The reliability and reviewability definitions are quoted in
`../survey/field-practice.md` from the glossary of the Minnesota Protocol on the
Investigation of Potentially Unlawful Death (2016), OHCHR, HR/PUB/17/4.

The requirement that automated processes be explainable in court, and that
investigators record the tools and software used, is quoted in the same file from
the Berkeley Protocol on Digital Open Source Investigations, OHCHR and the Human
Rights Center, HR/PUB/20/2, paragraph 25. The manifest is what this project offers
against that requirement.

No performance figure in this record was measured, and none is quoted.
