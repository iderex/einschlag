# The survey, and what it changed

The survey exists to change the plan rather than to decorate it. This file is
where that happens. It says what each survey file established, walks the plan
against it, and separates what the survey failed to find from what it found to be
absent.

It closes milestone 1.

## The survey files, and what each established

**`ellipse-accuracy.md`** established that the relation this project's uncertainty
model rests on has three published rows behind it, two of them from one research
group, and that no full text was obtained for any of them, so no material has a
measured error tied to a stated angle range.

**`measurement-practice.md`** established what an operator actually measures and
with what precision, and separated the instrument's quoted precision from the
precision of the quantity as a scene produces it. It established which inputs are
estimated rather than measured, and that the input format has to be able to say
which is which.

**`standards.md`** established that the standard governing exactly these
measurements requires an impact angle to be recorded and requires nothing about
its uncertainty, and that the ENFSI guideline does require probability as the
measure of uncertainty for evaluative reporting, so the field is not one where
nobody is required to state anything.

**`field-practice.md`** established what half the intended audience needs: a tool
that runs with no network, writes nothing outside the paths the operator chose,
names every file it writes, and produces a visual without a display. It also
established that the manual that audience works to has already told them this
class of analysis lacks an accepted process and needs a validated method behind it.

**`methods.md`** established where each direction method other than the ellipse
stops, and that the instrument uncertainty for reading a trajectory rod is around
a degree while the method uncertainties run past twenty, so the number most easily
available is the wrong one.

**`tools.md`** established what the existing tools emit. No surveyed tool was shown
to emit a probability distribution over positions, and established practice emits a
fixed five-degree cone rather than a line.

**`challenges.md`** established what gets attacked, which is the distance between
what a method supports and what was said on top of it, rather than the measurement
underneath.

## The plan walked against the survey

Confirmed means the survey supports the issue as written. Contradicted means the
plan was wrong and an issue is open to repair it. Untouched means nothing in the
survey bears on it either way, which is a real outcome and not a gap.

The issue numbers and titles below were read from the tracker on 2026-08-08 with

    gh issue list --state all --limit 100 --json number,title,state,milestone

so that this file quotes a list rather than restating one. Re-run it rather than
trusting the state described here.

### Milestone 2, the decisions of record

| Issue | Verdict | What the survey did to it |
| --- | --- | --- |
| #1, maintainer decisions | Untouched | The survey cannot answer a question reserved to the maintainer. |
| #11, language and toolchain | Confirmed | `field-practice.md` adds a fourth and fifth constraint the record now has to answer: run with no network, and name every file written. Both argue in the same direction as the record's starting recommendation rather than against it. |
| #12, model boundary | Confirmed and extended | `methods.md` ends with the list of methods to support and to defer that this record asked for, and `ellipse-accuracy.md` supplies the straight-line distance thresholds the boundary needs. |
| #13, uncertainty model | Confirmed | `measurement-practice.md` shows several inputs are estimated rather than measured, so the representation has to distinguish them. `challenges.md` supplies the sharpest reason: a probability attached to a conclusion by data that could not produce it is a documented error category, not a hypothetical. |
| #14, output representation | Confirmed | `field-practice.md` establishes what the human rights audience can use, and `tools.md` establishes what the field currently reads. |
| #15, frame, units and scene model | Confirmed and extended | `methods.md` found that the published studies do not consistently say whether an angle is measured from the surface or from the normal. That is a new reason for this record to fix a convention explicitly, and a new warning against combining figures from two studies. |
| #17, priors | Confirmed | `challenges.md` gives the failure mode this record exists against a documented shape. |
| #19, honesty rule | **Contradicted** | The properties in the issue are all about the value computed. `challenges.md` establishes that what gets attacked is the sentence, and that no property named in the plan reaches the words the tool prints. **#75** is open for it. |
| #74, the premise | **Contradicted** | The premise says established practice returns a line. `tools.md` establishes that it returns a fixed five-degree cone, and that one published program already emits an area. **#74** is the repair and is open. |

### Milestone 4, measurement and scene model

| Issue | Verdict | What the survey did to it |
| --- | --- | --- |
| #30, the hole record | Confirmed and extended | `measurement-practice.md` names each quantity that needs its own uncertainty and adds two flags the record has to carry: whether a quantity was measured or estimated, and what produced it. |
| #31, the material table | **Contradicted** | The table is sourced from a survey that produced no usable figure for any material, so as written it refuses every material. **#76** is open for it. |
| #32, the scene | Untouched | Nothing in the survey bears on the scene representation. |
| #33, the input parser | Confirmed | `standards.md` established that a file assembled from the governing standard frequently carries no uncertainty at all, and `../decisions/0007-input-format.md` already decides what happens then. |
| #34, units | Confirmed | Nothing contradicts it, and `methods.md` adds the angle-convention ambiguity as a second reason for the same discipline. |
| #35, exchange format import | Untouched | The survey establishes that operators arrive with total station and photogrammetry data. Which formats is decided in #15 and was not surveyed. |

