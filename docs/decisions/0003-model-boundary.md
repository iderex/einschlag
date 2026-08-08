# 0003. The model is straight-line geometry with error propagation, and the tool warns beyond 20 m

## Status

Accepted.

## Date

2026-08-08.

## The question

`README.md` describes this project as pure geometry plus error propagation. That
is a boundary, and an unwritten boundary gets crossed by whoever next has a good
idea, one reasonable addition at a time.

The pressure to cross it is specific and it is not vandalism. Every effect left
out of the model is a real effect. Drag is real, drop is real, wind is real, and
somebody who knows that will eventually notice that the tool ignores all three
and will be right that the trajectory is not a straight line. What the boundary
has to answer is not whether those effects exist. It is what including one costs
in inputs the operator does not have, and at what point excluding it makes the
answer wrong rather than merely approximate.

The answer has to carry a number. A boundary stated as a preference is not
checkable, and a boundary stated as "short range" is not either, because nobody
agrees on what short is. So this record states the range beyond which the
straight-path assumption stops being safe, and how that range was arrived at.

There is a second half. `../survey/methods.md` ends with a proposal for which
direction methods the first version should support and which it should defer.
That proposal was written from a reading and says the record is where it is
settled. This record settles it, and where it disagrees with the proposal it says
so here rather than leaving two lists in the tree.

## The options considered

**Model the full exterior ballistics: drag, drop, wind, and the projectile's own
dynamics.** This is what the field does for long range, and
`../survey/tools.md` records one published program that does exactly it. Cost:
every one of those terms needs an input the operator usually does not have. Drag
needs the ballistic coefficient, which needs the projectile identified to a
specific bullet and load. Drop needs the muzzle velocity, which needs the weapon
and the load. Wind needs a wind field at the time of the incident, which is
almost never recorded. A model that takes those inputs and, in their absence,
substitutes a typical value produces a narrow answer from a guess. That is the
failure this project exists against, and it arrives wearing the clothes of a
better model.

**Model the effects but carry the missing inputs as wide uncertainties.**
Superficially the honest version of the option above, and it is the one this
record spent the longest on. Cost: the widths are not available either. A
uniform over "some plausible range of ballistic coefficients" is a distribution
this project invented, and it would sit inside the propagation where no reader
can see it. It also fails asymmetrically: unlike the independence assumption in
`0004-uncertainty-model.md`, it does not err wide, because a wrong central value
for drop displaces the answer rather than spreading it.

**Straight lines only, with no statement about where that stops.** The cheapest
option and the current state of the README. Cost: it is not a boundary, it is a
silence, and a user reconstructing a 400 m shot gets an answer with nothing on it
saying that the model does not apply.

**Straight lines only, with the range at which the assumption stops being safe
stated in the record and warned on by the tool.** Cost: a threshold has to be
chosen and defended, the tool has to carry the arithmetic that decides when to
warn, and the threshold is a single number standing in for a family of curves
that depend on the projectile. It is also a boundary a user can argue with, which
is the property being bought.

## The option taken

The model is straight-line geometry from a perforation to a direction, with the
measurement uncertainty propagated through it, and nothing else. The tool warns
when any part of the answer lies further than **20 m** from the hole that
constrained it.

### What is inside the boundary, with the reason for each

- **The direction a perforation constrains, from its axis ratio and the
  orientation of its major axis on the surface.** This is the relation the
  project is built on and the only one for which this repository has read any
  measured error at all.
- **The uncertainty on that direction, propagated from the stated uncertainties
  of the measured quantities.** Without it the tool is a protractor.
- **Two defects declared by the operator to be from one projectile, as a
  geometric constraint on the line through both.** It is geometry over two
  measured points and it needs no error model of its own beyond the two points'
  own uncertainties.
- **A direction the operator supplies with its own stated uncertainty.** Inside,
  and the reason is in the reconciliation section below.
- **The combination of several such constraints, and the region of space
  consistent with them.** This is the output the project is for.
- **Occlusion by surfaces the operator declared, where no perforation was
  documented.** `0008-priors.md` already fixes that this is geometry and not a
  prior, because it is a statement about the projectile's path through the
  described scene.

### What is outside the boundary, with the reason for each

- **Aerodynamic drag.** Needs the ballistic coefficient, which needs the
  projectile identified to a load. Its effect on the reconstruction is through
  the time of flight, which is why it appears in the arithmetic below only as the
  reason a no-drag number is a lower bound.
- **Gravity drop.** Needs the muzzle velocity. This is the one omission whose
  size can be bounded without knowing the input, which is what the next section
  does, and it is the omission the 20 m warning is about.
