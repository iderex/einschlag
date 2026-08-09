# 0015. A key the build does not know is refused, and the file carries every key the scene model needs

## Status

Accepted.

## Date

2026-08-09.

## The question

`0007-input-format.md` says the parser does not accept unknown fields silently,
that an unknown field is either refused or reported, and that the decision record
says which. It does not say which. This is that record.

There is a second question underneath it, and the two cannot be separated. What
counts as a known key depends on what the file is required to carry, and the
worked example in 0007 does not carry everything the scene needs.
`0006-frame-and-units.md`, written the day after, requires a surface to have a
material, a declared contour and a bounded outline, and requires the origin of
the frame to be described in the file well enough that a person at the scene
could find it again. `#32` then built a scene that also carries a ground.
Deciding what an unknown key is means deciding, in the same breath, what the
known ones are.

What is at stake in the first question is the failure this whole boundary exists
against. An operator types `defromation = "moderate"`. If the key is reported and
the file is read anyway, the hole is graded by whatever the absent key falls back
to, the run produces a region, and nothing in the output is wrong-looking. If the
key is refused, the operator fixes a spelling.

## The options considered

**Ignore an unknown key.** Costs nothing to implement and is what most
configuration readers do. Cost: a mistyped key is a value the operator stated and
the tool did not read, and the tool cannot tell that case from a key belonging to
a newer version of the format. It is the silent-misread failure in its purest
form.

**Report an unknown key and carry on.** The operator is told, and the run
proceeds. Cost: a warning on a run that produced an answer is a warning nobody
reads, and this project's whole argument is that a narrow answer produced under
an unnoticed assumption is worse than no answer. It also needs somewhere for a
warning to go, and there is no output artefact yet: `#43` is where one is
decided.

**Refuse an unknown key.** Cost: a file written for a newer version of the format
is refused by an older build even where the added key is one the older build
could have ignored safely. That case is already covered, because the version is
required and a version this build does not read is refused before any key is
looked at. The residual cost is an operator who adds a comment as a key rather
than as a comment, and meets a refusal for it.

**On the second question: keep the format to what 0007's example shows, and
default the rest.** Cost: the defaults would be a contour, an outline and a
ground the tool invented, and a scene assembled out of them would carry
assumptions nobody stated into a region that looks measured.
`0007-input-format.md` argues that at length for the uncertainty and the
argument does not change for the shape of a wall.

**Require them, and let the worked example in 0007 be a file that is refused.**
Cost: the record's example stops being a file somebody can copy, and a reader
who copies it meets several refusals at once.

## The option taken

**A key this build does not know is refused**, naming the key, the table it was
written in and the line.

**The file carries every key the scene model requires**, which is more than the
worked example in 0007 shows:

- `[scene]` carries `origin` and `reference_direction`, both text, both required.
  0006 requires the origin to be described and requires +Y to be a direction the
  operator names.
- `[scene]` carries `ground`, a measured length.
- Each `[[scene.surface]]` carries `material`, `contour` and `outline` beside the
  identifier and the two normals the example shows. `contour` is one of `flat`,
  `convex` and `concave`; `outline` is a unit and at least three vertices.
- `[[scene.obstacle]]` exists, carrying an identifier and a set of faces, and a
  scene may have none.
- `trajectory` on a hole stays optional. A hole that names none is grouped with
  nothing, and the refusal above is what closes the route where a mistyped
  `trajectroy` silently ungroups two holes.

**The worked example in `0007-input-format.md` is a file that is refused.** It is
tracked as a fixture, byte for byte, and a test asserts both that it still
matches the record and that it is refused for the reason the record itself names:
a value stated beside `unknown = true`.

## The reasons

Refusing rather than reporting, because the two behaviours are not symmetrical in
their failure. A refusal costs an operator a minute and points at the line. A
report costs nothing until the day somebody quotes a region that was narrowed by
a key the tool did not read, and then it costs the thing this project is for.
That is the same asymmetry `0007-input-format.md` used to refuse a value stated
without its uncertainty, arriving through a different door.

Refusing rather than ignoring needs no separate argument: ignoring is reporting
with the report removed.

The version check is what makes the refusal affordable. The case an ignoring
reader is usually built for is a file from a newer version carrying a key this
build could skip. Here that file is refused at its version, before any key is
read, with a message saying which version was found and which this build reads.
So the forward-compatibility argument for ignoring a key has nothing left to
protect.

Requiring the scene keys rather than defaulting them, because every default here
is a physical claim: that a wall is flat, that the ground is at zero, that the
frame's origin is somewhere. A region narrowed by a claim the tool made is the
failure `docs/decisions/0010-honesty-rule.md` is written against.

Letting 0007's example be refused rather than amending it, because a record is
superseded and not edited, and because the example is already deliberately a
refused file: it carries a value beside `unknown = true` and says so. A record
whose example is a refused file is doing something useful as long as a reader is
told, and `fixtures/scene/README.md` and this record are where they are told.

## What this costs

**The cost falls on the operator first.** A file written from 0007's example
alone is refused, several times over, and the messages are the only thing telling
them what else is needed. There is no document walking an operator through
writing a file: `docs/IMPORT.md` belongs to `#35` and the worked example an
operator can run belongs to `#68`. Until one of those lands, the fixture at
`fixtures/scene/two-holes-in-one-wall.toml` is the nearest thing to a template
and it is not one.

**It costs the record's example its copy-and-paste value.** Somebody reading
0007 and typing what it shows will meet refusals. That is stated here and in
`fixtures/scene/README.md`, and it is not softened by having been decided
deliberately.

**A comment written as a key is refused.** TOML has comments and the format uses
them; an operator who writes `note = "assumed from the building"` on a hole meets
a refusal for a line they meant as a note. No key in this format takes free text
about a hole, and adding one is a change to the format rather than a thing the
reader should permit quietly.

## What would falsify this

An operator population that meets the unknown-key refusal mostly on keys that
were harmless, rather than on spellings. That would say the closed set is too
narrow rather than that refusing is wrong, and it is measurable the first time
anybody other than this project writes a file.

A version of the format that has to add a key an older build could safely ignore,
in a way the version number cannot express. `0007-input-format.md` says adding an
optional key the tool ignores when absent does not increment the version, which
is exactly the case this record refuses. If that case turns out to be common, the
two records disagree and this one is the newer.

Evidence that a default for one of the required scene keys is not a physical
claim after all. The ground is the candidate: a scene described entirely above a
single floor may have nothing to say about where that floor is, and a required
key then buys nothing over `unknown = true`, which the format already allows and
which `#32` refuses on the ground alone.

## Evidence

The requirement that a surface carry a material, a contour and an outline, and
that the origin be described in the file, is `0006-frame-and-units.md` under
"The scene" and "The frame".

That the contour is a value the operator already holds comes from
`../survey/standards.md`, which reads ANSI/ASB Standard 196 in full and reports
clause 4.3.4 requiring the contour of the target surface to be documented.

That the worked example in `0007-input-format.md` is a refused file is that
record's own sentence, under "A worked example": the bearing of `A2` carries a
value beside `unknown = true`, "which is refused", and it is left in the example
"so that the refusal has something to point at".

No count of how many keys an operator mistypes in practice was measured, and none
is quoted. The failure the refusal is aimed at is argued from the shape of the
format rather than from a frequency anybody has established.
