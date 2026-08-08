# 0008. Priors exist, none is on by default, and the line between a prior and geometry is drawn by whether the constraint is about the projectile or about the shooter

## Status

Accepted.

## Date

2026-08-08.

## The question

The geometry on its own is weak. One hole gives a direction and no distance, and
`0011-degenerate-cases.md` already records that the region for a single hole runs
until it meets the boundary of the space the operator described. Two nearly
parallel trajectories give a long thin volume that small measurement changes slide
a long way along its own length.

Everything that would make those answers narrow is an assumption. That a person
was standing on the ground. That a person is between one and two metres tall. That
nobody fires from inside a wall. Each of them is true often enough that leaving it
out feels like pedantry, and each of them is the exact mechanism by which a wide
honest answer becomes a narrow authoritative one.

This is the decision where this project's stated condition is most at risk, and
the risk does not arrive as a bad decision. It arrives as a convenience: a default
that nobody argued about, applied inside the tool, invisible in the output, and
quoted in a hearing as a reconstruction rather than as an assumption.

Three things are decided here. Whether priors exist at all in the first version.
What, if anything, is applied without being asked for. And on which side of the
line the constraints that feel like physics rather than assumption fall, because
that answer decides whether they are applied silently.

## The options considered

**No priors in the first version. Pure geometry and nothing else.** The strongest
possible position on the condition, and impossible to erode because there is
nothing to erode. Cost: the tool becomes unusable for the case it was built for. A
human rights organisation with two holes in one wall gets a region reaching the
extent boundary in every direction the geometry does not constrain, and the
knowledge that the shooter was on the ground rather than fifteen metres up in the
air lives in their head instead of in the artefact. They will apply it anyway,
outside the tool, undocumented, which is worse than applying it inside the tool
and recording it.

**Priors, with a standard set on by default, disclosed in the output.**
Immediately useful, and disclosure answers the honesty objection on paper. Cost:
the disclosure is a line in an artefact that a reader skips, and the number they
quote is the narrow one. The asymmetry is what decides it: an operator who wanted
a prior and did not get it notices immediately, because the answer is absurdly
wide. An operator who got a prior they did not want notices nothing at all,
because the answer looks better.

**Priors, none on by default, each stated by the operator, each named in the
output with what it removed.** Cost: the default output is wide, sometimes to the
point of looking broken, and the first impression of the tool is its worst one.
Every operator pays a step for something almost all of them want.

**Priors as a post-processing step outside the tool.** Keeps the reconstruction
pure and pushes the assumption to where it belongs, with the analyst. Cost: the
assumption then leaves no trace in the artefact at all, the run manifest cannot
record it, and the thing handed to a court is a region whose provenance stops
before the step that narrowed it. This is the option that most resembles current
practice and it is rejected for the reason the project exists.

On the classification question, separately:

**The ground plane and solid matter are geometry.** They are physical
impossibilities, not beliefs, so they are part of the scene rather than part of
the analyst's judgement. Cost: it makes "geometry" mean "things obviously true",
and once that door is open the next obviously true thing walks through it. It also
hides the fact that both constraints depend entirely on the operator having
described the scene completely, which no tool can check.

**They are priors like any other, subject to the same rule.** Consistent, and
nothing is applied that was not asked for. Cost: a default run reports positions
inside the ground, which is not merely wide but visibly silly, and an operator who
sees a silly answer stops reading the parts of it that are not silly.

**The line is drawn by what the constraint is a statement about.** Cost: it needs
an argument rather than an intuition, and the argument has to hold for cases
nobody has met yet.

## The option taken

Priors exist in the first version. **No prior is on by default.** Every prior is
stated by the operator, in the input file, by name. A run that states none applies
none.

Every applied prior appears in the output artefact and in the report, by name,
with the parameters it was given and with what it removed. `0004-uncertainty-model.md`
takes sampling throughout, so a prior is a filter over the drawn samples and what
it removed is a count: the artefact carries, per prior, the number of samples it
rejected and that number as a proportion of the samples that reached it. An
operator can therefore see that a prior removed four per cent of the answer or
that it removed ninety.

The order in which priors were applied is recorded, because the proportion each
one removed depends on it.