- **Wind.** Needs a wind field at the time and place of the incident. This is the
  effect this project has least ability to bound, because unlike drop it is not
  one-directional and its magnitude is not a function of range alone.
- **Projectile yaw, precession and nutation.** Needs the projectile, the twist
  rate and the range. Excluded for the same input reason as drag, and named
  separately because it is the effect that would move the axis ratio relation
  itself rather than the flight path.
- **Deflection inside a target, and deviation on perforating a thin panel.**
  `../survey/methods.md` records a measured deviation phenomenon for one
  projectile and substrate pair and no error distribution for it anywhere. The
  tool models neither the deflection nor an inflation to cover it, for the reason
  `0006-frame-and-units.md` already gives about curved surfaces: widening
  requires a number, and there is none.
- **Ricochet geometry as a direction estimator.** Deferred, with the reason in
  the reconciliation section.
- **The lead-in method.** Deferred, with the reason in the reconciliation
  section.

### Where the straight-path assumption stops being safe, and how that was worked out

The quantity that matters is not the drop. It is the angle between the straight
line the tool draws and the direction the projectile was actually travelling,
because that angle is what displaces the reconstructed origin.

Take the horizontal speed over the path as constant at `v` and take no drag. The
time of flight over horizontal range `R` is then `R/v`, the drop is `g(R/v)^2/2`,
and the angle between the launch direction and the velocity at `R` is
`arctan(gR/v^2)`. Drag can only increase the time of flight, so all three of
those are lower bounds on the real values, and that is the only role drag plays
in this record.

Rather than assume a muzzle velocity, which this project has no source for, the
arithmetic is inverted: for a given range, what is the slowest projectile for
which the no-drag deviation still sits inside a stated tolerance.

    python -c "
    import math
    g = 9.80665
    for R in (20.0, 30.0):
        vd = R * math.sqrt(g / 0.1)
        va = math.sqrt(g * R / math.tan(math.radians(0.3)))
        print('R=%2.0f m   5 cm of drop needs v=%6.1f m/s'
              '   0.3 deg of drop angle needs v=%6.1f m/s' % (R, vd, va))
    for v in (200.0, 250.0):
        for R in (20.0, 30.0):
            print('v=%3.0f m/s R=%2.0f m   drop=%5.1f mm   drop angle=%5.3f deg'
                  % (v, R, 1000 * 0.5 * g * (R / v) ** 2,
                     math.degrees(math.atan(g * R / v ** 2))))
    "
    R=20 m   5 cm of drop needs v= 198.1 m/s   0.3 deg of drop angle needs v= 193.5 m/s
    R=30 m   5 cm of drop needs v= 297.1 m/s   0.3 deg of drop angle needs v= 237.0 m/s
    v=200 m/s R=20 m   drop= 49.0 mm   drop angle=0.281 deg
    v=200 m/s R=30 m   drop=110.3 mm   drop angle=0.421 deg
    v=250 m/s R=20 m   drop= 31.4 mm   drop angle=0.180 deg
    v=250 m/s R=30 m   drop= 70.6 mm   drop angle=0.270 deg

`g` is 9.80665 m/s^2, the standard acceleration of free fall.

The two tolerances in that command, 5 cm of drop and 0.3 degrees of drop angle,
are not this project's inventions. They are the tolerances Kerkhoff, Broekhuis,
Mattijssen and Riva state alongside the two threshold distances they propose, and
the abstract is quoted in the evidence section below. The arithmetic says that at
20 m those tolerances hold, without drag, for any projectile whose mean
horizontal speed over the path is at least about 198 m/s, and that at 30 m they
hold above about 297 m/s. Both figures sit below the speeds a handgun bullet
travels at, which is why the two published thresholds are conservative rather
than tight, and it is also the check that this record's arithmetic and that
study's numbers are describing the same thing.

**The tool takes 20 m and not 30 m, because it cannot tell the two cases apart.**
The distinction those thresholds rest on is whether the bullet was subsonic,
transonic or supersonic, and nothing in the input format fixed by
`0007-input-format.md` carries the projectile, the load or the velocity. A tool
that chose the longer threshold would be choosing it on an assumption the
operator never made.

**What the warning is attached to.** The range is not an input; it is part of the
answer. So the condition is evaluated on the result: where any part of the region
at any reported level lies further than 20 m from the hole that constrained the
trajectory reaching it, the artefact carries the warning, and it names the
distance actually reached rather than only the fact of exceeding it. A scene
whose whole region is inside 20 m carries no warning, and the artefact says that
positively rather than by omitting the field, in the same shape
`0008-priors.md` requires for a run that applied no prior.