### Milestone 5, geometry and error propagation

| Issue | Verdict | What the survey did to it |
| --- | --- | --- |
| #36, one hole to a direction | Confirmed | `ellipse-accuracy.md` establishes the arcsine sensitivity behind the near-normal degeneracy this issue is mostly about. |
| #37, sampling | Untouched | |
| #38, holes on one trajectory | Confirmed and extended | `methods.md` found no study reporting the precision of a two-point direction against the separation of the defects, so the weighting this issue needs has no measured basis and the issue should say so. It also supplies the deflection and gravity failure modes. |
| #39, back-projection | Untouched | |
| #40, combining trajectories | Untouched | |
| #41, degenerate cases | Confirmed | `ellipse-accuracy.md` and `methods.md` support the near-normal treatment already decided. |
| #42, monotonicity | Untouched | Nothing in the survey bears on it. It remains the strongest guard in the plan. |

### Milestone 6, output

| Issue | Verdict | What the survey did to it |
| --- | --- | --- |
| #43, the distribution artefact | Untouched | |
| #44, regions at stated levels | Confirmed | `challenges.md` establishes that testimony has to be limited to what the empirical evidence supports, which is what a stated level and a stated construction are for. |
| #45, assumptions named in the output | Confirmed | The strongest confirmation in the survey. It is also where the "this width is a convention" problem from `tools.md` would have to surface, and whether the naming can express that is not settled. |
| #46, the refusal to summarise | Confirmed | Abruquah is this issue's argument made by a court about a different discipline. |
| #47, the report | Untouched | It waits on entry 7 of #1, which the survey cannot answer. |
| #48, a visual without a display | Confirmed twice | `field-practice.md` gives a second, independent reason for the requirement milestone 6 already had. |

### Milestone 7, validation and calibration

| Issue | Verdict | What the survey did to it |
| --- | --- | --- |
| #49, scene generator | Untouched | |
| #50, calibration | Confirmed, and made urgent | `field-practice.md` records that the Minnesota Protocol tells this project's audience that toolmark and firearms analysis lacked a precisely defined and universally accepted process and needs a validated method behind it. `challenges.md` records PCAST asking the same question of firearms analysis and answering it against the discipline. |
| #51, reproduce the published cases | **Contradicted** | There is nothing to reproduce: the per-angle tables are exactly what was not obtained. **#76** is open for it, and it is the same root cause as #31. |
| #52, the calibration report | Untouched | |
| #53, the hardware harness | Untouched | |

### The contradictions, collected

Three, and each has an open issue.

- The premise about what established practice returns. **#74**.
- The honesty rule reaching values and not words. **#75**.
- The material table and the published cases resting on full texts nobody obtained. **#76**.

## What the survey failed to find, and what it found to be absent

These are different claims and collapsing them is how a thin reading gets read as a
strong negative result. They are kept apart here.

### Found to be absent

Something was looked for in a place that would have held it, and it was not there.

The standard that governs shooting scene documentation does not require an
uncertainty to be recorded. Two documents were read in full and neither requires it.

No open source project reconstructing a shooter position from scene measurements
exists under the names searched on GitHub. The queries and their per-query totals
are in `tools.md`.

No material in `ellipse-accuracy.md` has a measured error tied to a stated angle
range, because no full text was obtained. This is an absence in this repository's
holdings rather than an absence in the literature, and it is written that way in
both places.

The published critiques and decisions read do not include one about trajectory or
shooting-scene reconstruction testimony specifically. Every entry in
`challenges.md` is a neighbouring discipline or a review outside a courtroom.

### Failed to find

The search itself was too narrow, too short, or blocked, so nothing follows about
whether the thing exists.

Full text for every study in `ellipse-accuracy.md` and `methods.md`. Publisher
pages refuse the route used, and one apparent success turned out to be a redirect
stub carrying no article text.

The older literature behind the trigonometric relation itself. Secondary
descriptions were seen; no primary record with a resolvable identifier was
obtained.

Any error figure for a two-point direction as a function of the separation of the
defects, and any study reporting how often a mark is misidentified as a ricochet.

The angle convention, from the surface or from the normal, for each study whose
range this project might use.

Case law and critique outside the United States. The searches were in English and
the sources that answered were American. Much of the intended audience is not.

The 2009 National Research Council report, read only through others quoting it.

ISO 21043-3:2025 clause on measurement uncertainty, whose text was not obtained, so
the question is open for that standard rather than answered negatively.

Instrument figures for a forensic inclinometer, a zero-edge protractor, and a
trajectory rod as an instrument, and any figure for operator error on top of any
instrument figure.

## What this file is not

It is not a summary that replaces the survey files. Every figure and every source
lives in the file that read it, and this one points rather than restates, so that a
number cannot drift here against the place it came from.

It is also not a verdict on whether the survey was good enough. It was thin in the
one place the project can least afford, which is the measured error of the relation
everything rests on, and #76 is where that is either repaired or declared
unrepairable.
