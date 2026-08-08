# The other angle and direction methods, and where each one stops

A perforation is not the only mark a projectile leaves that constrains where it
came from, and the ellipse relation is not the only route from a mark to a
direction. This file is a reading of what the published work says about the other
routes: what each one measures, what it assumes, the angle range over which it is
reported to work, the error where a figure was obtained, and how it fails when
the assumption does not hold.

The ellipse relation itself has its own file, `ellipse-accuracy.md`, and the
figures there are not repeated here. What this file adds about the ellipse is
where it stops relative to the alternatives, which is a comparison and not a
restatement.

This file is a reading. It does not derive an uncertainty model from any figure
below; that is milestone 5.

## How these entries were obtained

Bibliographic fields and abstracts were read from Europe PMC, and where a work is
not indexed there, bibliographic fields were read from Crossref. The two commands
were, with the query varied per method:

    curl -s -G "https://www.ebi.ac.uk/europepmc/webservices/rest/search" \
      --data-urlencode 'query=TITLE:"ricochet" AND (TITLE:"angle" OR TITLE:"critical")' \
      --data-urlencode 'format=json' --data-urlencode 'resultType=core'

    curl -s "https://api.crossref.org/works/10.1080/00085030.2023.2169478"

**No full text was obtained for any entry in this file.** Resolving the DOI was
tried for three of them and the result was this:

    10.1016/j.forsciint.2016.03.039 -> 200
    10.1111/1556-4029.15523         -> 200
    10.1080/00085030.2023.2169478   -> 403

produced by

    for d in 10.1016/j.forsciint.2016.03.039 10.1111/1556-4029.15523 10.1080/00085030.2023.2169478; do
      printf "%s -> " "$d"
      curl -s -o /dev/null -L -w "%{http_code}\n" "https://doi.org/$d"
    done

The two 200 responses are not article text. The body returned for the first is a
2709-byte redirect stub whose own page name is "Article Locator Bypass"; it
carries no part of the article. So the 403 and the two 200s are the same result
for this project's purposes, and it is worth writing down that a 200 here does
not mean a document was read.

Every claim below is therefore from an abstract. Where a field says a value was
not obtained, that means it was not in the abstract, not that the study failed to
report it. The per-angle error tables are exactly what this project needs and
none of them were read.

One thing the abstracts do not settle, and it matters more here than anywhere
else in this survey. **These studies do not consistently say whether an angle is
measured from the surface or from the surface normal**, and several of them use
"angle of incidence" and "impact angle" for what appears to be the same quantity.
Where a statement below says "low angle of incidence" it reproduces the source's
own wording and this survey has not established the convention behind it. That
ambiguity is an argument for the frame and units decision in milestone 2 fixing a
convention explicitly, and it is a reason not to combine figures from two of these
studies into one number without reading their full texts first.

## The probing method

**What it measures.** The straight line through a channel the projectile made,
read directly with a rod. The channel is either through one target thick enough
to hold the rod, or between a primary and a secondary defect from the same shot.

**What it assumes.** That the projectile travelled in a straight line over the
span being probed, that the material did not deflect it inside the channel, that
the rod is following the channel rather than widening it, and, in the two-defect
form, that both defects are from one shot.

