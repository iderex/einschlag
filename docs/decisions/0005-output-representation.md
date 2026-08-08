# 0005. The sample population is the primary artefact and the summaries the tool will not produce are listed by name

## Status

Accepted.

## Date

2026-08-08.

## The question

A distribution over positions can be handed over as the samples themselves, as a
density on a grid, as a set of nested regions at stated levels, or as a summary.
The choice decides what a reader can do with the result and, more importantly
here, what a reader can mistake it for.

The pull in this application runs one way. A court wants a position. A report
wants a sentence. An operator under deadline wants the picture that is easiest to
explain. Each of them is asking for a narrower answer than the evidence supports,
and each of them will get one from any artefact that contains a single
coordinate, because the coordinate is what gets quoted and the region around it
is what gets left behind.

So the question has two halves and they are not the same. What the tool hands
over, and what the tool refuses to compute at all. The second half has to be
settled here rather than in the first pull request that wants a mean, because by
then there will be a caller who needs one and the argument will be about that
caller rather than about the property.

`../survey/challenges.md` supplies the reason the second half is not
over-cautious. The FBI microscopic hair comparison review found examiners
assigning a statistical weight to a positive association where the number did not
come from data that could produce it. A tool whose whole output is numbers
attached to positions is closer to that failure than a person is, and it will be
more persuasive because it came out of a computer.

## The options considered

**The samples themselves as the primary artefact.** Honest, complete, and the
thing the run actually computes under `0004-uncertainty-model.md`. Every other
form can be derived from it and it cannot be derived from any of them. Cost:
large files, awkward to read, and, stated plainly rather than glossed, a reader
holding the samples can compute a mean position in one line. The refusal to
produce a point does not become a refusal to make one possible.

**A density over a voxel grid.** Readable, drawable, and the form a reader who
has seen a heat map already understands. Cost: a resolution has to be chosen, the
choice hides everything below it, and the choice is an assumption that narrowed
the answer without appearing in the list of assumptions. Two runs at two
resolutions give two different pictures of one result, and neither is wrong.

**Nested regions at stated probability levels.** What a reader actually wants and
what a report can carry. Cost: a region at a level is not one thing. Two
defensible constructions at the same level can differ substantially, so the
construction has to travel with the region or the number is a claim the reader
cannot check.

**A summary: a position with a radius, or a best line.** The most usable and the
one this project exists to refuse. Cost: it is the failure mode, and the cost
falls on the person the reconstruction is about.

**Several of these together, with none named primary.** Tempting because the
forms are not exclusive. Cost: a reader takes whichever one their question
suits, and there is no answer to the question of which one is the result when two
of them disagree, which they will whenever a construction or a resolution
changes.

## The option taken

Three artefacts, in a fixed relationship, and one of them is primary.

### The sample population, primary

The population of positions the run drew, written in full, with the seed and the
draw count that produced it.

**What a reader may correctly conclude from it.** That these are the positions
consistent with the declared constraints under the assumptions the manifest
names, in the proportions the model gives them. Nothing about how often such a
population contains a true origin, which is calibration and is milestone 7.

It is primary because everything else in the output is derived from it and it is
derived from nothing else. A region can be rebuilt from the samples under a
different construction years later; samples cannot be rebuilt from a region. It
is also the only artefact against which a claim in a report can be rechecked
without re-running the tool.

### Nested regions at stated levels, derived

Regions at the levels the run configuration names, each carrying its level and
the named construction that produced it, as issue #44 requires.

**What a reader may correctly conclude from one.** That the construction named,
applied to that sample population at that level, produced that region. Not that
the true position lies inside it with that probability. The step from the first
statement to the second is calibration, and until milestone 7 produces a figure
the tool does not licence it. `0010-honesty-rule.md` already states this as the
bound on its property 2.

They are derived and never independent. A region in the artefact is always
accompanied by the samples it was built from, and a run cannot emit regions with
the samples suppressed. A region travelling alone is a number whose construction
the reader has to take on trust.

### The manifest, derived

What the run was: the tool version and commit, the seed, the platform, a digest
of the input, the assumptions applied with what each one removed, the warnings
raised, and the validation status of the version that produced it.

**What a reader may correctly conclude from it.** What was run and on what. It is
the artefact that makes the other two re-runnable, and under
`0009-determinism.md` it carries everything a re-run needs.

The warning fixed by `0003-model-boundary.md` is a manifest field: where any part
of the region lies further than 20 m from the hole that constrained it, the
manifest says so and names the distance reached. A run inside the threshold says
so positively rather than omitting the field, in the shape `0008-priors.md`
already fixes for a run that applied no prior.

### What the report and the visual are

Renderings. `docs/OUTPUT.md` will say it and this record fixes it: the report in
issue #47 and the visual in issue #48 carry no information that is not in the
three artefacts above, and neither is a place where a new quantity first appears.
A number in a report that is in no artefact is a number nobody can recheck.

