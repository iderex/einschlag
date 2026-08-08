# The existing tools, and what their output actually is

This project's stated gap is that established practice returns a line. That is a
claim about other people's software, and repeating it is not the same as checking
it. This file checks it.

One entry per tool. Each carries the name, the vendor or project, whether source
is available, the licensing cost where a public figure was found, what the tool
takes in, and what it emits. Where the output form could not be established from
public material, the entry says so instead of guessing.

The last section answers the question the survey was opened for.

## How these entries were obtained, and what that route could not reach

Three routes. A GitHub repository search for open source work, a fetch of vendor
and trade pages for the commercial packages, and Europe PMC for peer-reviewed
measurements of named tools.

    gh api -X GET search/repositories -f q="bullet trajectory" -f sort=stars -f per_page=6

    curl -s -G "https://www.ebi.ac.uk/europepmc/webservices/rest/search" \
      --data-urlencode 'query=ABSTRACT:"FARO" AND ABSTRACT:"trajectory"' \
      --data-urlencode 'format=json' --data-urlencode 'resultType=core'

Several vendor pages could not be read on this route, and the failures are listed
here rather than left as gaps in the entries. The FARO Zone 3D product page and two
FARO article pages returned a redirect loop and were not read. The Leica Map360 Pro
page returned HTTP 403 and was not read. A Leica Geosystems blog post and a trade
magazine article were fetched successfully and are quoted below.

**Nothing in this file is quoted from a search result summary.** Where a page could
not be fetched, the entry says the output form was not established rather than
carrying a sentence nobody read at its source. That distinction is the whole point
of the file: a claim about somebody else's software, made from the nearest thing to
hand, is the defect this project exists against.

Pricing is worse than sparse. None of these vendors publishes a list price. The one
figure below comes from a procurement document a public body published, and it is a
quotation to one customer on one date rather than a list price.

## FARO Zone 3D

**Vendor.** FARO Technologies Inc.

**Source available.** No.

**Licensing cost.** One public figure was found. FARO quotation number 02280272,
dated 2023-04-24, issued to the Overland Park Police Department in Kansas and
published in that city's public document portal, carries the line item SSA51007-3Y,
"SSA51007-FARO Zone 3D Expert", quantity 1, unit price USD 4,900.00, discount USD
0.00. The same quotation prices a FARO Focus Premium laser scanner at USD 41,860.00
and a SCENE licence at USD 5,500.00, which is the more useful number: the software
is a small part of what an operator has to buy to feed it. The document is at
`https://opkansas.civicweb.net/document/85804/FARO-Quotation%20No.%202280272.pdf`
and was read directly. It carries a "Confidential" footer while being published by
the public body that received it, and it is one quotation rather than a price list,
so it is evidence that the software is sold at around this figure and not evidence
of what it costs today.

**Input.** Laser scan point clouds, total station data, photogrammetry and drone
imagery. Established from the measurement study below rather than from the vendor
page, which could not be read on this route.

