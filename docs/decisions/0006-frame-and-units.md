# 0006. The internal frame is right-handed with Z up and an operator-declared origin, the internal units are metres and radians, and a scene is a set of bounded planar surfaces

## Status

Accepted.

## Date

2026-08-08.

## The question

Every number this tool reads was produced in somebody else's frame, and the
mistakes here do not look like mistakes. A sign flipped between a source frame
and this one puts the shooter on the other side of a wall, and the region drawn
around that position is exactly as tidy as the correct one. Nothing downstream
catches it, because every later stage is arithmetic on numbers that are
internally consistent.

Four things have to be fixed before anything in milestone 4 can be written, and
they are fixed together because each one is meaningless without the others. The
frame: its handedness, which axis is up, where the origin sits, and how an angle
in it is stated. The unit the library works in, and which units are accepted at
the boundary. The way a scene is described. And what happens when the shape of a
surface contradicts what the hole measurement in it assumed.

Two pressures come from the field rather than from taste.

`../survey/field-practice.md` establishes what an operator actually arrives with:
a photograph with a scale in it, a sketch with tape measurements from a datum, a
compass bearing, a GPS position, sometimes a photogrammetric model, rarely a
total station or a laser scan. None of that is in this project's frame, and the
sketch is not in any frame that was written down.

`../survey/methods.md` establishes something worse. The published studies this
project's uncertainty model is built on do not consistently say whether an angle
is measured from the surface or from the surface normal, and several of them use
"angle of incidence" and "impact angle" for what appears to be the same quantity.
So the ambiguity is not only between an operator and this tool. It is inside the
literature the tool takes its error figures from.

## The options considered

### The frame

**A geodetic frame, latitude, longitude and height.** Places every scene on the
earth, lets two scenes be combined, and matches what a GPS position already is.
Cost: a geodetic datum, a projection, and a geoid model, none of which this
project has any business implementing, and all of which introduce their own error
into a reconstruction that is otherwise local geometry over a few tens of metres.
It also makes the common case, a room described with a tape, pay for the rare
case.

**A frame the operator declares in full, with no fixed convention, the tool
reading the handedness and the up axis out of the input.** Maximum flexibility,
and every source frame is representable. Cost: the convention becomes a field,
and a field is a thing that can be wrong. Worse, it moves the decision from this
record into every input file, so the question of what "up" means gets answered
once per scene by whoever was under the most time pressure.

**One fixed internal frame, with conversion at the boundary and the conversion
recorded.** The library has one convention and no branch on it. Cost: every
import path owes a transform, and the transform is a place a sign error can live.
That error is real, and it is concentrated in one place where a fixture can be
aimed at it rather than spread across the library.

**Z up or Y up.** Not a real argument about correctness, and it has to be decided
anyway. Y up is the convention of much graphics software, which is where a
photogrammetric mesh comes from. Z up is the convention of surveying and of the
instruments in the ASB 196 equipment list, and it is what a scene with a gravity
direction and a ground plane reads as.

### The units

**Internal metres, boundary metres only.** Simplest, and no conversion code
exists to be wrong. Cost: an operator working in inches converts by hand, outside
the tool, on paper, and that conversion is the one nobody checks. It also
excludes a large part of the intended audience for no gain, since the standard
this documentation is produced under is a United States standard.

**Internal metres, a closed set of boundary units, converted once at the
parser.** Cost: a conversion table, and a rule that has to be enforced rather
than trusted.

**Internal units carried as data, so a length knows what it is in.** Cost: every
arithmetic operation in the numeric core carries a unit check, or a unit library
carries it, which decision 0002 would have to pay for in a dependency. It buys
safety at the boundary that the boundary can buy for itself.

### The scene

**A triangle mesh throughout.** One primitive, and a photogrammetric model is
already one. Cost: a wall becomes a few hundred triangles, a hole is in one of
them, and the surface orientation a hole measurement depends on becomes a
property of a triangle rather than of the wall. Two holes in one wall can then
sit in facets with slightly different normals for a reason that is reconstruction
noise rather than the building.

