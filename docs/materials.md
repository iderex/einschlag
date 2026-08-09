# The material table

The angle a perforation implies depends on what the perforation is in. Drywall,
sheet metal, glass and wood do not behave alike under the same projectile, so
the uncertainty this tool attaches to an estimated angle has to come from a
measurement made on that material rather than from a constant that looked
reasonable.

`data/materials.toml` is where those figures live. It is tracked data and not
constants in source, so that somebody checking a number against the reading in
`survey/ellipse-accuracy.md` opens one file and never has to read the code.
`crates/einschlag/src/materials.rs` reads it.

## The table holds no rows

That is the finding rather than an unfinished piece of work.

`survey/ellipse-accuracy.md` states that by the test of a measured error tied to
a stated range of angles, no material in it has a usable figure. No full text was
obtained for any study in it, so every row in that file reads "abstract only",
and two materials have a direction of error with no magnitude. Issue #76 carries
that gap, and it says what follows from it in as many words: a table that refuses
what it has no source for, sourced from a survey with no usable figure for any
material, refuses every material.

A row invented to make the table non-empty would be worse than no row, because
the empty table refuses honestly and a plausible number does not. The refusal is
what ships until a figure arrives.

## Whether a number read off a chart may source a row

`survey/full-text-2026-08-09-nishshanka-2021.md` obtained one full text and
handed this question here, because it is the first study in the survey where the
question is live rather than academic. That manuscript plots the mean difference
between the estimated and the known angle for the ellipse method against the
known angle, and it prints no table of those values. The record reads nine points
off the rendered figure at eight times scale against gridlines two degrees apart,
and says of them that they are figure readings and are not measurements this
project may quote as though the study had printed them.

They do not source a row. Three reasons, and the first is the one that decides
it.

The reading is not the study's number. A value taken off a plot carries the
reader's judgement about where a point sits, and a row in this table is quoted
into an output as a measured uncertainty. The gap between "the study printed 4.5"
and "somebody read about 4.5 off a chart" is exactly the distinction this project
exists to keep, and a row has no field in which to keep it.

What the study concludes is not an uncertainty magnitude. Its own sentences are
that the ellipse method is not viable for that bullet and target combination, and
that the probing method is the accurate one there. A row stating a standard
deviation would turn a negative result into a figure to compute with.

The manuscript is an accepted version rather than the published one, its own
sample size is stated inconsistently in three places, and the values wanted are
in a figure whose error bars at two angles reach past eight degrees. None of that
makes it a bad study; it makes those nine numbers the wrong thing to build a
material row out of.

What the reading is good for is written where it was made. It is evidence that
the ellipse relation behaves badly on one-millimetre sheet metal with that
ammunition, worst around forty degrees rather than at the shallow end, and it is
a reason to expect the first row that ever lands in this table to be a
restriction rather than a figure.

## The shape of a row

Five keys, all required. A row missing any of them is refused when the table is
read, and so is a row whose citation is empty.

```toml
format_version = 1

[[material]]
name = "gypsum-wallboard-12mm"
valid_from_deg = 40.0
valid_to_deg = 90.0
angle_sd_deg = 2.6
citation = "docs/survey/ellipse-accuracy.md, the row for <study>, <what it printed>"
```

`name` is what a hole in an input file names. It is compared exactly, so a
material is one string and not a family of spellings.

`valid_from_deg` and `valid_to_deg` are the range of angles of incidence the
figure was measured over, in degrees, and the row says nothing outside them. The
lower bound has to be below the upper one, and the range has to sit inside zero
to ninety degrees, because an angle of incidence is measured from the surface.

`angle_sd_deg` is the standard deviation the material contributes to an estimated
angle, in degrees. It has to be above zero. A row contributing nothing would
assert that the relation is exact on that material, which no reading supports and
which would narrow an answer for free.

`citation` says where in the survey the figure was read. It is the guard that
keeps the table from filling up with plausible numbers, and it is refused when it
is missing and when it is present and empty.

## Degrees, and why nothing is converted here

The survey prints degrees, the table states degrees, and the key names carry the
unit so that a reader checking a row against a publication compares like with
like. The internal unit of the library is the radian, which decision 0006 fixes.

No conversion happens in this module. Where a caller holds an angle in the
internal unit, the conversion belongs at the one boundary issue #34 is about, and
a second conversion site here is the thing that issue exists to prevent. Nothing
consumes the table yet, so there is nothing to convert for today.

## What happens to a hole the table cannot answer for

There is no default and there is no fallback. A hole whose material has no row,
or whose angle falls outside the range the row was read over, produces a refusal
naming the material and what is missing, and the caller gets no number at all.
The two cases are different values rather than different strings, so a caller can
tell them apart without reading text.

A default here would be an invented uncertainty handed to a reader as a measured
one. It would look exactly like a measurement in the output, which is why it is
absent rather than merely discouraged.

## What the reader does not do

It reads the narrow shape above and refuses everything else. A line that is not
blank, a comment, a `[[material]]` header or `key = value` is refused rather than
read past, and so is a section it does not know, a key no row has, a key written
twice, a second row for one material, and a text value that is anything other
than one pair of quotes around text with no escape in it. The cost of a narrow
reader is that a form it does not know could be read as nothing, so every form it
does not know is a refusal instead.

That is the same shape and the same argument as
`crates/einschlag/tests/dependency_budget.rs`, which reads the workspace
manifests without a library. A library to read this file would be the first
direct dependency, entered in the document that counts them. The input format an
operator writes is a different question and is decision 0007's; whether the table
moves to whatever reads that format belongs with the parser in #33 rather than
here.

The reader judges the shape of the table. It cannot judge whether a citation
points at something real, whether the figure beside it is the figure that
publication printed, or whether the range of angles is the range it was measured
over. Those are what a reader with the survey open is for.

## What would add a row

A measured error for the ellipse method on a material, tied to a stated range of
angles, read out of a study rather than off a chart or out of a secondary
description. Issue #76 is where the reading continues, and
`survey/full-text-acquisition-2026-08-08.md` records which routes have been tried
and what they returned.

For these materials nothing at all has been found, neither a magnitude nor a
direction: wood and plywood, float glass, laminated glass, plastics and composite
panels, brick, concrete and other masonry, painted vehicle body panels as
distinct from bare thin sheet metal, corrugated and profiled sheet, textiles and
clothing, and soft tissue.