A prior is a declared constraint. It enters the constraint set with the geometric
ones, which means `0011-degenerate-cases.md` governs what happens when it cannot
be satisfied: a prior that rejects every sample produces the contradiction
outcome, not an empty region, and the contradiction names which subsets of the
constraints are mutually consistent. An operator whose ground-plane prior is
inconsistent with the geometry is told that, in those terms, rather than being
handed an answer with nothing in it.

### The line

**A constraint that is a statement about the projectile and its path through the
described scene is geometry.** It is part of working out what the measurement
means, it is applied always, and it is not a prior.

The case that matters is occlusion. A candidate trajectory that would have to pass
through a surface the operator declared, at a place where the operator documented
no perforation, is not consistent with the measurements themselves. Refusing to
apply that would mean reporting origins that this project's own input already
excludes. It is the same reasoning that turned the hole into a direction in the
first place, applied further back along the same line.

**A constraint that is a statement about the shooter is a prior.** It is never
applied unless asked for, whatever its truth.

That puts both constraints the issue asked about on the prior side, and they are
answered separately because they fail differently.

**Not below the ground plane: a prior.** It is a statement about where a person
can be, and it rests on a surface that `../survey/measurement-practice.md` records
is routinely not measured at all: a floor is taken as level and a wall as plumb
because that is what the building is supposed to be. It also fails in ordinary
ways. A slope outside the described extent, a stairwell, a trench, a basement
window, a lower storey the operator did not describe. Each of those is a real
place a shot has come from, and each is below a ground plane that was assumed
rather than surveyed.

**Not inside solid matter: split, and this is the part that would have been got
wrong.** The two halves are not the same claim.

The projectile could not have flown through a declared solid without a documented
hole in it. That is occlusion, it is geometry by the rule above, and it is applied.

The origin the tool reports is not a person. It is a point on the trajectory, and
the physical thing at that point is a muzzle. A muzzle at a window opening, at a
loophole, through a gap in a fence, out of a vehicle window, or through foliage is
not an exotic case; in the kind of incident this project's users document, it is a
common one. A constraint that excluded every position inside or against declared
solid matter would remove exactly those answers, and it would remove them silently
and for a reason that sounds like physics. So the muzzle-not-inside-solid
constraint is a prior, it is off by default, and its record says what it costs.

The distinction is that the first half constrains a flight path through the scene,
which is what was measured, and the second half constrains where a person and a
weapon can be, which was not.

### What a prior has to carry

A prior is not a switch. Each one is a named entry with:

- what it asserts, in one sentence, stated as an assumption rather than as a fact;
- its parameters, with units, and their uncertainty where it has one, since a
  height range is itself a measured or assumed quantity;
- what it is grounded in, which is either a source or the operator's own
  declaration, and the two are different words in the output;
- the count and proportion of samples it rejected, filled in by the run.

A prior with no parameters is still written out with its assertion, because the
sentence is the part a reader of the report needs.

### What the output says when no prior was applied

The output states that no prior was applied and that the region is therefore
bounded only by the geometry and by the described extent. That sentence is
present in the artefact and in the report on every run that used none.

The reason it is stated positively rather than left as an absence is that an
absence reads as an oversight. A wide region with nothing said about it invites
the reader to assume something was left out; a wide region with a line saying that
nothing was assumed is a result.

## The reasons

None on by default, because of the asymmetry of noticing. A missing prior is
loud and a silent prior is invisible, and the failure this project exists against
is the invisible one.

Priors inside the tool rather than outside it, because the operator will apply
them either way. The only question is whether the artefact records that they were
applied. Every argument for keeping the reconstruction pure ends with an analyst
narrowing the region by hand in a document that has no manifest.

The count of rejected samples rather than a description of the excluded region,
because the count is exact, it is free under the sampling model 0004 already
chose, and it is the number a reader can actually use. "This assumption removed
nine tenths of what the geometry allowed" is a sentence a court can weigh. A
description of an excluded volume is not.

The line drawn by subject rather than by obviousness, because obviousness is not
a property that survives contact with a real scene. Everything on the prior side
of this line is something almost always true, and "almost always" is where the
wrong answers live. The occlusion constraint is not on the geometry side because
it is more certain; it is there because it is about the projectile, which is the
thing that was measured.

