# 0010. The condition becomes six properties, each stated so a test can decide it, and none of them is enforced yet

## Status

Accepted.

## Date

2026-08-08.

## The question

The condition this project is held to is that the output stays honest when the
honest answer is wide. Every pressure in this application runs the other way. A
court wants a position, a report wants a sentence, an operator under deadline
wants the picture that is easiest to explain, and each of them is asking for the
same thing: a narrower answer than the evidence supports.

A condition that lives only in prose does not survive that. It is not defeated in
an argument; it is worn away one convenient change at a time, each of which is
reasonable on its own and none of which is the change that broke it.

So the question is what the condition looks like when it is written as something
a machine can decide. Not as a value, not as a review checklist, but as a set of
properties with a test each, where the test failing is the only thing standing
between a change and the mainline.

There is a second question underneath it, and it is the one this record spends
most of its length on. Which parts of the condition can be given a property at
all, and what is honestly said about the parts that cannot. A rule recorded as
enforced when nothing enforces it is worse than the same rule recorded as prose,
because the first one stops being watched.

## The options considered

**Leave the condition as prose in the contributing document and rely on review.**
Costs nothing and reads well. Cost: it is the state this record exists to change,
and the corpus this project's practice is derived from records exactly this
failure, repeatedly. Review catches what a reader thought to look for on the day.

**One large conformance test asserting the condition holds.** Appealing because
the condition feels like one thing. Cost: the condition is not one thing, and a
single test asserting six unrelated properties fails for six unrelated reasons
with one message. When it goes red, the person who reads it learns that something
about honesty broke.

**A property per failure mode, each with its own test, each failing for its own
reason.** Cost: more tests, and a list that has to be argued about when it grows.
Also the honest cost, which is that a list of properties reads as a guarantee when
it is a floor. It holds what somebody thought of.

**Properties enforced at the type level rather than by tests, so a violation does
not compile.** Strongest where it is available, and decision 0002 chose a language
that can carry some of it. Cost: not everything below is expressible that way, and
a mix of compile-time and test-time enforcement has to say which is which or the
reader assumes the stronger one everywhere.

## The option taken

Six properties. Each is stated below in a form a test can decide, each names the
test that will refuse a violation, and each names the issue that lands that test.

The list is a floor and not a guarantee. It holds the failure modes this project
has thought of, and a change that breaks the condition in a way nobody anticipated
passes all six.

### What is enforced today: nothing

Stated before the properties rather than after them, because a reader who stops
after the list would otherwise stop with the wrong impression.

**None of the six tests below exists.** There is no library, no command line and
no test suite in this repository at this commit:

```
$ git rev-parse HEAD
5fbc1c9ff9177d045b087a08fe693f53aef8ecc1
$ git ls-files | wc -l
29
$ git ls-files "*.rs" "*.toml" | wc -l
0
```

Twenty-nine tracked files, none of them source and none of them a build manifest.
Issue #21 is where a project first exists and issue #23 is where a test suite
first runs. Until each property's own issue lands its test, every property below
is prose, and this record is a plan rather than a mechanism.

That sentence stays in this record after the tests land. It gains a line saying
which of them are done, with the command that shows it; it does not get replaced
by a statement that the condition is enforced.

### Property 1. No call returns a single position or a single line

**The property.** No function in the library's public interface, and no
subcommand or output mode of the command line, returns a single position, a single
line, a single trajectory or a single best estimate as a reconstruction result.
Not deprecated, not discouraged, not available behind a flag. Absent.

**What a test decides.** A test enumerates the public interface of the library and
of the command line and fails when a call appears whose result type is a single
position or a single line. It carries the list of forbidden result shapes and the
reason for each, in the test itself, so that somebody who trips it reads why
before deciding what to do about it.

**Where a centroid is legitimately needed.** Drawing a region requires arithmetic
over it, and some of that arithmetic produces points. Those are internal, they are
named for what they are, and none of them is reachable from the public interface
as a result. The artefact format has no field a reader could mistake for a point
estimate.

**The mechanism, and who owes it.** Issue #46. That issue also requires the test
to be shown biting, by adding such a call on a temporary commit, quoting the
failing output, and removing it.

**The bound.** The test enumerates shapes it was told about. A call returning a
region of zero extent, or a region and a separate field holding a preferred point,
satisfies this property as stated and defeats it in substance. Property 6's bound
paragraph applies here too: the enumeration holds what somebody thought of.

### Property 2. Every emitted region carries the level it was constructed at

**The property.** No code path emits a region without an attached probability
level and the named construction that produced it. A region without a level is not
a weaker result; it is a number a reader will attach their own level to.