**Why the warning is not a widening.** At 20 m the no-drag drop angle for a
200 m/s projectile is 0.281 degrees, which is roughly a tenth of the plus or
minus 2.6 degrees at approximately 95 per cent that `../survey/ellipse-accuracy.md`
records for a trajectory rod read by a laser scanner, and a much smaller fraction
of the method errors in `../survey/methods.md`. It would be tempting to conclude
that the omission is negligible against the uncertainty the tool already carries,
and this record does not draw that conclusion. The drop error is systematic and
it has a sign: the projectile is always below the straight line, never above and
never scattered about it. The measurement error is not. Combining several
trajectories narrows the random part and does not touch the systematic part, so a
systematic term that is small against one hole's uncertainty is not small against
the combined answer, and it displaces rather than widens. That is the same
mechanism issue #78 raises for correlated measurement error, arriving from a
different source, and the response here is the same: state it and do not let a
convenient comparison bury it.

## The reasons

Straight lines were chosen over the full model because of what the missing inputs
do, not because the full model is worse. With the ballistic coefficient, the
muzzle velocity and a wind field, the arced reconstruction is the better one.
Without them, every one of those terms has to be supplied by the tool, and a term
the tool supplied is a term the reader will read as measured.

The full model with wide uncertainties on the missing inputs was rejected because
it fails in the direction this project cannot accept. The independence assumption
in `0004-uncertainty-model.md` is kept despite being false because it errs wide.
Substituting a plausible ballistic coefficient does not err wide; it moves the
centre of the answer to wherever the substituted value puts it, and the width
around the wrong place says nothing about the distance from the right one.

The threshold is stated as a warning rather than as a refusal because a
reconstruction beyond 20 m is not meaningless. It is a reconstruction whose
vertical component carries a systematic error the operator has to weigh, and a
tool that refused it would push the operator back to the practice that carries
the same error and does not mention it.

The threshold is stated on the answer rather than on the input because the
distance is not something the operator declares. Attaching it to a declared
expected range would let the warning be switched off by an operator who guessed
low.

## What this costs

**It costs the long-range case entirely.** A shot from 400 m is exactly the case
`../survey/tools.md` records a published program addressing, and this tool will
warn and produce a reconstruction whose vertical component is wrong by an amount
it cannot state. The cost falls on the operator working an incident with a long
line of sight, which in the human rights documentation setting is not the rare
case, and it is stated here rather than discovered.

**It costs the wind case with no warning at all.** The 20 m threshold is derived
from drop. Nothing in this record bounds the wind term, and a crosswind can
displace the reconstruction laterally at ranges where the drop term is still
inside tolerance. This is the largest hole in the boundary and it is not closed
by the number above.

**It costs the thin-panel case.** A reconstruction through a perforated sheet
carries a deviation the survey has measured for one projectile and substrate pair
and this model does not represent, and the tool says nothing about it because
there is no figure to say anything with.

**It costs users who want the model improved before the inputs exist.** Somebody
will offer a ballistic coefficient table. Accepting one would cross this boundary
and this record is the thing that says no.

## What would falsify this

A measured figure showing that the deviation from the straight path is material
inside 20 m for a projectile and substrate combination this tool is used on. The
threshold would then be wrong and this record superseded by one carrying the
shorter distance and its source.

An input the operator reliably does have that constrains the drop. If it turns
out that the projectile and load are known in most of the cases this tool is
used on, the drag and drop exclusions rest on a false premise about what an
operator has, and the boundary moves.

The warning firing on most real scenes. If the region routinely reaches past 20 m
from its holes, then the threshold is not marking an unusual case, it is marking
the normal one, and a warning that fires every time is a warning nobody reads.
The response would be to model the drop with the operator declaring a velocity
class, not to raise the threshold.

A measurement of the wind term showing it is bounded over the ranges this tool
supports, which would close the largest gap named under costs and would be an
addition to this record's arithmetic rather than a supersession of it.

## Reconciliation with the method list in `../survey/methods.md`

That file ends with a proposal for what the first version supports and defers.
This record agrees with it on five of six items and resolves one disagreement.

**Agreed, support: the ellipse method.** Same reason.

**Agreed, support: two defects on one trajectory as a geometric constraint.** Same
reason, and this record adds that the survey found no study reporting the angular
precision of a two-point direction as a function of the separation between the
defects. The constraint is inside the boundary because it needs no such figure:
the two points carry their own measured uncertainties and the geometry does the
rest. Issue #38 is where the behaviour at small separations is asserted.

