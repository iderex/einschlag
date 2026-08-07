# What an operator measures at a scene, and with what precision

This project's input is somebody else's measurement. Its uncertainty model is
worth nothing if it invents the input precision instead of reading it off the
instruments actually used.

Two things are separated throughout. An instrument's own quoted precision, which
is what its documentation says it can do under the conditions the documentation
names. And the precision of the quantity as an operator actually produces it at a
scene, which is worse and is sometimes not a measurement at all. The second is
what this project has to model, and it is the one the sources are thinnest on.

## What is required to be recorded

The standard that governs this is read in `standards.md`. In short, ANSI/ASB
Standard 196, 1st Ed., 2026 requires each projectile impact to be documented with
photography, labeling, measurements and location; requires its size as width and
length; requires the target surface material, contour and angle; and requires the
impact location to be measured in a clearly defined and recorded coordinate
system. For a trajectory it requires directionality, impact sites and angle
measurements when practicable, and it states that angles are commonly reported to
a whole degree. It requires no uncertainty on any of it.

The equipment list in clause 6 of that standard is the sourced list of what is in
use: cameras, trajectory rods and probes with centering cones, lasers, strings,
zero-edge protractors, inclinometers and angle finders, plumb bobs, 3D scanners,
total stations, laser measurement tools, micrometers, levels, calipers, tripods,
compasses, tape measures and scales, chemical reagents, and a carpenter's square.

## One row per measured quantity

### Hole centre position in the scene frame

Instruments in practice, from the ASB 196 list: steel tape measure with a
carpenter's square and a level, laser distance meter, total station, 3D laser
scanner, photogrammetry from photographs.

Steel tape. Tapes sold in Europe carry an EC accuracy class. Class I has a
permitted deviation of a + bL with a = 0.1 mm and b = 0.1 mm for L in metres,
giving 0.2 mm at 1 m, 0.6 mm at 5 m and 1.1 mm at 10 m. Class II has a = 0.3 mm
and b = 0.2 mm, giving 0.5 mm at 1 m, 1.3 mm at 5 m and 2.3 mm at 10 m. Source
<https://www.richter-messzeuge.de/en/products/measuring-tapes/accuracy-table-according-to-ec-class-i/>
and
<https://www.richter-messzeuge.de/en/products/measuring-tapes/accuracy-table-according-to-ec-class-ii/>.
These are the tolerances of the tape as an instrument. They are not the error of
a position taken off a wall with a tape, which also carries the operator's
placement of the zero end, the alignment of the tape, sag, and the definition of
where the centre of a ragged hole is. No source was found for that combined
figure.

Total station. Leica Viva TS16, taken as representative because it is a common
instrument in this role. Distance measurement to a prism, single measurement,
1 mm + 1.5 ppm, typically 2.4 s. To any surface without a prism, single
measurement, 2 mm + 2 ppm, typically 3 s. Both quoted as a standard deviation to
ISO 17123-4. Angular accuracy is offered in 1 in (0.3 mgon), 2 in (0.6 mgon),
3 in (1 mgon) and 5 in (1.5 mgon) variants, quoted as a standard deviation to
ISO 17123-3. Source
<https://surveyorssource.com/wp-content/uploads/2024/01/Leica_Viva_TS16_DS_en.pdf>.

3D laser scanner. Trimble X7, taken as representative for the same reason. Range
0.6 m to 80 m. Range accuracy 2 mm, given as one sigma, on a matte surface at
normal angle of incidence. Range noise under 3 mm at 60 m on 80 per cent albedo.
The automatic level compensator works over plus or minus 5 degrees with an
accuracy under 3 in, which the datasheet expresses as 0.3 mm at 20 m. Source
<https://www.buildingpointmwgc.com/wp-content/uploads/2024/04/trimble-x7-datasheet.pdf>.

### Major and minor axis lengths of the perforation

Instruments in practice: calipers, a scale in a photograph, a measurement taken
off a 3D model.

Calipers. Manufacturers state conformance to DIN 862. A published table of that
standard's maximum permissible errors gives 20 micrometres up to 50 mm of
measuring length and 30 micrometres at 100 mm, for calipers with a 0.01 mm or
0.02 mm reading value, with 20 micrometres to be added for inside and depth
measurements. Source
<https://pdf4pro.com/view/din862-page-1-4-23faeb.html>. The standard itself was
not obtained; this is a third-party reproduction of its table and is cited as
such.

The instrument precision is not the interesting number here and quoting it alone
would be misleading. A caliper reads to hundredths of a millimetre, and the
question a hole poses is where the edge of the ellipse is on a torn, deformed,
petalled perforation. That judgement is the dominant term and no source was found
that quantifies it. The ellipse-accuracy survey is where its consequences appear:
the studies there report the end-to-end error of the angle derived from these two
lengths, not the error of the lengths themselves.

Note also that this quantity is the one the whole method is most sensitive to.
The axis ratio enters through an arcsine, so near a normal impact, where the
ratio approaches one, a small error in either length moves the derived angle a
long way. That sensitivity is geometry rather than metrology and it belongs in
milestone 5, but it is why this row cannot be left with an instrument figure and
nothing else.

### Orientation of the major axis on the surface

Instruments in practice: a protractor or a square against a reference direction
on the surface, or a measurement taken off a photograph or a 3D model.

No precision figure was found from any instrument documentation for this
quantity, and it is not one the ASB 196 equipment list separates out. What is
known is the failure mode rather than the number: as the impact approaches normal
incidence the perforation approaches circular and the major axis stops being
defined, so the orientation is not merely imprecise but absent. The measurement
therefore has to be able to say "not determinable" and not only "plus or minus
something", which is a requirement on the input format rather than on the
instrument.

