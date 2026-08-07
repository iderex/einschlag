# 0011. Unconstrained and contradictory are different results, and neither is an error

## Status

Accepted.

## Date

2026-08-07.

## The question

One hole in one wall constrains a direction and nothing about distance. Two holes
whose measured uncertainties overlap constrain almost nothing. Contradictory holes
constrain nothing at all, and mean that somebody mismeasured or that the holes are
from different shots.

These are the normal cases, not the edge cases. A tool that treats them as errors
will be worked around by an operator under pressure, and the workaround will be to
delete the inconvenient hole, which produces a narrow answer from a scene that did
not support one.

The shape of the answer matters more than the wording. An unconstrained direction
is a real result. A contradiction is a different kind of thing: it says the inputs
cannot all be true, and reporting a region there would be a fabrication.

## The options considered

**Every degenerate case is an error and the run fails.** Simple, and the operator
always knows something happened. Cost: the single-hole case is the most common
input this tool will ever see, and it is not an error. Making it one teaches the
operator that the tool's refusals are noise to be routed around.

**Every degenerate case returns a region, possibly a very large one.** Uniform for
a caller, and nothing special-cases. Cost: it collapses the contradiction into the
unconstrained case. A region reported for a set of constraints that cannot all be
true is a statement that the shooter was somewhere in it, and no such region
exists. This is fabrication with a large error bar on it.

**Two outcomes, distinguished in the result type: a region, or a contradiction.**
The caller has to handle both and cannot accidentally read one as the other. Cost:
a result type with two shapes is more work for every caller, including the ones
that only ever meet the ordinary case.

**Three or more outcomes, one per degenerate case.** Most informative. Cost: the
distinctions between the unconstrained cases are about how wide the answer is and
why, which is information about a region rather than a different kind of result.
Splitting them into separate outcomes makes every caller enumerate cases that all
mean the same thing to them.

## The option taken

Two outcomes, distinguished in the type of the result.

**A region.** The set of positions consistent with the constraints at the stated
level, together with the assumptions that produced it and with flags describing
its shape. A region may be enormous, may reach the boundary of the described
extent, and may be the whole described space. All of those are regions.

**A contradiction.** A statement that no position is consistent with all the
declared constraints at any level the run was asked for, together with which
subsets of the constraints are mutually consistent and at what level the
inconsistency appears.

A caller distinguishes them by the shape of the value, not by inspecting a field
and not by a sentinel. Reading a contradiction as a region has to be a thing the
caller cannot do by forgetting.

Neither outcome is an error. Errors are reserved for a refused input, a refused
format version, and a request the run cannot satisfy such as too few samples.
Those are conditions in which the tool has not produced a reconstruction at all.

## The reasons

The dividing line between the two outcomes is not how wide the answer is. It is
whether an answer exists. Every unconstrained case has a true answer somewhere
inside the reported region, and the region is honest about not knowing where.
A contradiction has no true answer inside anything, because the premises cannot
all hold. Those are different statements and a reader who cannot tell them apart
has been told the wrong thing.

Two outcomes rather than more, because that is the number of genuinely different
statements. The rest is description of a region, and description belongs on the
region.

Not an error, because an error is a thing the operator is expected to fix, and in
these cases there is nothing to fix. The scene constrains what it constrains.

## What this costs

Every caller handles two shapes, including the visual output, the report and the
command line. The cost is real and it is the cost of the property being bought.

An operator meeting a contradiction for the first time on a real case will assume
the tool is broken. Nothing in this record fixes that; a worked example that
produces a contradiction, shipped in the repository, is what fixes it, and that
belongs to the example issue rather than here.

The contradiction result reports which subsets are consistent, which is more work
than reporting the fact of the contradiction, and on many holes the number of
subsets is large. A bound on that enumeration is not fixed here and will be needed.

## What would falsify this

Operators routinely receiving contradictions on scenes that are in fact fine. That
would mean the uncertainty model is too narrow rather than that the case analysis
is wrong, but it would show up here first, and it would make the contradiction
outcome noise.

A third genuinely different statement turning out to be needed. The candidate is a
scene where the consistent region is non-empty but lies entirely outside the
described extent, which is handled below as a region with a flag; if callers
consistently need to treat it as its own outcome, this record is superseded.

Callers reading a contradiction as an empty region in practice, despite the type
distinction. That would mean the distinction is not carried where it needs to be.

## The cases

### A single hole

What is returned. A region. A single perforation constrains a direction with an
uncertainty and constrains nothing about how far along that direction the shot
came from. The region is therefore the intersection of a cone-like volume with the
described extent, and it reaches the boundary of that extent.

How a caller distinguishes it. It is a region, carrying the flag that it reaches
the extent boundary, described below.