**Disagreement, resolved in favour of support: a direction the operator supplies
with its own stated uncertainty.** The issue that asked for this record placed
inside the boundary "the direction a hole constrains", which does not obviously
cover a direction the tool did not derive. The proposal in the survey placed it
inside. This record places it inside, and the reason is that the boundary is
about which physics the model represents, not about which instrument produced a
constraint. A supplied direction with a supplied uncertainty is propagated by
exactly the same straight-line arithmetic as a derived one, so admitting it
crosses no line this record draws.

It carries one condition, and it comes from the failure mode
`../survey/methods.md` identifies as the important one in that file: an
instrument figure of about a degree quoted for a direction that came from a
method whose error is ten or twenty degrees understates the uncertainty by an
order of magnitude. So a supplied direction records what its uncertainty is an
uncertainty of, the artefact names it, and the tool does not treat the two as the
same kind of number.

**Agreed, defer: the lead-in method.** The reported error is a curve per calibre
and ammunition combination rather than a figure, and in some cases exceeds 20
degrees, so no defensible uncertainty can be read out of what was obtained. This
record adds that the deferral is about the error model and not about the
geometry: a lead-in direction can enter today through the supplied-direction
route above, with the operator's own number on it and named as theirs.

**Agreed, defer: ricochet geometry as a direction estimator.** What the
literature supplies is critical angles for particular projectile and material
pairs and a statement that the ricochet angle is systematically below the
incidence angle. That is a systematic displacement with no distribution attached,
which is the same shape as the drop term this record excludes, and it is excluded
for the same reason.

**Agreed, defer and keep in view: the critical angle as a consistency check.** The
survey defers it because the scene model does not yet carry materials. Issue #76
adds a second reason that was not available when that file was written: the
material table has no rows, because no full text was obtained for any study in
`../survey/ellipse-accuracy.md`. A consistency check against published critical
angles needs a per-material table this repository cannot yet fill.

## Evidence

The two threshold distances and the two tolerances this record's arithmetic is
checked against are from the abstract of Kerkhoff, Broekhuis, Mattijssen and
Riva, "The systemic error in the vertical component of handgun bullet trajectory
reconstructions", Journal of Forensic Sciences, 2024,
[10.1111/1556-4029.15431](https://doi.org/10.1111/1556-4029.15431). Read from
Europe PMC with

    curl -s -G "https://www.ebi.ac.uk/europepmc/webservices/rest/search" \
      --data-urlencode 'query=DOI:"10.1111/1556-4029.15431"' \
      --data-urlencode 'format=json' --data-urlencode 'resultType=core'

which returns, in the abstract field:

> Threshold values of 20 and 30 m are proposed as conservative distances up to
> where bullet trajectories can be modeled as straight lines with
> subsonic/transonic handgun bullets and with supersonic handgun bullets
> respectively. Both the bullet drop and vertical offset will be below 5 cm at
> these distances for those categories. The drop angle will be below 0.3°.

**Abstract only.** No full text was obtained for that study. The per-combination
drop tables it reports are not in this repository, and this record therefore
takes the two thresholds and the two tolerances as stated and derives its own
arithmetic independently rather than reproducing theirs. The routes tried and
refused are recorded in `../survey/full-text-acquisition-2026-08-08.md`.

The plus or minus 2.6 degrees at approximately 95 per cent for a trajectory rod
read by a 3D laser scanner is Greenwood, Paduch and Allen, 2023,
[10.1111/1556-4029.15230](https://doi.org/10.1111/1556-4029.15230), as recorded
in `../survey/ellipse-accuracy.md`. Abstract only, on the same route.

The arithmetic in this record was run with the command printed beside its output
above. It rests on no measurement of this project's own, and it uses no muzzle
velocity, no ballistic coefficient and no drag model, which is the point of
inverting it.

**Five files this record refers to are not on the default branch at the commit it
was written on.** `../survey/methods.md`, `../survey/tools.md`,
`0004-uncertainty-model.md`, `0006-frame-and-units.md` and `0008-priors.md` are
on the branch of the open pull request that lands issues #3, #5, #11, #13, #15,
#17 and #19, and they were read there at `8134ddc19a7a957c9e94e5e959a30377089fe502`:

    git show 8134ddc19a7a957c9e94e5e959a30377089fe502:docs/survey/methods.md

The links resolve once that pull request lands, and until it does they do not.
This is written here rather than left for a reader to discover, because a
reference that does not resolve is exactly the defect a record about boundaries
should not carry silently.