**What a test decides.** Two things, and they are different. That a region value
cannot be constructed without a level, which decision 0002's language can carry in
the type rather than in a test. And that the artefact writer emits the level for
every region it writes, which is a test over a written artefact.

**The mechanism, and who owes it.** Issue #44, which also requires the nesting
assertion, that a region at a higher level contains the region at every lower
level, and the refusal of a region built from too few samples for the level
requested, which decision 0009 already fixes.

**The bound.** A level that is attached but wrong is not caught here. Whether the
number means what it says is calibration, which is milestone 7, and issue #50 is
where a stated level is checked against how often it actually contains the truth.

### Property 3. Every assumption that narrowed the answer is named in the output

**The property.** Every prior applied, every default used because the operator did
not state a value, every declaration that holes belong to one trajectory, and every
material row consulted appears in the artefact by name. Where an assumption
excluded part of the space, the artefact says how much it excluded.

Decision 0008 fixes what a prior entry carries and requires the proportion of
samples it rejected. This property is the general form of that requirement, over
every assumption rather than only over priors.

**What a test decides.** A run with a prior applied produces an artefact naming
it. The same run without the prior produces an artefact that states no prior was
applied, rather than omitting the field. An absent field and a stated absence read
identically to a machine and differently to a person, and this project needs the
second, so the test asserts the stated absence and not merely the absence of a
name.

**The mechanism, and who owes it.** Issue #45. That issue additionally asks for an
assertion that no assumption can be applied through a path that does not record it,
and it already anticipates that the language may not enforce it structurally, in
which case the test enumerates the paths and fails when a new one appears
unrecorded.

**The bound.** Naming an assumption is not explaining it. A reader who does not
know what a material row means learns from this only that one was used.

### Property 4. Widening an input uncertainty never narrows the reported region

**The property.** For a fixed scene, a fixed seed and a fixed level, increasing
the stated uncertainty of any one input never decreases the extent of the reported
region.

This is the strongest guard in the list against a subtle error in the propagation,
and it is the one most likely to catch a mistake that reads correctly in review.
It is also the only property here that tests the arithmetic rather than the
interface.

**What a test decides.** A property test generates scenes and measurements across
the ranges the tool supports, widens one input uncertainty at a time, and asserts
the reported region at a fixed level never shrinks. The generator covers the
awkward regions rather than the comfortable middle: near-normal incidence, nearly
parallel trajectories, holes close together, and scenes whose region reaches the
described extent. The case count and the seed are printed so a failing run can be
repeated.

**The mechanism, and who owes it.** Issue #42.

**The bound, and it is a real one.** The property is stated over the extent of a
region, and extent has to be defined before the assertion means anything. Two
regions can be compared by volume, by containment, or by a bounding measure, and
they do not agree: a region can gain volume and lose containment at the same time.
Which comparison is used is fixed in issue #42 with the reason, not here, because
fixing it needs the region construction that issue #44 decides. A monotonicity
test over the wrong measure passes while the property fails.

Sampling noise is the second bound. Under decision 0004 the region is drawn from
samples, so two runs differing only in an input uncertainty differ by noise as
well as by the effect being tested, and a tolerance has to be set that is wide
enough not to fire on noise and narrow enough to catch a real inversion. That
tolerance is a number, it does not exist yet, and it will be fixed with the
command that produced it rather than chosen.

### Property 5. An unconstrained result is an explicit result, never an empty or default one

**The property.** A reconstruction the inputs do not constrain returns a value
that says so. Not an empty region, not a zero-extent region, not a default region,
and not an error. Decision 0011 already fixes the two outcomes, a region or a
contradiction, and requires that a caller distinguishes them by the shape of the
value rather than by inspecting a field or reading a message.

**What a test decides.** A test enumerates the result values the library can
produce and asserts that none of them is a region of zero extent. Tests construct
each degenerate case decision 0011 lists and assert the documented result, named
after the case so a reader can match test to record without searching.

**The mechanism, and who owes it.** Issue #41.

**The bound.** This property is about the value. Whether the message beside it is
honest is property 6.

### Property 6. The text the tool emits does not carry the certainty vocabulary

**The property.** No text the tool emits contains the phrases that PCAST
recommends courts never permit, or the phrase the National Commission on Forensic
Science found has no scientific meaning.

**Why it is here.** Properties 1 to 5 all constrain the value the tool computes.
None of them constrains a word it prints. A report generator, an error message, a
help text, a documentation page or the worked example can carry "to a reasonable
degree of scientific certainty" while every computed value is impeccable, and the
sentence is what gets read into a record. The gap was found by
`../survey/challenges.md` while reading what has already been attacked in court,
and it is the failure mode the plan did not prevent.

