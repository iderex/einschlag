# What this tool does not do

`NOTICE.md` says the software is for lawful use. That is the beginning of the
statement and not the whole of it, because the specific ways a shooting
reconstruction can do harm are not general ones. This document is the rest.

Read it before using the output anywhere it will be relied on. It is short on
purpose and none of it is boilerplate.

## The tool is not validated

In those words, and it is the first thing here because everything else is read
differently once it is known.

No figure in this repository states how often a region this tool reports contains
the true position, on synthetic scenes or on real ones. The calibration that
would produce such a figure is milestone 7 and it has not been run.
`docs/survey/ellipse-accuracy.md` records that no measured error for the ellipse
relation, tied to a stated range of angles, has been obtained for any material,
so `data/materials.toml` holds no rows and the tool refuses every hole handed to
it rather than estimating one. `docs/materials.md` says what would change that.

Once milestone 7 produces numbers, they stand beside this section rather than
replacing it, and the sentence above is then a statement a reader can check
instead of a statement they have to take.

## The output is an aid to a person, not an opinion this tool is offering

A reconstruction produced here is material for somebody who takes responsibility
for the reconstruction. It is not itself an expert opinion, it does not become
one by being produced by a program, and no number in it carries authority that
the person presenting it does not carry.

That sentence is deliberately not the whole answer. Somebody will want to use
this in work that is handed to a court, and telling them only what the tool is
not leaves them doing it anyway without knowing on what terms. So the conditions
are named.

A person using this in work they are answerable for is expected to be able to say
what the model represents and what it leaves out, which is
`docs/decisions/0003-model-boundary.md` and is summarised below; to state the
tool's validation status, which today is the section above; to have checked the
input file against their own measurements line by line, which is what the format
in `docs/decisions/0007-input-format.md` was chosen for; to reproduce the run
from the manifest it records; and to present the region at the level it was
computed at rather than a point inside it.

Anybody who cannot do all of that is using an output they cannot defend, and the
first place that becomes visible is under questioning by somebody who did not
write the tool.

## What the tool cannot do

**It does not identify a weapon.** Nothing in the model reads a calibre, a load,
a barrel or a mechanism. A perforation constrains a direction and this tool
propagates that constraint; the projectile that made it is not an output and not
an input.

**It does not identify a person.** A region of space is not somebody standing in
it. The tool has no access to who was where, and a region containing a place a
named person was known to be is a statement about the place, not about them.

**It does not establish intent.** Geometry says nothing about what anybody meant
to do. A reconstruction is consistent with any number of intentions and
distinguishes between none of them.

**It does not distinguish a shot fired from a shot deflected.** The model is
straight-line geometry from a perforation backwards.
`docs/decisions/0003-model-boundary.md` excludes deflection inside a target and
the deviation a projectile takes on perforating a thin panel, because the survey
records a measured deviation for one projectile and substrate pair and no error
distribution for it anywhere. A projectile that changed direction before making
the hole this tool is reading produces a region that is confidently in the wrong
place, and nothing in the output marks it.

**It does not know whether the measurements it was given are correct.** Every
number in the input is the operator's. A parser can refuse a shape that cannot be
right, which is what the one in #33 is for, and no check anywhere can tell a
well-formed wrong measurement from a well-formed right one. A metre entered as a
millimetre, an axis measured on the
wrong side of a hole, or a surface normal assumed rather than measured all
produce ordinary-looking output.

## Where the model stops

`docs/decisions/0003-model-boundary.md` is the record and carries the arithmetic.
What follows is its result, and where a number appears it is that record's.

The model is straight-line geometry from a perforation to a direction, with the
stated measurement uncertainties propagated through it, and nothing else.

**Beyond about 20 m the straight-path assumption stops being safe.** That record
inverts the usual arithmetic rather than assuming a muzzle velocity: at 20 m, a
drop below 5 cm and a drop angle below 0.3 degrees hold, with no drag, for any
projectile whose mean horizontal speed over the path is at least about 198 m/s,
and at 30 m the same tolerances need about 297 m/s. The two tolerances and the
two threshold distances are Kerkhoff, Broekhuis, Mattijssen and Riva's, from the
abstract quoted in that record. The tool takes the shorter threshold, because
nothing in the input format carries the projectile, the load or the velocity, so
it cannot tell the two cases apart.

The threshold is a warning and not a refusal, and the warning is attached to the
answer rather than to the input, since the distance is part of the result.

**A drop error displaces the answer rather than widening it.** The projectile is
always below the straight line, never above and never scattered about it.
Combining several trajectories narrows the random part of the error and does not
touch that. A systematic term that looks small beside one hole's uncertainty is
therefore not small beside the combined answer, and this is why the boundary
record refuses to call the omission negligible.

**Wind is the largest hole and it carries no warning at all.** The 20 m threshold
is derived from drop. Nothing bounds the wind term, and a crosswind can displace
a reconstruction sideways at ranges where the drop term is still inside
tolerance.

**These effects are outside the model, each because it needs an input an operator
usually does not have:** aerodynamic drag, which needs the ballistic coefficient;
gravity drop, which needs the muzzle velocity; wind, which needs a wind field at
the time and place; and projectile yaw, precession and nutation, which need the
projectile and the twist rate. A model that substituted a typical value for any
of them would narrow the answer from a guess, and the reader would have no way to
see it.

**Ricochet geometry and the lead-in method are deferred as direction
estimators.** What the literature supplies for ricochet is critical angles and a
systematic offset with no distribution attached; for the lead-in method it is a
curve per calibre and ammunition combination, exceeding 20 degrees in some cases.
A lead-in direction can still enter as a direction the operator supplies with
their own stated uncertainty, named as theirs.

**A direction supplied by the operator is only as good as what its uncertainty is
an uncertainty of.** An instrument figure of about a degree quoted for a
direction that came from a method whose error is ten or twenty degrees understates
the uncertainty by an order of magnitude. The boundary record requires a supplied
direction to record what its uncertainty is an uncertainty of, and the artefact
to name it, so that the two are not read as the same kind of number. Nothing
implements that yet.

## What the tool computes today

Nothing yet, and this section will be wrong before the rest of the document is.

The geometry that turns a direction into a region is milestone 5 and does not
exist. There is no parser for the input format, no output artefact and no report.
What is in the tree is the hole record, the scene, the sampling, the material
table and the checks around them.

So the limits above are the limits of the model this project has decided to
build, stated before the code that depends on them exists, which is the order
`docs/decisions/README.md` requires. They are not a description of a working
program.

## What is not settled here

Whether this tool's output may be relied on in a particular proceeding is a
question for the law that applies there and for the person presenting it. This
document states what the tool does and does not do; it does not state what any
jurisdiction will accept, and no sentence in it should be read as doing so.

`docs/survey/challenges.md` records what courts have been told about claims of
this kind, and `docs/survey/standards.md` records what the governing
documentation standard requires of a scene record. Both are readings of other
people's documents rather than positions this project takes.
