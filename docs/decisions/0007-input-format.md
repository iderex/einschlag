# 0007. The input is a single TOML file, versioned, with units and uncertainties on every measured value

## Status

Accepted.

## Date

2026-08-07.

## The question

The input file is evidence. It will be attached to a report, disclosed to the
other side, and read by somebody who did not produce it and does not have this
tool. It has to be checkable line by line against a measurement sheet by a person
with a pen, and it has to carry units and uncertainties explicitly rather than
positionally, because a field whose meaning depends on its position is a field
that gets misread in exactly the way this project exists to prevent.

Two things are decided here beyond the format itself. What happens when the file
declares a version the tool does not know, because a silently misread field is a
wrong reconstruction that nothing catches. And what happens to a hole whose
uncertainty was not stated, which `../survey/standards.md` establishes is not a
rare case but the normal one: the standard that governs this documentation
requires an angle to be recorded and requires nothing about its uncertainty.

## The options considered

**A binary format.** Compact, fast, unambiguous to a machine. Cost: unreadable
without this tool, which fails the requirement outright. Disclosure of a binary
input is disclosure of nothing a reader can check. Rejected before the others were
weighed.

**CSV or another tabular text format.** Familiar to anybody with a spreadsheet,
and a measurement sheet is already a table. Cost: meaning is positional, which is
the failure mode named above. Uncertainties and units either become more columns,
which makes a hole a row of twenty fields nobody can read across, or they go in a
header, which puts them somewhere other than the value they qualify. Nested
structure, which the scene needs, has no representation at all.

**JSON.** Universally implemented, unambiguous, easy to generate. Cost: no
comments, so an operator cannot write down which measurement sheet a value came
from or that a surface angle was assumed rather than measured. Punctuation-heavy
enough that a hand-written file is usually a syntactically invalid one. Numbers
have no way to carry a unit, so units become string fields beside them.

**YAML.** Readable, supports comments, widely known. Cost: the type coercion.
A value that looks like a number becomes a number, a bare word can become a
boolean, and the rules differ between implementations and versions. For a format
whose whole purpose is that a value means exactly what it says, a specification
in which the meaning of an unquoted token depends on the parser is the wrong
foundation. Significant whitespace also makes a hand-edited file fail in ways that
are hard to see on paper.

**TOML.** Text, line-oriented, comments supported, no significant whitespace,
explicit key names, unambiguous scalar types, one obvious way to write a table of
records. Cost: less universally known than JSON or CSV, and it adds a dependency
in whatever language is chosen, which decision 0002 will constrain.

**A format specific to this project.** Total control over what is expressible,
including making an uncertainty syntactically mandatory rather than a convention.
Cost: a parser nobody has fuzzed, a grammar nobody else implements, no editor
support, and a reviewer on the other side who has to learn a language to check a
measurement. The one thing it buys, syntactic enforcement of the uncertainty, is
available from a schema check over a known format at a fraction of the cost.

## The option taken

The input is a single TOML file.

Every measured value is written as a table with an explicit unit and an explicit
uncertainty, rather than as a bare number. The shape is fixed as three keys:
`value`, `unit` and one of the uncertainty forms below. A bare number where a
measured value is expected is refused.

The uncertainty is written in one of these forms, and exactly one of them is
present:

- `sd`, a standard deviation in the same unit as the value;
- `interval`, a two-element array giving a lower and an upper bound in the same
  unit, for a quantity known only to lie within a range;
- `unknown = true`, an explicit declaration that no uncertainty was established.

`unknown = true` is not a default and cannot be reached by omission. It has to be
typed, so that a file containing it is a file whose author decided to say so.

Every file declares `format_version` at the top level, an integer.

## The reasons

TOML was taken because it is the only candidate that satisfies all four
requirements at once. It survives being pasted into a document, because it is
plain lines with no significant whitespace. It can be checked line by line,
because every value carries its own key. It carries units and uncertainties beside
the value they qualify rather than in a header or a column position. And it
supports comments, which is how an operator records that a surface angle was
assumed rather than measured, which `../survey/measurement-practice.md` shows is a
distinction the model depends on.

The three-key value shape was taken over a compact string such as
`"42.0 mm +/- 0.5"` because a compact string needs its own grammar, its own
parser and its own error messages, and the format was chosen partly to avoid
exactly that. It is more verbose to write and the verbosity is what makes it
checkable.

The three uncertainty forms exist because the survey found three genuinely
different epistemic states in practice and collapsing them loses the distinction
this project is about. A quantity measured with an instrument has a standard
deviation. A quantity known only to lie between two values, which is what an
estimate off a photograph usually is, has an interval. A quantity for which
nothing was established has neither, and saying so is different from guessing a
number for it.

## What this costs

A TOML dependency in whatever language decision 0002 chooses. If that decision
produces a language with no maintained TOML implementation, this record is wrong
and is superseded rather than worked around with a hand-written parser.

Verbosity. A two-hole scene in this format is longer than the same scene in CSV by
a large factor. The cost falls on the operator writing the file, and it is
accepted because the reader who has to check it is the one this format is for.

Refusing a bare number costs an operator time on their first file, and it will
read as pedantry until the first time it catches a millimetre entered as a metre.

The cost of `unknown = true` falls on the operator too. They have to type it, and
typing it is a statement they may not want to make. That is the point, and it is
also the thing most likely to be worked around by an operator under pressure, by
typing a plausible standard deviation instead. Nothing in the format can prevent
that, and the output naming its assumptions is what limits the damage.

## What would falsify this

An operator population that in practice writes the file with a script rather than
by hand, in which case the readability that drove the choice is buying nothing and
a schema-checked JSON would cost less.