**What a test decides, and what is still open.** A phrase list, a defined scope of
text the property covers, and a test that refuses a build emitting a listed
phrase. The scope question is not settled here: whether the property covers only
generated report text, or also error messages, help text, the shipped
documentation and the worked example, is decided in the issue below, and a
property that covered only the report while being described as covering the output
would be worse than none.

**The mechanism, and who owes it.** Issue #75, which holds the phrase list, its
sources, the scope decision, and the requirement that the test be shown to bite.

**The bound, stated here because it is the one most likely to be misread.** A
phrase list catches known phrasings and nothing else. Passing this check does not
mean the output is honest; it means the output does not contain those words. A
report can overstate its certainty in a sentence nobody has written down yet, and
this property will pass it.

## What is deliberately not a property

**That the answer is correct.** Nothing here checks that the region contains the
truth. That is calibration, it is milestone 7, and issue #50 asks whether a stated
region contains the truth as often as it claims. A conformance suite that passes
tells you the tool refuses to overstate, not that it is right.

**That the operator's own text is honest.** The tool constrains what it puts in
the operator's hands with its name on it. What the operator writes is theirs.

**That the report is understood.** A region correctly labelled and correctly
constructed can still be read as a point by somebody who wanted a point. Nothing
in software fixes that, and pretending otherwise would be this record making the
mistake it is about.

## The reasons

Six properties rather than one, because they fail for six different reasons and a
person meeting a red suite should learn which one. The interface property and the
propagation property have almost nothing in common except the condition they
serve.

Enforcement split between the type system and tests, stated per property, because
the two are not equally strong and a reader who is not told assumes the stronger
one. Where decision 0002's language can make a violation fail to compile, that is
said in the property; where it cannot, a test is named.

Property 4 stated over one input at a time, because that is what a generator can
actually search. Widening several at once and asserting the same thing is a weaker
test that looks stronger, since a narrowing caused by one input can be masked by a
widening from another.

The list published as a floor, because publishing it as a guarantee is the
identical error to the one property 6 exists against: a check whose passing gets
read as a property it does not have.

## What this costs

Property 1 costs the users who genuinely want a point. Somebody drawing a diagram,
somebody comparing this tool against another that reports a position, somebody
whose workflow downstream takes coordinates. All of them are told no, and some of
them will use a different tool. That is the cost being bought deliberately, and it
is the whole point of the property, so it is not softened here.

Property 4 costs run time in the default suite, and property tests are the
tests most likely to be quietly reduced when the suite gets slow. Issue #42
already fixes what happens then: the case count is reduced and the reduced count
is recorded, rather than the test being moved out of the default suite.

Six properties are six things to keep true, and each one is a place a future
change has to argue past rather than simply make. That is the intended cost.

The list will grow, and every addition is an argument about whether a new failure
mode is real. That argument is cheaper than the failure.

## What would falsify this

A property here turning out to be unenforceable in the chosen language and in a
test, in which case that property is not a property and this record is superseded
by one that says what replaces it. The candidate is property 1: enumerating a
public interface from inside the language is not something every language supports,
and issue #46 already carries the instruction to say so in `docs/OUTPUT.md` rather
than record an unenforced rule as enforced.

A failure mode that breaks the condition and passes all six. That does not
falsify the properties; it falsifies the completeness this record explicitly
disclaims, and the response is a seventh property with its own issue.

Property 4 firing constantly on sampling noise, which would mean the tolerance
cannot be set to separate noise from an inversion, and the property has to be
restated over a quantity that is stable under resampling.

A monotonicity counterexample that turns out to be correct behaviour rather than a
defect. If widening an input can legitimately narrow the region under some
construction, then the property is wrong about the arithmetic and its statement
needs the construction named in it.

## Evidence

The phrases property 6 refuses, and their sources, are recorded in
`../survey/challenges.md`, which quotes the PCAST report from the PDF, downloaded
and converted with `pdftotext -layout`, and quotes the National Commission on
Forensic Science through the same report. Issue #75 carries the list and the scope
decision.

The two outcomes property 5 rests on, a region or a contradiction distinguished by
the shape of the value, are decided in `0011-degenerate-cases.md`.

The requirement that a region built from too few samples is refused rather than
reported is decided in `0009-determinism.md`.

That the model samples throughout, which is what makes property 4 a statistical
test rather than an exact one, is decided in `0004-uncertainty-model.md`.

What a prior entry carries, and the requirement that a run applying none says so,
are decided in `0008-priors.md`.

The counts showing that no source file and no build manifest is tracked at this
commit are quoted above with the commands that produced them, run at
`5fbc1c9ff9177d045b087a08fe693f53aef8ecc1`, which is the commit this record was
written on and one commit before the one it lands as.

No test in this record has been run, because none of them exists. No number in
this record was measured.