The muzzle split, because it is the one place where the obvious classification
would have quietly deleted the correct answer in the cases this project's users
care about most.

## What this costs

The first run an operator does produces a wide region, and for a single-hole scene
it produces one that reaches the described extent in every unconstrained
direction. That is the tool's first impression and it is a bad one. Nothing here
softens it, and the worked example in milestone 10 is where an operator is shown
what a run with stated priors looks like beside one without.

Every operator pays a step for something most of them want. The cost falls on the
operator, every time, and it is bought against the one case where they did not
want it and would not have noticed.

Priors that are typed by hand are priors that can be typed wrong. A height range
entered in the wrong unit narrows the answer to nothing or to everything, and the
first of those produces a contradiction that names the prior, which is the good
failure. The second produces a wide region and no complaint.

The occlusion constraint is applied always and depends on the operator having
described the scene. A wall that was not described does not occlude, so an origin
behind it stays in the region, and the answer is wider than the truth. That
direction of error is the acceptable one, and it is worth saying plainly which
direction it runs in: an incompletely described scene produces an answer that is
too wide, never one that is too narrow, and the same incompleteness under a
silently applied solid-matter prior would have produced one that was too narrow.

Recording the proportion each prior removed costs an ordering, and the ordering
has to be recorded with it or the proportions do not mean anything. That is a
small thing that will be got wrong once.

## What would falsify this

Every operator turning on the same three priors on every run. That would mean the
default is a ceremony rather than a protection, and the answer is not to change
the default but to look at whether the artefact is being read at all. If the
proportion-removed figures turn out to be ignored, the disclosure is not doing the
work this record claims for it, and something stronger is needed.

A case where the occlusion constraint removed the true origin because the scene
description was wrong rather than incomplete. A declared surface that is not there
occludes something real, and unlike the incompleteness case it errs narrow. If
that happens in practice, occlusion moves to the prior side or gains an
uncertainty of its own.

A muzzle-position prior that operators can state precisely enough to be useful and
that never removes the window and loophole cases. That would show the split above
is finer than it needs to be.

The contradiction outcome firing so often on stated priors that operators stop
stating them. That would mean the constraint set is over-tight somewhere else,
most likely in the uncertainty model, and it would show up here first.

## What is not decided here

Which priors ship with names in the first version. This record fixes the rule and
the shape of an entry; the catalogue belongs with the code that implements them
and will be listed by a command rather than in this document.

How a prior is written in the input file. That is the input format's shape and
follows `0007-input-format.md`, which requires an explicit unit and an explicit
uncertainty on every measured value; a prior's parameters are measured values by
that rule.

Whether the tool ever refuses to run because a prior removed too much. Fixing a
threshold needs a number and there is none, so the tool reports the proportion and
does not judge it.

## Evidence

That the ground plane is routinely assumed rather than measured, and that a floor
is taken as level and a wall as plumb because that is what the building is
supposed to be, is in `../survey/measurement-practice.md`, which records it as one
of the quantities that is estimated rather than measured and states that no
precision figure was obtained for a forensic inclinometer or a zero-edge
protractor.

That a single hole constrains a direction and nothing about distance, and that the
resulting region reaches the boundary of the described extent, is decided in
`0011-degenerate-cases.md` and is the reason this record's default is wide.

That the model samples throughout, which is what makes the rejected-sample count
available, is decided in `0004-uncertainty-model.md`.

That the run manifest already carries every assumption that narrowed the answer is
in `0009-determinism.md`, which names that field and points at the issue that owns
it. The prior entries described here are what fills it.

**No measurement supports the claim that muzzles at window openings and loopholes
are common in the incidents this project's users document.** It is stated in this
record as the reason for a decision and it is an assertion about the world, not a
figure this project obtained. `../survey/field-practice.md` establishes who the
users are and what equipment they arrive with; it does not count firing positions,
and no source that does was found. If that assertion is wrong, the muzzle split
above is the part of this record that fails first.

No figure in this record was measured. No threshold is fixed in it.