**Bounded planar surfaces, with a mesh reduced to them on import.** Matches the
thing the measurement was actually taken against, which is a wall. Cost: a scene
that genuinely is not planar has to be approximated or refused, and the reduction
from a mesh is a step that can go wrong.

**Imported scan geometry used directly, with no scene model of this project's
own.** Cost: the tool's answer then depends on a point cloud nobody can check
line by line, which contradicts the reason decision 0007 chose a text input
format.

## The option taken

### The frame

A single internal frame. Right-handed, Cartesian, three axes.

- **+Z is up**, antiparallel to the local gravity vector. Up is not a convention
  in this project; it is the direction a plumb bob is not pointing, and the
  scene's ground plane and the shooter's own posture are consistent with it.
- **+Y is the scene reference direction**, declared by the operator. Where a
  compass bearing was recorded, it is magnetic or grid north as the operator
  states; where none was, it is any direction the operator names in the file, such
  as an edge of a building.
- **+X completes the right-handed set**, which places it 90 degrees clockwise
  from +Y seen from above. With +Y taken as north, +X is east and +Z is up.

The origin is a physical point in the scene, declared by the operator and
described in the input file well enough that a person standing at the scene could
find it again. There is no default origin and no implied one. A scene whose
origin is not described is a scene whose coordinates cannot be checked against
anything, and the description is the only part of this a reader on the other side
can verify.

The frame is scene-local. This project does not carry a geodetic datum, a map
projection or a geoid model in its first version, and it does not accept
latitude, longitude and height as scene coordinates. Where a source survey is in
a national grid, the importer reduces it to a scene-local frame and records the
source frame and the transform it applied.

### Angles

Two conventions are fixed, and they are fixed by giving the two readings
different names so that neither can be written where the other is meant.

**Azimuth** is measured in the horizontal plane, clockwise from +Y seen from
above, so +Y is 0 and +X is 90. This is the compass and surveying sense, and it
is the sense the worked example in decision 0007 already uses, where the north
wall's inward normal is written as 180.

**Elevation** is measured from the horizontal plane, positive upward, in the
range -90 to +90. It is not measured from any normal and not from the vertical.

**The angle of incidence** of a trajectory on a surface is the angle between the
trajectory and the plane of the surface, so a shot arriving perpendicular to a
wall has an angle of incidence of 90 degrees and a grazing shot approaches 0.
That is the reading under which the arcsine relation this project's model rests
on holds, as `../survey/ellipse-accuracy.md` states it: the ratio of the minor
axis to the major axis is taken as the sine of the angle between the trajectory
and the surface.

The angle between a trajectory and a surface normal is the complement of that,
and this project never names it. No key in the input format, no field in the
output artefact and no column in any table carries an angle measured from a
normal. Where a figure is taken from a study, the study's own convention has to
be established before the figure is used, and where the convention was not
established the figure is not used. `../survey/methods.md` records that for the
studies read so far it was not established, from abstracts alone.

The words "high angle" and "low angle" are not used in this project's own text
without saying what they are high and low with respect to, because the surveyed
literature uses them both ways.

### The units

The internal length unit is the **metre** and the internal angle unit is the
**radian**. Those are two units for two different dimensions rather than two
units for one quantity: there is exactly one internal unit for every quantity in
the library, which is what issue #34's boundary rule refers back to. The
internal numeric type is double precision.

At the boundary, a length may be written in `m`, `cm`, `mm`, `in` or `ft`, and an
angle may be written in `deg`. Every value carries its own unit, as decision 0007
requires, so a file may mix them and two holes in one file may be in different
units. Any other unit string is refused, naming the string that was found and
listing what is accepted. Radians are not accepted at the boundary: an operator
who wrote a scene angle in radians has almost certainly made a mistake, and the
one who has not loses nothing by writing degrees.

The conversion factors, and where each comes from:

| Unit | Metres | Source |
| --- | --- | --- |
| `m` | 1 | definition |
| `cm` | 0.01 | definition |
| `mm` | 0.001 | definition |
| `in` | 0.0254 | exact, see evidence |
| `ft` | 0.3048 | exact, 12 in |