**Angle range and reported error.** Mattijssen and Kerkhoff
([10.1016/j.forsciint.2016.03.039](https://doi.org/10.1016/j.forsciint.2016.03.039),
2016) compared variants of the probing, ellipse and lead-in methods on drywall,
MDF and sheet metal at various angles of incidence and report that in most
situations the best accuracy and precision is seen when the probing method is
applied, with the ellipse or lead-in method performing better only at the lowest
angles of incidence. No magnitude was obtained. Santangelo, Liscio and Nugent
([10.1080/00085030.2023.2169478](https://doi.org/10.1080/00085030.2023.2169478),
2023) measured the probing method on drywall; that study already has a row in
`ellipse-accuracy.md` and its finding is not restated here.

**Failure mode.** The method needs a channel, so it is unavailable exactly where
the material is thin and there is no second impact site. Keldson and Liscio
([10.1111/1556-4029.15523](https://doi.org/10.1111/1556-4029.15523), 2024) state
that as the reason the other methods exist: probing with trajectory rods is not
appropriate where there is a single thin target material or no secondary bullet
impact site. Where a rod is forced into a channel that does not support it, the
survey obtained no figure for how far the reading moves, and that absence is the
finding.

## The ellipse method

**What it measures.** The angle between the trajectory and the surface, from the
ratio of the axes of the perforation, and an azimuth from the orientation of the
major axis on the surface.

**Reported error.** In `ellipse-accuracy.md`. Not repeated here.

**Where it stops, relative to the others.** Two limits were read that belong in a
comparison rather than in the accuracy file. Liscio and Park
([10.1016/j.forsciint.2021.110914](https://doi.org/10.1016/j.forsciint.2021.110914),
2021) state that the ellipse method has been shown to have challenges when the
bullet impact site is highly deformed, which is the case on thin metal panels and
is where they propose the lead-in method instead. Mattijssen and Kerkhoff, above,
rank it below probing except at the lowest angles of incidence.

**Failure mode.** As the impact approaches normal the perforation approaches
circular and the derived angle becomes arbitrarily sensitive to the measurement.
That is recorded in `ellipse-accuracy.md` and decided in
`../decisions/0011-degenerate-cases.md`; it is named here so that a reader
comparing methods sees that this failure is specific to the ellipse and does not
apply to probing.

## The lead-in method

**What it measures.** The direction, from the lead-in mark: the region at the
edge of the impact where the projectile was in contact with the surface before it
perforated. Liscio and Park (2021) is the study that defines the procedure used in
the later work.

**What it assumes.** That a lead-in area exists, that it is large enough to be
measured, and that the operator can identify where it starts and stops. The area
exists because the projectile struck obliquely, so the assumption is an
assumption about the angle.

**Angle range.** Liscio and Park state that the lead-in method is useful only over
lower ranges of impact angles, and that the range of errors is greater at higher
angles of incidence where the lead-in area is relatively small. Keldson and Liscio
(2024) report the same direction on vehicle metal: the lower the impact angle, the
fewer errors the participants made, and as the impact angle increased the
measurement errors increased because of the smaller lead-in area.

**Reported error.** Liscio and Park tested five calibres, each with two ammunition
types, with 15 blind participants, and report that each calibre and ammunition
combination has its own characteristic error curve which changes with the known
impact angle, that errors are not constant, and that in some cases they exceed 20
degrees. Keldson and Liscio tested three calibres with three blind and two
non-blind participants on a typical vehicle metal surface and report errors
ranging from as low as 1 degree to as high as 13.9 degrees, with each calibre
having its own characteristic error curve, and no significant effect of blind
against non-blind participation.

**Failure mode.** There is no single error figure to carry. The error is a curve
per calibre and ammunition combination, and a tool that took one number from one
of these studies and applied it to a scene with different ammunition would be
asserting a precision the source does not support. This is the reason the method
is deferred below rather than the reason it is bad.

## A channel through two separated surfaces

**What it measures.** Two defects from one shot, on surfaces separated in space,
define a line through both. In the field this is a vehicle with an entry and an
exit panel, a window and a wall behind it, or an intermediate target and the final
one. Mattijssen and Kerkhoff describe the two available bases for an estimate in
exactly these terms: the spatial relation between a primary and a secondary bullet
defect, or the shape and dimensions of the primary defect alone.

**What it assumes.** One shot for both defects, and a straight flight between
them. The second assumption has two known ways of failing, and both were found.

The first is deflection at the first surface. Nishshanka, Shepherd and Ariyarathna
([10.1111/1556-4029.14717](https://doi.org/10.1111/1556-4029.14717), 2021) report
a deviation phenomenon for 7.62 by 39 mm bullets perforating 1 mm sheet metal, and
state that it introduces a potential error when probing, stringing or laser
methods are used on that projectile and substrate combination. Hirakawa, Saimoto
and Ishimatsu ([10.1111/1556-4029.13060](https://doi.org/10.1111/1556-4029.13060),
2016) is a case of the same thing at the extreme: on an automobile windshield with
a .38 calibre LRN bullet, an incidence angle below 45 degrees gave a complete
perforation and above 60 degrees gave a ricochet, so the first surface decides
whether there is a second defect at all.

The second is gravity. Kerkhoff, Broekhuis, Mattijssen and Riva
([10.1111/1556-4029.15431](https://doi.org/10.1111/1556-4029.15431), 2024) put
distances on the straight-line assumption; that study already has a row in
`ellipse-accuracy.md` and its two threshold distances are not restated here.

**Reported error.** None obtained. **No study was found that reports the angular
precision of a two-point direction as a function of the separation between the two
defects**, which is the figure this project would need to weigh a two-point
constraint against an ellipse constraint. The geometry says the angular error
falls as the separation grows and the position error stays fixed, but that is an
inference from the geometry and not a measurement, and it is written here as an
inference.

**Failure mode.** Two defects that are not from the same shot produce a confident
line through nothing. Nothing in the geometry detects it; it is detected by the
rest of the scene disagreeing, which is an argument for the tool reporting a
contradiction rather than a region.

## Ricochet geometry

**What it measures.** Two different things, and they should not be run together.
A ricochet mark on a surface constrains the direction the projectile came from.
Separately, the critical angle for that projectile and material bounds whether a
ricochet could have happened at all, so a mark identified as a ricochet is itself
a statement about the incoming angle.

**What it assumes.** That the mark has been correctly identified as a ricochet
rather than a graze or an impact, and that a published critical angle exists for a
projectile and material pair close enough to the scene to be used.

**Reported figures.** These are critical angles, not errors of a reconstructed
direction, and the distinction is the point.

| Study | Year | DOI | Material | Projectile | Critical angle reported |
| --- | --- | --- | --- | --- | --- |
| Mattijssen, Pater and Stoel | 2016 | [10.1111/1556-4029.13201](https://doi.org/10.1111/1556-4029.13201) | plain float glass | .32 Auto FMJ; 9 mm Luger FMJ; .45 Auto FMJ; 9 mm Luger Action NP | 21.0, 15.8, 17.6 and 21.3 degrees respectively |
| Kerkhoff, Alberink and Mattijssen | 2015 | [10.1111/1556-4029.12738](https://doi.org/10.1111/1556-4029.12738) | Abachi, Southern Yellow Pine, Beech and Ipe wooden boards | .32 Auto and 9 mm Luger | 10.4 and 10.3 degrees on the lightest and softest wood, 45.0 and 33.4 degrees on the heaviest and hardest, with a strong linear relationship to both density and Janka hardness |
| Nishshanka, Shepherd, Jayawickrama and Ariyarathna | 2023 | [10.1111/1556-4029.15180](https://doi.org/10.1111/1556-4029.15180) | glazed ceramic floor and wall tile | 9 mm Luger pistol | 14.8 degrees for floor tile and 16.6 degrees for wall tile |
| Hirakawa, Saimoto and Ishimatsu | 2016 | [10.1111/1556-4029.13060](https://doi.org/10.1111/1556-4029.13060) | automobile windshield | .38 calibre LRN | perforation below 45 degrees of incidence, ricochet above 60 degrees, from both firing tests and a rigid-body simulation |

All four rows are abstract only.

**Failure mode.** The outgoing direction is not the incoming direction. Mattijssen,
Pater and Stoel state that mean ricochet angles are always lower than the
corresponding angles of incidence, and that they differ depending on the state of
the ricocheted bullet: lower for full metal jacket bullets with undamaged jackets
than for bullets whose jacket was damaged or which partially ricocheted and
partially perforated. A reconstruction that reads the mark's own geometry as the
incoming direction is therefore wrong by an amount that depends on the material,
the projectile, and a property of the bullet after the event that the scene may not
record. No study read here reports that difference as an error distribution usable
for propagation.

## A trajectory rod read by an instrument

This is not a way of deriving a direction. It is a way of recording one that a
method above has already produced, and it is in this file because the field treats
it as a method and because its numbers are the ones most often quoted.

**What it measures.** The orientation of a physical rod, read by a 3D laser
scanner or a total station, expressed as a vertical and an azimuth angle.

**Reported error.** Liscio, Guryn and Stoewner
([10.1111/1556-4029.13719](https://doi.org/10.1111/1556-4029.13719), 2018)
examined laser scanning of trajectory rods in drywall for angles between 25 and 90
degrees. They report an inherent error range of 0.02 to 2.10 degrees and an
overall error for laser scanning between 0.04 and 1.98 degrees; inter-observer and
intra-observer variation for rod placement between 0.1 and 1 degree in drywall and
between 0.05 and 0.5 degrees in plywood; and virtual trajectory marking accuracy
with 75 per cent of values below 0.91 degrees on azimuth and 0.61 degrees on
vertical. Greenwood, Paduch and Allen
([10.1111/1556-4029.15230](https://doi.org/10.1111/1556-4029.15230), 2023) report
a figure for the same operation under an accreditation requirement; that study
already has a row in `ellipse-accuracy.md` and its figure is not restated here.
Both are abstract only.

**Failure mode, and it is the important one in this file.** These instrument
figures are around one degree. The method figures above are between several
degrees and more than twenty. **An instrument uncertainty quoted for a
reconstruction whose direction came from a lead-in mark or an ellipse understates
the real uncertainty by an order of magnitude**, and it is the number most easily
available, because the scanner reports it and the method does not. A tool that
accepts a direction with an uncertainty attached has to be able to record which of
the two the operator is giving it.

## What was looked for and not found

Nothing was dropped for being unreachable. This is what was searched for and not
obtained.

The full text of every study above, for the reason and with the evidence in the
first section.

An error figure for a two-point direction as a function of the separation of the
defects, as described in that section.

The stringing and laser practice as an estimator in its own right. The searches
returned it only as a documentation route, as measured in the rod section, and as
a thing the deviation phenomenon degrades. Whether there is a body of work
treating it as a method with its own error was not established, and the negative
here is weak: it is the absence of a result from these searches on these indexes,
not a demonstration that no such work exists.

The angle convention behind the reported ranges, per study, as described in the
first section.

Any study reporting how often a mark is misidentified as a ricochet, which is the
error rate that would matter for the assumption in that section.

## What the first version should support, and what it defers

This list is the input the model boundary decision in milestone 2 asked for. It is
a proposal from this reading and not the decision. The record is where it is
settled, and where this list is wrong, the record says so and this file stays as
it is.

**Support: the ellipse method.** It is the only method in this survey for which a
per-material reading of the measured error exists in this repository, and the
uncertainty model is being built on its relation.

**Support: two defects on one trajectory, as a geometric constraint.** It is the
best performing method in the one comparative study read, and it enters the tool as
two measured points with their own uncertainties rather than as a second estimator
that would need its own error model.

**Support: a direction the operator supplies with its own stated uncertainty.** It
costs one input form, and it lets a scene carry a constraint produced by a method
this tool does not implement, with the operator's own number on it, instead of the
tool refusing the scene. The rod section is why this input has to record where the
uncertainty came from.

**Defer: the lead-in method.** The reported error is a curve per calibre and
ammunition combination rather than a figure, it is not constant across the angle
range, and in some cases it exceeds 20 degrees, so no defensible uncertainty for it
can be read out of what was obtained here.

**Defer: ricochet geometry as a direction estimator.** What the literature supplies
is critical angles for particular projectile and material pairs and a statement
that the ricochet angle is systematically below the incidence angle, which is not
an error model for a reconstructed direction.

**Defer, but keep in view: the critical angle as a consistency check.** A
reconstruction that requires a ricochet at an angle below the published critical
angle for that material is making a claim the material contradicts. That is a
refusal the tool could make later on published figures alone, and it needs no
error model, so it is cheap and it is not in the first version only because the
scene model does not yet carry materials.

**Not a method, recorded so it is not read as one: near-normal incidence.** The
degeneracy there is a property of the ellipse relation and is already decided in
`../decisions/0011-degenerate-cases.md`.