**Output.** A trajectory, as an azimuth angle and a vertical angle in the scene,
which the software can extend back through the scene. Liscio, Le and Guryn
([10.1111/1556-4029.14144](https://doi.org/10.1111/1556-4029.14144), 2020) measured
the accuracy and reproducibility of exactly these tools. Twelve participants were
given laser scanner data for 21 bullet trajectories on drywall at impact angles
between 25 and 90 degrees. With the impact plane aligned manually by the operator,
75 per cent of absolute errors were within 0.91 degrees for azimuth and 0.98 degrees
for vertical; aligning the plane to gravity, which removes the operator, improved the
vertical figure to 0.47 degrees. Abstract only.

**What that output is not.** The quantity the study reports is an angle with an
error, and the error is the software's own reproducibility rather than a
reconstruction of where the shooter could have been. Nothing read here shows the
software combining several trajectories into a region with a stated probability.

## Leica Map360

**Vendor.** Leica Geosystems, part of Hexagon, as part of the Incident Mapping Suite.

**Source available.** No.

**Licensing cost.** No public figure found on this route.

**Input.** Total station and GNSS data in the Standard edition and laser scanner
point clouds and photogrammetry in the Pro edition, according to the vendor blog
post below.

**Output.** A cone. A Leica Geosystems blog post dated 2020-07-01, describing a
county crime laboratory's use of the software, says that "the analyst simply clicked
on the rods to elongate them to 5-degree cones to better understand the bullet path".
That page was fetched and read. The Map360 Pro product page, which search results
indicate describes an error cone and a set tolerance, returned HTTP 403 and was not
read, so this entry does not carry that description.

**What that output is.** A fixed five-degree cone around a measured rod direction. It
is a region rather than a line, and its width is a convention rather than anything
derived from the measurement it was applied to. That distinction is what the closing
section is about.

## Trimble Forensics Reveal

**Vendor.** Trimble.

**Source available.** No.

**Licensing cost.** No public figure found. A software marketplace listing for the
product was fetched and carries no price.

**Input.** Scene survey data, drone data and a 3D model library, per that listing.

**Output.** Not established. The listing names "bullet trajectory analysis" among its
forensic analysis tools and describes no output form, and the vendor's own product
page was not read on this route. This entry stays at that.

## Recon-3D

**Vendor.** An iOS application, released 2022-05, using the LiDAR sensor in recent
Apple devices fused with photogrammetry.

**Source available.** No.

**Licensing cost.** No public figure found on this route.

**Input.** The device's own LiDAR and camera.

**Output.** A 3D point cloud in e57 format. This is a documentation tool rather than
a reconstruction tool, and it is in this file because it is the cheapest way an
operator can produce the data the other tools consume. Chase and Liscio
([10.1016/j.forsciint.2023.111787](https://doi.org/10.1016/j.forsciint.2023.111787),
2023) compared it against a FARO Focus S350 laser scanner for bullet trajectory
documentation, using twelve trajectory rods on a wooden panel indoors as a first
test. The comparison figures were not obtained; the abstract states the purpose and
the design, and the results section was not read.

## The manual rod, string and laser procedure

**Vendor.** None. This is the procedure a person carries out with a rod, a protractor
and an angle gauge, or with string and a laser, and it is in this file because
treating it as a tool is the only way to compare it against the software honestly.

**Source available.** Not applicable, and worth saying plainly: this is the only entry
in this file whose method is fully inspectable by the person using it.

**Licensing cost.** The cost of rods, gauges and a laser. No figure obtained.

**Input.** The physical scene.

**Output.** A line materialised in space, read off as two angles, or photographed
with the rod in place. Chase and Liscio, above, state that manual methods using a
protractor and angle gauge are quite common.

**Its error.** In `methods.md` and `ellipse-accuracy.md`, which is where the figures
for the underlying methods live. The relevant comparison for this file is that the
instrument reading of a rod is accurate to about a degree while the method that
placed the rod may be wrong by ten or twenty, so a tool that reports the first
number as its uncertainty reports the wrong one.

## The point mass model program in Riva and others, 2025

**Vendor.** None published. This is a computer program described in a study rather
than a product, and it is the most important entry in this file.

**Source available.** Not established. No repository or download was found for it on
this route.

**Input.** An impact point and the parameters of a long-range trajectory.

**Output.** An area. Riva, Broekhuis, Haag, Koene and Kerkhoff
([10.1111/1556-4029.15697](https://doi.org/10.1111/1556-4029.15697), 2025) state
that a computer program using the point mass model can perform long-range trajectory
reconstructions starting from an impact point, and that "the reconstruction results
in an area where the shot is expected to be fired from, not a single location", which
is caused by varying the input parameters of the model. They validated it against 20
handgun bullet trajectories measured by Doppler radar from 500 m to 1800 m, and
report that comparing the calculated area against the actual shooter position
demonstrates the limits of these reconstructions, particularly at high incident
angles. Abstract only.

**What was not established.** Whether the area carries a stated probability level, or
is the envelope of a parameter sweep with no level attached. The abstract says the
area is caused by varying the input parameters, which is consistent with either. It
also states that examiners usually deal with trajectories of 30 m or less, where a
linear model is appropriate, so how much this program's regime overlaps this
project's was not established either.

## Open source projects that reconstruct a trajectory

None found.

The GitHub repository search was run with the queries below on 2026-08-08 and the
totals are the API's own `total_count` for each:

| Query | Total repositories matched |
| --- | --- |
| `bullet trajectory reconstruction` | 0 |
| `shooting scene reconstruction` | 0 |
| `ballistic trajectory forensic` | 1 |
| `ballistics reconstruction` | 0 |
| `bullet hole angle` | 0 |
| `impact angle bullet hole` | 0 |
| `bullet trajectory` | 64 |
| `forensic ballistics` | 6 |
| `shooting incident` | 485 |

The one match for `ballistic trajectory forensic` is a student project combining
several unrelated forensic tasks, with no licence declared. The 64 matches for
`bullet trajectory` are game engine plugins, airsoft and shooting-sport ballistic
calculators, and similar; the top results were read and none reconstructs a shooter
position from scene measurements. The 485 matches for `shooting incident` are
overwhelmingly datasets and analyses of police shooting statistics, which is a
different sense of the phrase.

**This negative is bounded and the bound is stated.** It is the result of a
repository search on one host, over names, descriptions and readme text, with these
queries, on this date. Work that exists under a name none of these queries reaches,
inside a larger project, or on a host other than GitHub, would not appear. It is not
a demonstration that no such project exists.

## Does any surveyed tool already emit a distribution over positions

**No tool surveyed here was established to emit a probability distribution over
shooter positions.** The premise survives in that narrow form.

It does not survive in the form "established practice returns a line". Established
practice returns a cone. Fries, writing in LIDAR Magazine on 2023-07-22, states that
a window of error of five degrees must be accounted for when tracing a bullet from an
entry angle to origin, and describes experts tracing five-degree windows for several
shots fired in a tight pattern so that the cones grow, intersect, and the intersecting
section identifies where the shooter had to be. The Leica Map360 account above is the
same practice in a named package. That is a region, produced from several constraints,
and it is what the field already does.

What the five-degree cone is not is an uncertainty. It is the same five degrees on
drywall and on sheet metal, at 20 degrees of incidence and at 80, for an ellipse
measurement and for a rod pushed through a channel, and the measured errors in
`methods.md` run well past it in several of those cases. So the practice emits a
region whose width is a convention rather than a propagation of what was measured.

And one published program already emits an area rather than a location, in the Riva
entry above, produced by varying the inputs, which is the shape of what this project
proposes. Whether it carries a level was not established.

Both findings are the trigger this issue asked for rather than a detail, so they are
not resolved in this file. Issue #74 holds the re-examination of the premise. What
this file can say is what was read and what was not, and the sentence the premise
should not survive unchanged is the one about a line.