Angles convert as `radians = degrees * pi / 180`.

Conversion happens once, in the parser and in the importers, and nowhere else.
The value that leaves the boundary is in metres and radians and stays that way
until it is formatted for output. Issue #34 owns the test that no
conversion factor appears outside the boundary, and this record is the thing that
test measures against.

### The scene

A scene is a set of **bounded planar surfaces**, an optional set of solid
obstacles, and an extent.

A surface is a plane with a bounded polygonal outline on it, an identifier, a
material, and a declared contour. The plane's orientation is given by the azimuth
and elevation of its outward normal, both carrying an uncertainty as any measured
value does, and the record is explicit that a wall assumed to be plumb carries an
interval rather than a standard deviation, because that is what decision 0007's
example already shows.

An obstacle is a closed solid the shooter could not have been inside. Whether an
obstacle narrowing a region is geometry or a prior, and what the tool has to say
when it applies one, is decided in `0008-priors.md` and not here.

The extent is the space the operator described, stated explicitly and never
inferred from the surfaces. Decision 0011 already depends on it: a region that
reaches the extent boundary carries a flag saying so, and the flag is meaningless
if the extent is something the tool made up.

A triangle mesh is not a scene. It is an import source, reduced to planar
surfaces on the way in, and what the reduction did is recorded so that a reader
can see how many triangles became one wall.

### When the surface is not the shape the measurement assumed

ANSI/ASB Standard 196 clause 4.3.4 requires the contour of the target surface to
be documented as flat, convex or concave, which `../survey/standards.md`
establishes by reading the standard in full. So the operator already has this
value, and the input format carries it.

A hole in a surface whose declared contour is not flat is **refused for direction
derivation**. The tool reads the hole, records it, states that it was not used and
why, and continues with the rest of the scene. It does not estimate an angle from
it and it does not widen an uncertainty to cover the curvature.

The reason is that widening requires a number, and there is none.
`../survey/ellipse-accuracy.md` finds no measured error figure for the ellipse
method on any material, and for curvature it does not even find a direction of
error. A tool that invented an inflation factor for a curved surface would be
producing a number whose only source is this project, in the one place where its
own survey says nothing is known. That is the failure mode the whole board exists
against.

The refusal is per hole and not per scene. A scene with one hole in a curved
panel and three in flat walls is reconstructed from the three, and the output
names the fourth and says it was excluded and for what reason.

Where the declared contour is flat and the imported geometry says otherwise, the
tool reports the discrepancy with both values rather than choosing between them.
It has no basis for deciding which of the operator and the scanner is wrong.

### The two exchange formats

Decision required by issue #15 and consumed by issue #35, which imports them.

**E57, specified by ASTM E2807.** Chosen because it is the vendor-neutral
standard the scanning route already emits, which means it is the format an
operator has without buying anything new for this tool.
`../survey/tools.md` establishes that Recon-3D, an iOS application using the
device's own LiDAR, outputs a point cloud in e57, and Recon-3D is in that survey
precisely because it is the cheapest way an operator can produce this kind of
data. An open reference implementation of the standard exists, which matters for
a project that will have to justify every dependency it takes.

**PLY, the Stanford polygon format.** Chosen because a photogrammetric model is a
mesh, `../survey/field-practice.md` establishes that a photogrammetric model is
what this project's least-equipped users are most likely to have, and a mesh of
polygons maps onto this project's planar surface primitive without a fitting step
in between. It is also small enough to implement without a dependency, and its
ASCII form can be read by a person, which is the same property that decided the
input format.

Two costs of that pair are stated here rather than discovered in milestone 4.

E57 carries points, and this project's scene is surfaces. Turning a point cloud
into a plane means fitting one, and a fitted plane has an uncertainty of its own
that has to be carried into the reconstruction rather than dropped. An importer
that fits a plane and reports it as exact would inject a false certainty at the
first step. That requirement belongs to issue #35 and is named here so that it is
not discovered late.