A measured quantity that none of the three uncertainty forms can express, and
`../survey/measurement-practice.md` already names a candidate: the orientation of
a major axis on a nearly circular perforation, which is not uncertain but absent.
If the degenerate-case record needs a fourth form, this record is amended by
supersession rather than by stretching `unknown`.

Files failing to survive the round trip through the document formats they are
actually pasted into. If the format is regularly corrupted in transit in a way a
different one would survive, the first requirement is not being met.

## Versioning and the refusal on an unknown version

`format_version` is an integer at the top level of the file. It is required. A
file without it is refused, with a message saying so; it is not treated as version
1, because a file predating the field and a file whose author forgot it are
indistinguishable, and guessing between them is the optimistic read this rule
exists to prevent.

The tool knows a set of versions. On a version outside that set the tool refuses
the file and stops. It does not attempt a partial read, it does not warn and
continue, and it does not read the fields it recognises and ignore the rest. The
refusal message states the version found, the versions this build accepts, and
the version of the tool, so that an operator meeting it knows whether they need a
newer tool or an older file.

The version increments when a change could alter the meaning of a file that
already parses. Adding an optional key that the tool ignores when absent does not
increment it. Changing the meaning, the default or the unit of an existing key
does, and so does making an optional key required.

## What is mandatory, what has a default, and the hole with no stated uncertainty

Mandatory, refused if absent:

- `format_version`;
- for each hole: its identifier, the surface it is in, the material, the position
  of its centre, the major and minor axis lengths, and an uncertainty declaration
  for each of those measured values;
- for each surface referred to by a hole: its definition in the scene;
- the extent of the described space.

Defaulted, with the default stated in the documentation and echoed into the run
manifest so that an output records which defaults were in force: the coordinate
frame conventions, and the confidence levels at which regions are reported.

Not defaulted, and this is the case that matters. A hole whose uncertainty was not
stated is refused. `unknown = true` is available and has to be written.

The reason for refusing rather than defaulting is that the two available
behaviours are not symmetrical in their failure. A default hides the assumption
inside the tool, and the operator learns nothing; the number that comes out is
narrow, and its narrowness comes from a constant nobody chose for their scene. A
refusal costs the operator a minute and forces them to decide, in the one place
where they know something the tool does not.

The refusal is not a dead end. `unknown = true` is always available, and the tool
accepts it and proceeds, treating the quantity as unconstrained rather than as
precise. `../survey/standards.md` establishes that the governing documentation
standard does not require an uncertainty to be recorded, so a file assembled from
a compliant scene record will frequently have nothing to state, and a tool that
refused those files outright would be unusable by the audience it is for. What is
refused is silence, not ignorance.

## A worked example

A two-hole scene, both holes in one wall, from one projectile.

```toml
format_version = 1

[scene]
name = "synthetic example, not a real case"
# Extent is stated explicitly and is not inferred from the surfaces below.
extent = { x = [0.0, 12.0], y = [0.0, 8.0], z = [0.0, 3.0], unit = "m" }

[[scene.surface]]
id = "wall-north"
# Assumed vertical from the building, not measured. Recorded as an interval
# rather than a standard deviation because nothing was measured to have one.
normal_azimuth = { value = 180.0, unit = "deg", interval = [178.0, 182.0] }
normal_elevation = { value = 0.0, unit = "deg", interval = [-2.0, 2.0] }

[[hole]]
id = "A1"
surface = "wall-north"
material = "gypsum-wallboard-12mm"
trajectory = "T1"
centre = { value = [4.210, 0.0, 1.480], unit = "m", sd = [0.005, 0.005, 0.005] }
major_axis = { value = 14.8, unit = "mm", sd = 0.6 }
minor_axis = { value = 9.1, unit = "mm", sd = 0.6 }
# Nearly circular holes make this meaningless; this one is not.
major_axis_bearing = { value = 63.0, unit = "deg", sd = 4.0 }
deformation = "moderate"

[[hole]]
id = "A2"
surface = "wall-north"
material = "gypsum-wallboard-12mm"
trajectory = "T1"
centre = { value = [4.905, 0.10, 1.395], unit = "m", sd = [0.005, 0.005, 0.005] }
major_axis = { value = 15.2, unit = "mm", sd = 0.6 }
minor_axis = { value = 9.4, unit = "mm", sd = 0.6 }
major_axis_bearing = { value = 61.0, unit = "deg", unknown = true }
deformation = "severe"
```

Two things in the example are there to be read rather than to be copied. The
surface normals carry intervals and a comment saying they were assumed, because
that is the common case and the file has to be able to show it. And `A2` carries a
bearing value alongside `unknown = true`, which is refused: a value and a
declaration that no uncertainty was established cannot both be asserted about the
same quantity. It is left in the example so that the refusal has something to
point at, and the message for it names the key and both fields.

The `trajectory` key is how an operator declares that two holes are from one
projectile. Nothing infers it, and that is decided elsewhere rather than here.

## Evidence

The requirement that the format carry units and uncertainties explicitly, and the
list of quantities that need one, come from `../survey/measurement-practice.md`.

The finding that a scene record compliant with the governing standard will
frequently carry no uncertainty at all comes from `../survey/standards.md`, which
reads ANSI/ASB Standard 196, 1st Ed., 2026 in full and reports no requirement to
state one.

The requirement that the file be checkable by a reader who does not have this tool
comes from `../survey/field-practice.md`, and specifically from the Minnesota
Protocol's definition of reviewability.

No performance or size measurement was made for any candidate format. The
verbosity cost above is an observation about the shape of the example, not a
measured figure.