### Surface orientation

Instruments in practice, from the ASB 196 list: inclinometer or angle finder,
zero-edge protractor, level and plumb bob, or derived from a total station or
scanner survey of the surface.

No manufacturer precision figure was obtained for a forensic inclinometer or a
zero-edge protractor on this route. The one figure that exists is a convention
rather than a precision: ANSI/ASB 196 clause 4.4.3 a) 1) states that angles are
commonly reported to a whole degree.

In practice this quantity is often not measured at all. A wall is taken as
vertical and a floor as level, and the surface angle is recorded as 90 degrees
because that is what the building is supposed to be. That is an estimate, and the
distinction matters: a wall that is out of plumb by a degree moves every direction
derived from a hole in it by a degree, in a direction nothing in the data
reveals.

### Positions of scene reference points

Instruments in practice: total station, 3D laser scanner, tape and offsets from
building features, photogrammetry.

The instrument figures are the total station and scanner figures given above.
What is not covered by them is registration: a scene documented from several
scanner stations, or a model built from photographs, carries a registration error
between stations on top of the per-point figure, and that error is what a
reconstruction combining holes on opposite sides of a room actually rides on. No
figure for it was obtained.

### The quantity nobody measures directly: the trajectory angle itself

Not a row in the sense above, but the number that bounds all of them, so it is
recorded here. Greenwood, Paduch and Allen, 2023,
[10.1111/1556-4029.15230](https://doi.org/10.1111/1556-4029.15230), determined an
overall measurement uncertainty of plus or minus 2.6 degrees at an approximate
95 per cent confidence interval for trajectory angles measured from trajectory
rods with a 3D laser scanner, over seven substrates with 9 mm and .45 ammunition.
Abstract only; the full text was not obtained.

That figure is worth holding next to the scanner's own 2 mm range accuracy. The
instrument is three orders of magnitude better than the answer it is used to
produce, because the error lives in the rod, the substrate and the hand that
placed them, not in the ranging.

## Quantities that are estimated rather than measured

Each of these needs a wider distribution than a measured input, and the model has
to be able to tell them apart.

Surface orientation taken from the assumption that a wall is plumb and a floor is
level, as above.

The material of the target, which is identified by eye and by name rather than
measured, and which the ellipse-accuracy survey shows is what the error of the
whole method depends on.

The degree of deformation of the perforation, which is a judgement.

Which holes belong to one projectile, which ANSI/ASB 196 clause 4.3.2 records as
an operator's association of impacts with a single path, expressed as a labelling
convention. It is a declaration, not a measurement, and nothing in the data
verifies it.

Whether several shots came from one position, which is an assumption about the
incident rather than an observation of the scene.

## Photogrammetry, and the figures that exist for it

Photogrammetry is in the ASB 196 workflow implicitly through cameras and 3D
scanners, and it is the route most available to an operator with no survey
equipment. Two measured figures were found, both for camera position rather than
for hole geometry, and both are recorded with that limit stated.

Fung and Liscio, 2026,
[10.1111/1556-4029.70298](https://doi.org/10.1111/1556-4029.70298), reconstructed
body-worn camera positions from video with photogrammetry software against a
laser scanner ground truth and report average errors of 7.6 cm, 10.1 cm and
9.5 cm for three camera models, a total average error of 9.1 cm with a standard
deviation of 4.8 cm, and camera position estimable within a 24 cm radius circle
accounting for 98.7 per cent of errors. The abstract notes accuracy can decrease
in low-light or low-texture conditions. Abstract only.

Chen and Liscio, 2026,
[10.1111/1556-4029.70283](https://doi.org/10.1111/1556-4029.70283), compared
photogrammetric reconstructions from body-worn camera video against a laser
scanner at 12.48 m, 2.42 m and 0.24 m and report a maximum mean error of 14.42 cm
at 720P at the long distance. Read as a search-index abstract summary rather than
from the publisher, which is a weaker source than the others in this file and is
marked as such.

Neither of these measures the precision of a hole dimension read off a
photogrammetric model, which is the number this project would actually want. It
was not found.

## What the input format must be able to carry an uncertainty for

Every quantity below can be measured to a very different precision depending on
what was in the operator's hands, and in several cases is not measured at all.
The input format in milestone 4 must therefore be able to carry a stated
uncertainty for each of them, rather than assuming one:

- the position of the hole centre, in all three coordinates, since a tape and a
  total station differ by more than an order of magnitude here;
- the major axis length and the minor axis length, separately rather than as a
  single figure on the ratio, since the two are not measured to the same
  precision on a torn hole;
- the orientation of the major axis on the surface, with a representation that
  can express "not determinable" rather than a large number;
- the orientation of the surface the hole is in, with a way to mark it as assumed
  rather than measured;
- the positions of scene reference points, and separately the registration error
  between parts of a scene documented from different stations.

Two further things the format has to carry are not uncertainties but flags, and
they are here because the model has to see them: whether a quantity was measured
or estimated, and which instrument or method produced it. An estimated 90 degrees
and a measured 90 degrees are the same number and a different input.

## What is not covered

No figure was obtained for a forensic inclinometer, a zero-edge protractor, or a
trajectory rod as an instrument. No figure was obtained for the operator error on
top of any instrument figure, for any quantity. No figure was obtained for the
precision of a hole dimension read from a photogrammetric model or a scan rather
than with a caliper. No figure was obtained for scanner registration error across
stations. The DIN 862 table here is a third-party reproduction and the standard
was not obtained. The instrument figures are from two manufacturers' datasheets
for two instruments, chosen as representative, and a scene may have been
documented with anything on the ASB 196 list or with nothing on it.