PLY carries no unit. The header declares the format and a version number and says
nothing about what one unit of the coordinates is; the specification page read for
this record shows the header line `format ascii 1.0` and a vertex element with
plain `x`, `y` and `z` properties. So a PLY import requires the operator to state
the unit and the reference direction, and the importer refuses rather than
assuming metres. This is the same refusal decision 0007 makes about a bare number,
arriving through a different door.

## The reasons

Z up rather than Y up, because this scene has gravity in it. A ground plane, a
plumb wall, a shooter standing on something, and a region that has to be
consistent with all three are physical facts, and the axis they are all about
should be the one every surveying instrument in the ASB 196 equipment list
already treats as vertical. The graphics convention would be right if the scene
were a model; it is a place.

Azimuth clockwise from +Y rather than the mathematical counter-clockwise from +X,
because the operator has a compass bearing in their notes and the tool should read
it without a transformation nobody thinks about. The cost is that the horizontal
angle convention and the right-handed axis convention have opposite senses, which
is a thing an implementer has to hold in their head. It is written here, and there
will be a fixture for it.

The angle of incidence measured from the surface, because that is the reading the
arcsine relation is stated under in the survey this project's model rests on.
Choosing the other reading would mean restating every figure that ever enters the
material table, and a restatement is a place a complement gets applied twice or
not at all.

A single internal unit, because the alternative is a unit check inside the numeric
core, and the numeric core is where decision 0009's determinism promise lives. The
fewer things happening per operation in there, the better.

Inches and feet accepted at the boundary, because ANSI/ASB Standard 196 is the
standard that governs this documentation and its audience does not work in
millimetres. Issue #34 already assumes it: its own fixture supplies one hole in
millimetres and one in inches in the same file.

Planar surfaces rather than a mesh, because the surface orientation is an input to
the reconstruction and the operator measured it once, for the wall. Letting each
triangle carry its own normal would give two holes in one wall two different
surface orientations, differing by an amount that came out of a reconstruction
algorithm rather than out of the building, and the difference would propagate into
the answer as if it were evidence.

Refusing a hole in a non-flat surface rather than widening it, because a widening
factor would be a number this project made up. The refusal is visible, it is per
hole, and it points at what would retire it, which is a measured error figure for
the ellipse method on a curved surface.

## What this costs

An operator whose scene is a curved car body, a helmet, or a corrugated fence gets
nothing from this tool for those holes. That is a real class of case, and vehicle
panels are named in `../survey/ellipse-accuracy.md` as one of the materials with
no measured figure at all. The cost falls on the operator, and it is the honest
form of a gap that the alternative would hide.

The refusal to accept geodetic coordinates costs an operator who documented a
scene with GNSS a conversion step, and the conversion is outside this tool, which
is exactly the position this record criticises for hand-converted units. The
difference is that a datum and a projection are a domain this project would
implement badly, and the manifest records that the reduction happened elsewhere.
It is a weaker answer than the unit case and is recorded as such.

Fixing the reference direction as an operator declaration rather than as true
north means two scenes cannot be combined without knowing both declarations. The
first version does not combine scenes, so nothing is lost today and something will
be if it ever does.

The two angle conventions having opposite senses will be got wrong at least once.
It is bought deliberately, against the operator reading their own compass bearing
without arithmetic.

Accepting five length units means five conversion factors, and a conversion factor
is a constant that can be typed wrong. The table above and the fixtures issue #34
already asks for are what stand against that.

## What would falsify this

An operator population that routinely documents scenes in a national grid and
finds the reduction step is where their errors come from. That would mean the
geodetic refusal moved the error rather than avoiding it, and this record would be
superseded by one that names a specific grid and carries the transform inside the
tool where a fixture can reach it.

Holes in non-flat surfaces turning out to be a large fraction of real cases. The
refusal is defensible while the case is uncommon; if it is the normal case, the
tool refuses most of its input, and the answer is a measured error figure for
curvature rather than a change of convention.