## What the tool will not produce, by name

Each of these is computable and each is refused. `0010-honesty-rule.md` property
1 is the mechanism, issue #46 lands the test, and this section is the list that
test carries.

- **A most likely position, a modal position, or a maximum a posteriori
  estimate.** It is the single number a reader will quote as the answer, and
  under `0004-uncertainty-model.md` it is also the least stable one: a mode
  estimated from samples moves with the seed and with the smoothing used to find
  it, so it is both the most quoted and the least reproducible thing the tool
  could emit.
- **A mean position or a centroid, as a result.** The mean of a region shaped
  like a courtyard is a point in the middle of the courtyard, which may be a
  place the geometry excludes. `0010-honesty-rule.md` already permits a centroid
  computed internally for drawing, named for what it is and unreachable as a
  result, and this record does not widen that.
- **A median position, or per-axis medians.** Three marginal medians are not a
  position. The point they define need not be in the region at all, and it looks
  more like a measurement than a mean does.
- **A single best line, a best trajectory, or a preferred direction per hole.**
  The same failure one step upstream. `../survey/tools.md` records that a line is
  what the surveyed practice already returns, and returning one here would make
  this tool a slower way to get the same thing.
- **A point of origin.** Refused as a name as well as a value. It is the term the
  field uses, so an output field carrying it would be read as the answer whatever
  it contained.
- **A distance or a bearing from a named point to the reconstructed position.**
  It is a point estimate with the point hidden inside the arithmetic, and it is
  the shape a request from a report generator will arrive in.
- **A probability that a named place, a named person or a named single
  coordinate was the origin.** This is the FBI hair review's second category
  arriving in this project's vocabulary, and it is refused for that reason rather
  than for a general dislike of numbers.

### The one summary the tool does produce, and why it is not in that list

For a region the operator declares in the input file, the run reports the number
of samples that fell inside it, that number as a proportion of the samples that
reached the test, and an interval on the proportion from the finite draw count.

It is produced because the question behind it is the operator's real question and
refusing it does not remove it. Somebody asks whether the shot could have come
from a particular window. An operator refused an answer will read the picture and
answer it worse.

It is not a point estimate because the operator supplied the region, the tool did
not choose it, and the number reported is a property of the sample population
rather than a coordinate. It is stated as a proportion of the samples this run
drew under the assumptions the manifest names, never as the probability that the
shot came from that place, and the interval on it is reported because a
proportion from a finite draw count has a spread and quoting it to a fraction of
a per cent is the overstatement this project is against.

It is refused where the declared region is small enough that the sample count
cannot support the answer, which is the same refusal `0009-determinism.md`
already fixes for a region built from too few samples for the level requested.

## The reasons

The samples were made primary over the regions because of what survives time. A
region embeds a construction, and a construction is a decision this project may
get wrong and will want to change. If the region were primary, every stored
result would carry the construction of the day it was made and no later
improvement could be applied to it. With the samples primary, an old run can be
re-read under a better construction, and the two answers can be compared, which
is the property an evidential artefact needs most.

The voxel density was rejected as primary rather than rejected outright, and it
is not in the artefact set at all in the first version. A resolution is an
assumption that narrows the answer, `0010-honesty-rule.md` property 3 requires
every such assumption to be named in the output, and a grid resolution named in a
list of assumptions is a strange thing for a reader to weigh. The drawn output in
issue #48 may rasterise for display, and a rasterisation for display is not an
artefact.

Naming one artefact primary was preferred to publishing several as equals because
of the disagreement case. Regions and samples will disagree the day a
construction changes, and a reader needs to know which one is the result. Without
that, the answer to "what did the tool say" depends on which file was opened.

The refusal list is stated by name rather than as a principle because a principle
does not fail a test. Issue #46 needs a list to enumerate against, and a list is
also the thing somebody proposing an addition has to argue with.

The containment query was admitted, against the general direction of this record,
because the alternative is worse and because the two properties that make it
dangerous are both absent here: the tool does not choose the region, and the
number is reported about the sample population rather than about the world.

## What this costs

**The primary artefact is large and nobody will read it directly.** A sample
population is not a document. Every human reader will read the regions, the
report or the picture, so the artefact this record calls primary is the one with
the fewest readers. That is accepted because its job is to be checkable rather
than to be read.

**Shipping the samples makes a point estimate one line of arithmetic away.** The
refusal is a property of the tool and not of the data, and this record does not
pretend otherwise. `0010-honesty-rule.md` property 1 constrains what the tool
returns; it cannot constrain what a reader computes from what the tool returned.
The alternative, withholding the samples, would make the result uncheckable in
order to make it harder to misuse, and that trade is the wrong way round for this
project.

**The containment number will be quoted without its interval.** It is a
percentage, percentages travel alone, and this record admits that the interval
beside it is the part most likely to be dropped between the artefact and the
courtroom. What is bought is that the tool cannot be the place the interval was
dropped.

