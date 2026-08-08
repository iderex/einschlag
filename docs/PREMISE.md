# The gap this project says it fills, re-examined against what the survey found

`README.md` states, in one clause, what this project is for. That clause is a
claim about other people's practice, and a survey was opened to check it rather
than repeat it. This file is what the check produced, and it is where the premise
now lives in full. The README carries the short form and points here.

## What the premise said

That reconstructing a shooter position from bullet holes is performed today with
string, lasers and proprietary software whose output is typically a line rather
than a probability distribution, and that this project's contribution is to
return a distribution instead.

## What the survey found, and what it did not

`survey/tools.md` set out to answer one question: does any surveyed tool already
emit a probability distribution over shooter positions. The answer it reached was
no, and that half of the premise stands. Two findings sit between a line and a
distribution, and the premise as written had no place for either.

The first is that the routine practice already returns a region and not a line.
The second is that one published program already returns an area produced by
varying its inputs, which is the shape of what this project proposes to do.

Neither finding was reached from a search summary. Both were read at their
sources, and this file re-read both rather than taking them from the survey file,
because a claim about somebody else's work made from the nearest thing to hand is
the defect this project exists against.

## Finding one: the practice returns a cone

Jason C. Fries, "Lidar's Revolution of Forensic Ballistic Trajectory Analysis",
LIDAR Magazine, 2023-07-22, fetched and read at

    curl -sSL -o lidar.html -w "http=%{http_code}\n" \
      "https://lidarmag.com/2023/07/22/lidars-revolution-of-forensic-ballistic-trajectory-analysis/"
    http=200

The article states the rule and then states what is done with it:

> because factors like impact misdirection or friction affect how a bullet
> travels through an obstruction, a window of error of five degrees must be
> accounted for when tracing a bullet from an entry angle to origin

> When experts trace five-degree windows of error for multiple shots fired in a
> tight pattern, the five-degree cones will grow and intersect at a certain
> distance. Like a Venn diagram, experts can find the intersecting section
> between all bullets' five-degree error windows and know the gun that fired
> these shots had to be in this area.

`survey/tools.md` records the same practice inside a named commercial package,
from a vendor blog post it fetched and read, where an analyst elongates rods to
five-degree cones.

So the field does not stop at a line. It traces a cone per shot, intersects the
cones from several shots, and reports the intersection as the region the shooter
was in. That is a region built from several constraints, and it is structurally
what this project set out to produce.

### What a five-degree cone is

A convention. One number, applied to every reconstruction, whatever the shot went
through and however it was measured.

The article gives the reason for the number as impact misdirection and friction,
which is a statement about the physics rather than a measurement of it, and no
source read here reports how the five was arrived at, on what materials, over
what angles, or with what coverage.

### What a five-degree cone is not

It is not an uncertainty derived from the measurement it is applied to. The same
five degrees is used on drywall and on sheet metal, at 20 degrees of incidence
and at 80, for a perforation measured as an ellipse and for a rod pushed through
a channel. The measured errors this repository has read do not behave that way.
`survey/methods.md` records a lead-in study reporting errors that are not
constant, that follow a different curve for each calibre and ammunition
combination, and that exceed 20 degrees in some cases; the same file records
instrument figures for a rod read by a laser scanner of about one degree, and
`survey/ellipse-accuracy.md` records plus or minus 2.6 degrees at approximately
95 per cent for that operation. A single constant sits between those two and
matches neither.

What that means at the distances a reconstruction actually spans:

    python -c "
    import math
    for R in (10.0, 20.0, 50.0, 100.0):
        print('R=%3.0f m   5 deg half-angle: %5.2f m   5 deg included: %5.2f m'
              '   20 deg half-angle: %6.2f m'
              % (R, R*math.tan(math.radians(5)), R*math.tan(math.radians(2.5)),
                 R*math.tan(math.radians(20))))
    "
    R= 10 m   5 deg half-angle:  0.87 m   5 deg included:  0.44 m   20 deg half-angle:   3.64 m
    R= 20 m   5 deg half-angle:  1.75 m   5 deg included:  0.87 m   20 deg half-angle:   7.28 m
    R= 50 m   5 deg half-angle:  4.37 m   5 deg included:  2.18 m   20 deg half-angle:  18.20 m
    R=100 m   5 deg half-angle:  8.75 m   5 deg included:  4.37 m   20 deg half-angle:  36.40 m

**Whether the five degrees is a half-angle or the full included angle was not
established.** The sources read say "a window of error of five degrees" and
"five-degree cone" without fixing it, so both readings are printed above and this
project does not choose between them on the material it has. The gap between the
two columns is a factor of two in radius at every range, which is itself an
argument about what a convention with no stated construction can support.

The third column is what a lead-in measurement at the upper end of its reported
error would give at the same ranges. Where the real error is that size, a
five-degree cone is not a conservative envelope; it is a region several times too
small, presented as an allowance for error.

That is the sharper form of this project's complaint, and it is a worse fault
than the one the premise originally named. A bare line at least looks like what it
is. A cone looks like an uncertainty statement, so it is harder to argue with,
and the practice around it reaches conclusions in the language of increased
certainty: the article says that with modifiable tracing cones experts can assert
the potential origin of a fired bullet with much greater certainty than ever
before, and that the intersection tells them where the gun had to be.