A published error figure for the ellipse relation stated from the normal rather
than from the surface, in a study whose full text makes the convention
unambiguous, that this project then has to use. That would not falsify the
convention, but it would show that the complement has to be applied somewhere, and
this record would gain a successor stating where.

A measured need for a unit not in the table, most plausibly a unit in the input a
foreign importer produces. That is a small supersession rather than a redesign.

Two holes in one physically flat wall producing inconsistent directions, at a
magnitude that a per-facet mesh normal would have explained. That would mean the
single-plane-per-wall reduction is throwing away real geometry, and the reasoning
above is wrong in the direction it was most confident about.

## What is not decided here

How the plane fitted from an E57 point cloud carries its uncertainty, which is
issue #35's to decide and is flagged above as owed.

Whether an obstacle is geometry or a prior, which is `0008-priors.md`.

The tolerance at which imported geometry is said to contradict a declared flat
contour. Fixing it needs a measured registration error, and
`../survey/measurement-practice.md` records that no figure for scanner
registration error across stations was obtained. The threshold will be fixed with
the command that produced it, in the issue that implements the importer.

## Evidence

The finding that the published studies do not consistently state whether an angle
is measured from the surface or from the normal is in `../survey/methods.md`,
which reached abstracts only and records that the publisher pages refused the
route used.

The statement of the arcsine relation as the sine of the angle between the
trajectory and the surface is in `../survey/ellipse-accuracy.md`.

The requirement that the contour of the target surface be documented as flat,
convex or concave is ANSI/ASB Standard 196, 1st Ed., 2026, clause 4.3.4, read in
full and recorded in `../survey/standards.md`. The same file records that the
standard requires the impact location to be measured in a clearly defined and
recorded coordinate system, clause 4.3.5.1, and requires nothing about the
uncertainty of any of it.

The equipment list that the up-axis argument leans on is clause 6 of the same
standard, reproduced in `../survey/measurement-practice.md`.

What an operator realistically arrives with, and the finding that a
photogrammetric model is the most available route for the least-equipped users, is
in `../survey/field-practice.md`.

That Recon-3D outputs a point cloud in e57 is in `../survey/tools.md`.

The inch is exactly 25.4 mm. Read from the NIST page "SI Units - Length" at
<https://www.nist.gov/pml/owm/si-units-length>, fetched on 2026-08-08, which
states that the new conversion factors were announced in 1959 in Federal Register
Notice 59-5442 of 30 June 1959, and that the value for the inch derived from the
value of the yard effective 1 July 1959 is exactly equivalent to 25.4 mm. The
Federal Register notice itself was not read on this route. The foot follows as
12 inches and is not separately sourced.

E57 is specified by ASTM E2807. Read from <http://www.libe57.org/>, fetched on
2026-08-08, which describes the E57 file format as a vendor-neutral format for
point clouds, images and metadata produced by 3D imaging systems, documented in
the ASTM E2807 standard, and describes libE57 as an open source implementation of
ASTM E2807 Standard Specification for 3D Imaging Data Exchange in C++. **The ASTM
standard itself was not obtained.** `https://www.astm.org/e2807-11r19e01.html`
returned HTTP 403 on the same route and date. So this record establishes that the
format is an ASTM standard and does not describe its contents, and an importer
will need the specification before it can be written.

The PLY header and its version line were read from
<https://paulbourke.net/dataformats/ply/>, fetched on 2026-08-08, which gives the
complete ASCII description of a cube beginning `ply` and `format ascii 1.0`,
annotates that line as "ascii/binary, format version number", names
`binary_little_endian` and `binary_big_endian` as the alternatives, and shows the
vertex element carrying plain `x`, `y` and `z` float properties with no unit
anywhere in the header. That page is a widely used description of the format
rather than a normative standards document, and it is cited as such.

No claim in this record about which conventions other software uses internally was
measured. The statement that graphics software commonly uses a Y-up convention is
general knowledge stated as an argument, not a measurement, and nothing in the
decision rests on it.

No performance figure appears in this record and none was measured.