What the operator is told. That one hole gives a direction and no distance, that
the region shown is bounded by the space they described rather than by the
evidence, and that a second hole on the same trajectory or a second trajectory is
what would bound it.

### Holes so nearly parallel that the intersection is unstable

What is returned. A region, and a wide one. Two nearly parallel constraints
intersect over a long, thin volume, and small changes in either input move the
intersection a long way along its length.

How a caller distinguishes it. It is a region. It carries a conditioning flag
saying that the geometry is poorly conditioned and naming the constraints
involved, so that a caller can present it differently and so that the calibration
work in milestone 7 can break its figures down by conditioning.

What the operator is told. That the trajectories are nearly parallel, that the
answer is long and thin for that reason and not because the measurements were
poor, and that a constraint from a different direction is what would shorten it.

This case is the one where a tool that reports a single point does most damage,
because the point sits in the middle of a volume the data does not distinguish.

### Holes whose constraints do not intersect at any level

What is returned. A contradiction.

How a caller distinguishes it. It is the contradiction outcome, not a region.

What the operator is told. That the constraints cannot all be true. Which
constraints are mutually consistent and which are not. The level at which the
inconsistency first appears, because constraints that separate only at a high
level are a different situation from constraints that separate immediately.

What the operator is not told is which measurement is wrong. The tool can say that
the constraints do not intersect. It cannot say which one is at fault, and it does
not imply that it can. The message names the two most likely explanations, that a
measurement is wrong or that the holes are from different shots, and it says that
choosing between them is not something the tool can do.

There is one thing the tool may legitimately add, because it follows from the data
rather than from a judgement: which single constraint, if removed, would leave the
remainder consistent, where exactly one such constraint exists. That is an
arithmetic fact about the constraint set. It is reported as that and not as an
identification of the mistaken measurement, and where more than one such
constraint exists all of them are named.

### A hole at near-normal incidence, where the angle relation degenerates

What is returned. A region. As the impact approaches normal the perforation
approaches circular, the major axis stops being defined, and the azimuth derived
from it is unconstrained rather than merely uncertain.

The azimuth is treated as unconstrained, over its full range, rather than as a
wide distribution around a computed value. The distinction matters: a wide
distribution still has a mode, and the mode is an artefact of the noise in a
measurement that carries no information.

How a caller distinguishes it. It is a region, carrying a flag naming the hole and
saying that its azimuth was taken as unconstrained.

What the operator is told. That the hole is too close to circular for its
orientation to mean anything, that the tool has used it for what it does
constrain, which is that the shot came from the side of the surface the hole
entered, and that a confident bearing from this hole is not available from any
method.

An operator who states a bearing for such a hole in the input is not overridden
silently. The input value is used only if the axis ratio is far enough from one
for it to carry information, and where it is not, the run says that the stated
bearing was not used and why.

### A scene in which the entire consistent region falls outside the described space

What is returned. A region, and the region is empty within the described extent.
This is not a contradiction: the constraints are mutually consistent, and they
place the shooter somewhere the operator did not describe.

How a caller distinguishes it. It is a region, carrying the flag that it is empty
within the extent and the flag that it reaches the extent boundary.

What the operator is told. That the constraints are consistent with each other,
that no position inside the space they described satisfies them, and that this
usually means the described extent is too small rather than that the reconstruction
failed. The message says what to do about it, which is to describe a larger space
and re-run.

This is the case most easily mistaken for a contradiction by a reader, and it is
the reason the two are separated by a type rather than by wording.

### Regions that reach the boundary of the described extent

Not a case on its own, and recorded here because three of the cases above produce
it. Any region whose boundary touches the extent carries a flag saying so.

The difference between "the shooter was inside this courtyard" and "the shooter
was somewhere, and this is the part of it we described" is the difference between
a reconstruction and an overstatement. A region that ends at the extent boundary
looks in a picture exactly like a region that ends because the evidence ended, and
the flag is what tells them apart. It is carried in the artefact, stated in the
report, and drawn in the visual.

## Evidence

The near-normal degeneracy is not a hypothetical. `../survey/ellipse-accuracy.md`
records that the ellipse relation takes the arcsine of the axis ratio, so as the
ratio approaches one the derived angle becomes arbitrarily sensitive to the
measurement, and that the studies read report their worst behaviour at the angles
where the perforation is least elongated.

`../survey/measurement-practice.md` records that no precision figure was found for
the orientation of a major axis, and that the failure mode there is absence rather
than imprecision, which is what the unconstrained treatment above implements.

No figure in this record was measured. The thresholds implied by phrases such as
"far enough from one" are not fixed here, because fixing them requires the
uncertainty model, and they will be fixed with the command that produced them.