## Finding two: a published program already returns an area

Riva, Broekhuis, Haag, Koene and Kerkhoff, "Long-range trajectory reconstructions
using the point mass model", Journal of Forensic Sciences, 2025,
[10.1111/1556-4029.15697](https://doi.org/10.1111/1556-4029.15697). The abstract
was read at

    curl -s -G "https://www.ebi.ac.uk/europepmc/webservices/rest/search" \
      --data-urlencode 'query=DOI:"10.1111/1556-4029.15697"' \
      --data-urlencode 'format=json' --data-urlencode 'resultType=core'

**Abstract only. No full text was obtained.** Everything in this section is
therefore what an abstract supports and no more.

What the abstract states. A computer program using the point mass model can
perform long-range trajectory reconstructions starting from an impact point. The
reconstruction results in an area where the shot is expected to have been fired
from rather than a single location, and that area is caused by varying the input
parameters of the model. The model was validated against 20 handgun bullet
trajectories determined by Doppler radar over ranges from 500 m to 1800 m.
Comparing the calculated area against the actual shooter position demonstrates
the limits of these reconstructions, particularly at high incident angles, and
the differences between the reconstructed deflections and the deflections
measured by the tracking radar are stated to be rather large, attributed either
to measurement errors in the crosswind as a function of height or to inaccuracy
in the radar's deflection measurements.

What the abstract does not establish, and this file does not assume.

- Whether the area carries a stated probability level, or is the envelope of a
  parameter sweep with no level attached. "Caused by varying the input
  parameters" is consistent with either.
- Whether the program is available to anybody outside the group that wrote it. No
  repository or download was found on the route `survey/tools.md` used.
- How the input parameters were varied, and whether the variation was derived
  from measured uncertainties or chosen.

What the abstract does establish about overlap with this project. The same
abstract states that forensic examiners usually deal with short-range
trajectories, typically 30 m or less, and that a linear trajectory reconstruction
model is appropriate there. The validation range is 500 m to 1800 m. So the
program addresses the regime `decisions/0003-model-boundary.md` places outside
this project's boundary, and this project addresses the regime that abstract
calls linear.

## The premise, restated

Reconstructing a shooter position from bullet holes is performed today with
string, lasers, laser scanners and proprietary software. The routine practice
does return a region rather than a bare line: a fixed five-degree cone per shot,
intersected across shots. What no surveyed tool was established to do is derive
the width of that region from the measurement it came from, or attach a stated
level to it. This project takes measured hole geometry with its uncertainties and
propagates them, so that the width of the answer is a consequence of the data
rather than a constant, and so that a region carries the level and the
construction that produced it.

## Which findings it survives against, and how

It survives finding one by no longer claiming that the practice returns a line.
The complaint moves from the shape of the output to where its width comes from,
which is a claim the survey supports and the original claim was not. Nothing in
the milestones changes, because none of them rested on the practice returning a
line; they rested on the width being propagated, which is still the gap.

It survives finding two by range rather than by capability, and that is the
weaker of the two survivals. The point mass model program does produce an area
from varying inputs, which is structurally what this project proposes. What it
does not do, on the reading available, is address the short-range scenes this
project is aimed at, since its own authors place the linear model there. The
premise therefore survives as a claim about the short-range regime and not as a
claim about the whole field, and this file says so rather than letting the
distinction sit unstated.

Two things would change that. A full text showing the program carries a stated
level and is available for use would mean somebody has already built the thing
this project describes, at long range, and the honest response would be to say so
here and to argue this project's case on the short-range regime alone. A full
text showing it is applicable below 30 m would remove the range argument
entirely, and the premise would then need a different defence or would fail.
`survey/full-text-acquisition-2026-08-08.md` records that no full text was
obtained for that study on the routes tried, and which routes remain.

## What this re-examination did not settle

Whether the five degrees is a half-angle or an included angle, above.

How the five-degree convention was arrived at. No source read here gives its
origin, and knowing it would say whether the number is a rounded measurement or a
rule of thumb.

Whether any tool outside those in `survey/tools.md` derives its width from the
measurement. That survey's negative is bounded by the routes it used and states
its own bound; this file inherits that bound and does not widen it.

## Where the files named above are

`survey/tools.md` and `survey/methods.md` are not on the default branch at the
commit this file was written on. They are on the branch of the open pull request
that lands issues #3, #5, #7, #11, #13, #15, #17 and #19, and they were read
there at `8134ddc19a7a957c9e94e5e959a30377089fe502`:

    git show 8134ddc19a7a957c9e94e5e959a30377089fe502:docs/survey/tools.md

The two references to them resolve once that pull request lands, and until it
does they do not. The two sources this file rests on most heavily, the LIDAR
Magazine article and the Riva abstract, were fetched and read here rather than
taken from those files, with the commands beside the quotations, so the argument
above does not depend on them.

`survey/ellipse-accuracy.md`, `survey/full-text-acquisition-2026-08-08.md` and
`decisions/0003-model-boundary.md` are in the tree at this commit.