**Refusing a point costs the users who need one.** Somebody comparing this tool
against one that reports a position, somebody with a workflow downstream that
takes coordinates, somebody drawing a diagram in software that wants a marker.
`0010-honesty-rule.md` already states this cost against its property 1 and it is
not softened by being restated here.

**Two artefacts that must travel together can be separated by a file system.**
Nothing stops somebody mailing the regions without the samples. The rule that a
region is never emitted alone is a rule about what the tool writes, not about
what survives copying.

## What would falsify this

Regions and samples disagreeing in a way this record's ordering handles badly. If
it turns out that the sample population is routinely too large to store alongside
the runs that reference it, then primary and stored are different things and this
record is superseded by one that says which is which.

The containment query being used as a point estimate in practice. If reports
start declaring a single small operator-declared region with a high proportion
and presenting that as the position, the query has become the thing it was
admitted as not being, and it is withdrawn or constrained to a minimum region
size.

A construction under which a region cannot be rebuilt from stored samples, for
instance one needing an intermediate quantity the run discarded. That would break
the reason samples were made primary, and the response is to store the
intermediate rather than to promote the region.

The refusal list turning out to be unenforceable in the language chosen by
`0002-language-and-toolchain.md`. Issue #46 already carries the instruction to
say so in `docs/OUTPUT.md` and open the issue for the mechanism rather than
record an unenforced rule as enforced, and this record's list would then be a
list a person checks.

## Reconciliation with the failure modes in `../survey/challenges.md`

That file ends with eight failure modes phrased as things this tool must not do,
each saying whether the plan already prevents it. This section says what this
record does about each, and it does not upgrade any of that file's negatives.

**It must not emit a conclusion the method cannot support.** The refusal list
above is the concrete form of that for the output shape, and the primary artefact
is the population rather than a conclusion.

**It must not report a region as though its width were derived from the
measurement when it is a convention.** Addressed and not fully closed. Every
region carries its construction and its level, and this tool has no fixed-width
mode, so a width in its output is always propagated rather than conventional.
That file asks whether "this width is a convention" is expressible in the naming
of assumptions, and this record does not answer it, because the case does not
arise in the artefacts fixed here. `../PREMISE.md` is where the convention in the
surveyed practice is argued.

**It must not attach a probability to a position unless that probability came
from data that can produce it.** This is the failure mode the containment section
is written against. The number reported is a proportion of a sample population
with an interval from the draw count, and the sentence around it says what it is
a proportion of. That file records this as not prevented by any named mechanism
today, and it stays not prevented: this record fixes what is emitted, and issue
#46 lands the test.

**It must not cite its own operational history as support for a conclusion.** No
artefact fixed here has a field for a case count, a run count or a history. That
file records it as not applicable rather than prevented, and this record does not
convert that into a positive assurance.

**It must not state or imply a certainty it cannot support, in words.** Not
addressed here and not weakened. This record fixes artefacts and their fields;
`0010-honesty-rule.md` property 6 and issue #75 hold the text.

**It must not present as determined something the inputs leave undetermined.**
The manifest carries the assumptions and what each removed, and
`0004-uncertainty-model.md` already fixes that a quantity marked unknown is named
as unconstrained rather than as uncertain. The per-input half that file records as
unprevented stays unprevented.

**It must not be unable to say what its own error rate is.** The manifest carries
the validation status of the version that produced the run, which is the field
issue #52 fills once calibration figures exist. Today that field says the version
has no calibration behind it, and it says it in the artefact rather than only in
a document.

**It must not let a validation status go unstated where the output is read.** The
field above exists for that reason. What it says is entry 6 of the maintainer
decisions in issue #1, which is open. Under any of the options in that entry the
field is present and populated; what changes is its wording, so that answer
supersedes the wording without disturbing the artefact shape fixed here.

## Evidence

The failure modes reconciled above are quoted from `../survey/challenges.md`,
which records the PCAST report and the Department of Justice review as read from
their PDFs and the FBI hair review as read through PCAST's citation of it rather
than from the press release.

That file, `../survey/tools.md`, `0002-language-and-toolchain.md`,
`0004-uncertainty-model.md`, `0008-priors.md` and `0010-honesty-rule.md` are not
on the default branch at the commit this record was written on. They are on the
branch of the open pull request that lands issues #3, #5, #7, #11, #13, #15, #17
and #19, and they were read there at
`8134ddc19a7a957c9e94e5e959a30377089fe502`:

    git show 8134ddc19a7a957c9e94e5e959a30377089fe502:docs/survey/challenges.md

The links resolve once that pull request lands, and until it does they do not.

No number in this record was measured. The artefacts it fixes do not exist. No
source file and no build manifest is tracked at the commit this record was
written on:

    $ git ls-files "*.rs" "*.toml" | wc -l
    0

Issue #21 is where a project first exists. Every statement above about what the
tool emits is a statement about what it will emit, and issue #46 is where the
first of them is refused by a test rather than by this document.
