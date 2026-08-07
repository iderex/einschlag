# Measured accuracy of the ellipse method, by material and angle

The ellipse method estimates the angle of incidence of a bullet from the shape of
the perforation it leaves. A round projectile striking a flat surface at an angle
leaves an approximately elliptical hole, and the ratio of the minor axis to the
major axis is taken as the sine of the angle between the trajectory and the
surface. This project's uncertainty model rests on that relation, so the error of
the relation has to be read off published measurements rather than assumed.

This file is a reading. It states what each study measured and what it reported.
It does not derive an uncertainty model from them; that is milestone 5.

## How these entries were obtained

Bibliographic fields were taken from Crossref, and abstracts from Europe PMC,
PubMed or the Semantic Scholar graph API. The publisher pages for every study
below refused the request used here: Wiley returned HTTP 402, ScienceDirect and
Taylor and Francis returned HTTP 403. No full text was obtained for any entry in
this file. Every row therefore reads "abstract only", and where a field says the
value is not in the abstract, it means the value was not obtained, not that the
study failed to report it. The numbers a full text carries, in particular the
per-angle error tables, are exactly the numbers this project most needs, and they
are outstanding.

## Studies reporting on the ellipse method

| Study | Year | DOI | Material | Calibre | Angles tested | Sample size | Reported error and how it was expressed | Full text or abstract |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Mattijssen and Kerkhoff | 2016 | [10.1016/j.forsciint.2016.03.039](https://doi.org/10.1016/j.forsciint.2016.03.039) | drywall, MDF, sheet metal | not in the abstract | not in the abstract, described only as a range of angles of incidence | not in the abstract | No numeric error was obtained. The abstract reports a ranking: probing gives the best accuracy and precision for most target materials and angles, and the ellipse or lead-in method performs better only at the lowest angles of incidence. | abstract only |
| Walters and Liscio | 2020 | [10.1111/1556-4029.14309](https://doi.org/10.1111/1556-4029.14309) | drywall | four calibres, of which 0.45 is named as the best performing; the other three are not named in the abstract | 11 angles of incidence, decreasing until ricochet; the endpoints are not in the abstract | 220 shots, being 4 calibres by 11 angles by 5 repetitions, assessed by 31 participants | No numeric error was obtained. The abstract reports that accuracy and repeatability were better below 64 degrees of incidence for all calibres, and best for the 0.45 calibre ammunition. | abstract only |
| Liscio and Imran | 2020 | [10.1016/j.forsciint.2020.110504](https://doi.org/10.1016/j.forsciint.2020.110504) | thin sheet metal | 9 mm, six ammunition types | seven angles from 90 to 14 degrees | not in the abstract; the design is six ammunition types by seven angles, the repetition count was not obtained | No numeric error was obtained. The abstract reports the error as a pattern rather than a magnitude: a significant quadratic relationship between known and calculated angle for three of the six ammunition types, and no quadratic fit for the other three. It states that deformation of the metal produces "considerable errors between the known and calculated angle" without quantifying them in the abstract. | abstract only |

Three rows is a thin base for the most load-bearing assumption in this project,
and the thinness is the finding rather than an accident of searching. Two of the
three come from one research group, and the third is the only one that compares
the ellipse method against alternatives on more than one material.

## Studies read that are adjacent but are not ellipse-method rows

These were read while searching for the rows above. They measure the accuracy of
a different estimator, or of the instrument rather than the relation, on the same
substrates. They are listed so that a reader can see what was looked at and
rejected for this table, and because the ranking in Mattijssen and Kerkhoff only
means something against them. The other angle and direction methods are the
subject of a separate survey and are not worked here.

| Study | Year | DOI | What it measures | Material | Calibre | Angles | Sample size | Reported figure | Full text or abstract |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Santangelo, Liscio and Nugent | 2023 | [10.1080/00085030.2023.2169478](https://doi.org/10.1080/00085030.2023.2169478) | the probing method | drywall | .40 S&W from four manufacturers | 90 down to 10 degrees | 84 impacts, being three replicates per angle and ammunition type | The abstract states that the error of the measured value increased as the angle of incidence decreased, and that 60 to 90 degrees was more accurate than 10 to 45 degrees. No magnitude was obtained. | abstract only |
| Liscio and Park | 2021 | [10.1016/j.forsciint.2021.110914](https://doi.org/10.1016/j.forsciint.2021.110914) | the lead-in method | metal panels | five calibres with different ammunition types | not in the abstract | 15 blind participants; the impact count was not obtained | Each calibre and ammunition combination has its own error curve that changes with the known impact angle. Errors are stated as not constant and as exceeding 20 degrees in some cases, larger at higher angles of incidence. | abstract only |
| Greenwood, Paduch and Allen | 2023 | [10.1111/1556-4029.15230](https://doi.org/10.1111/1556-4029.15230) | the measurement uncertainty of a trajectory rod read by a 3D laser scanner | seven substrates, not named in the abstract | 9 mm and .45 | not in the abstract | not in the abstract | Plus or minus 2.6 degrees at an approximate 95 per cent confidence interval, for all trajectory angles measured from trajectory rods. | abstract only |
| Nishshanka, Shepherd and Ariyarathna | 2021 | [10.1111/1556-4029.14717](https://doi.org/10.1111/1556-4029.14717) | impact mark dimensions as a predictor of angle of incidence | 1 mm sheet metal | 7.62 by 39 mm | a range of angles of incidence, endpoints not in the abstract | not in the abstract | A strong inverse relationship between particular impact mark dimensions and the angle of incidence. No error magnitude was obtained. The abstract also reports a deviation phenomenon that introduces a potential error when probing, stringing or laser methods are used on this projectile and substrate combination. | abstract only |
| Kerkhoff, Broekhuis, Mattijssen and Riva | 2024 | [10.1111/1556-4029.15431](https://doi.org/10.1111/1556-4029.15431) | the systemic error a straight-line assumption carries, from gravity | not applicable, this is a flight-path study | 10 handgun and ammunition combinations | not applicable | not applicable, distances up to 100 m were modelled | Bullet drop and vertical offset stay below 5 cm and the drop angle below 0.3 degrees out to 20 m for subsonic and transonic handgun bullets and out to 30 m for supersonic ones. Those two distances are proposed as conservative thresholds for modelling a trajectory as a straight line. | abstract only |
| Nordin, Bominathan, Abdullah and Chang | 2020 | [10.1111/1556-4029.14142](https://doi.org/10.1111/1556-4029.14142) | a review of gunshot impact marks on inanimate objects | many | many | not applicable | not applicable | No error figure. The abstract states that impact marks vary with the combination of ammunition and surface material and that real conditions differ from controlled studies. | abstract only |

The last of these bears on the boundary of the model rather than on the ellipse
relation, and the two threshold distances in it are the kind of number the model
boundary decision needs.

## Studies that could not be obtained

Nothing was dropped for being unreachable. What follows is what was looked for
and not read.

The full text of every study in both tables above. The route used here reaches
Crossref, Europe PMC, PubMed and Semantic Scholar, and is refused by Wiley,
ScienceDirect and Taylor and Francis. Institutional access was not available on
this route.

The older literature that the reviews cite for the trigonometric relation itself,
including the work on wood pressboard, sheetrock and vehicle sheet metal that
secondary sources describe as the origin of applying the axis-ratio relation to
bullet holes. Secondary descriptions of it were seen in search results; no
primary record with a resolvable identifier was obtained, so it is not given a
row. A row built from a search snippet would be a citation this project has not
read.

## Materials for which no usable figure was found

A usable figure means a measured error for the ellipse method on that material,
tied to a stated range of angles. By that test, no material in this survey has
one, because no full text was obtained. Two materials have a direction of error
without a magnitude, from the abstracts: drywall, where accuracy and
repeatability are better below 64 degrees of incidence, and thin sheet metal,
where the error follows a per-ammunition pattern and where deformation is stated
to produce considerable error.

For the following materials nothing at all was found, neither a magnitude nor a
direction: wood and plywood, float glass, laminated glass such as a windshield,
plastics and composite panels, brick, concrete and other masonry, painted vehicle
body panels as distinct from bare thin sheet metal, corrugated and profiled
sheet, textiles and clothing, and soft tissue.

That list is what the material table in milestone 4 refuses on. A material with
no row in that table is a material this project has read no error for, and the
tool has nothing to offer about a hole in it.
