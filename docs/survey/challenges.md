# How a reconstruction is attacked, and what the record says went wrong

This tool's output is meant to survive an adversarial reading. The cheapest way to
learn what will be attacked is to read what has already been attacked.

One entry per case or critique. Each carries the source, the year, the jurisdiction
where one applies, what was claimed, and the ground on which it was challenged or
found wanting. The file ends with the failure modes those entries produce, phrased
as things this tool must not do, and says for each whether the current plan already
prevents it.

## How these entries were obtained, and what this reading does not cover

Two court and government documents were downloaded and read directly. They were
converted with `pdftotext -layout` and searched, so every quotation below can be
reproduced:

    curl -sL -o abruquah.pdf "https://www.mdcourts.gov/data/opinions/coa/2023/10a22.pdf"
    curl -sL -o pcast.pdf "https://obamawhitehouse.archives.gov/sites/default/files/microsites/ostp/PCAST/pcast_forensic_science_report_final.pdf"
    curl -sL -o doj-ferguson.pdf "https://www.justice.gov/sites/default/files/opa/press-releases/attachments/2015/03/04/doj_report_on_shooting_of_michael_brown_1.pdf"
    pdftotext -layout pcast.pdf pcast.txt

Two limits on this reading, and both are large.

**No appellate decision specifically about bullet trajectory or shooting-scene
reconstruction testimony was found.** Searches for one returned firearms
identification cases instead, which is a different discipline attacked in a similar
shape. Every entry below is therefore either about a neighbouring discipline or
about a reconstruction reviewed outside a courtroom. Whether such a decision exists
and was missed is not established; this is the absence of a result from these
searches, not a demonstration that none exists.

**This reading is United States material almost throughout.** The searches were in
English and the sources that answered are American. Nothing from German, Dutch,
other European or international jurisdictions was searched on this route, which
matters because much of the intended audience sits in exactly those places, and
because the ENFSI guideline already read in `standards.md` is the European document
this file has no case law to sit beside.

## Abruquah v. State

**Source.** Kobina Ebo Abruquah v. State of Maryland, No. 10, September Term 2022,
Supreme Court of Maryland, opinion by Fader, C.J., filed 2023-06-20. Read from the
court's own published opinion.

**Jurisdiction.** Maryland.

**What was claimed.** A firearms examiner testified without qualification that
bullets left at a murder scene were fired from a gun the defendant acknowledged was
his.

**The ground of the challenge, and what the court held.** From the reported holding:

> Firearms identification examiner testifying as an expert witness should not have
> been permitted to offer an unqualified opinion that crime scene bullets and a
> bullet fragment were fired from the petitioner's gun. The reports, studies, and
> testimony presented to the circuit court demonstrate that the firearms
> identification methodology employed by the examiner in this case can support
> reliable conclusions that patterns and markings on bullets are consistent or
> inconsistent with those on bullets fired from a particular known firearm. Those
> reports, studies, and testimony do not, however, demonstrate that the methodology
> used can reliably support an unqualified conclusion that such bullets were fired
> from a particular firearm.

**Why it is in this file.** The attack was not on the measurement. It was on the
distance between what the method can support and what the witness said. The method
survived; the sentence did not. Three of the seven justices dissented, so this was
not a unanimous court, and the holding is Maryland's rather than a general rule.

## PCAST, Forensic Science in Criminal Courts

**Source.** President's Council of Advisors on Science and Technology, "Forensic
Science in Criminal Courts: Ensuring Scientific Validity of Feature-Comparison
Methods", 2016. Read from the report PDF.

**Jurisdiction.** None. It is advice to the President and to the courts, not law,
and it was publicly disputed by the Department of Justice at the time.

**What was found wanting.** Two things, and they are separate.

On the discipline nearest this project:

> Foundational validity. PCAST finds that firearms analysis currently falls short of
> the criteria for foundational validity, because there is only a single
> appropriately designed study to measure validity and estimate reliability. The
> scientific criteria for foundational validity require more than one such study, to
> demonstrate reproducibility.

On what witnesses say:

> Reviews of trial transcripts have found that expert witnesses have often overstated
> the probative value of their evidence, going far beyond what the relevant science
> can justify.

and, as advice to the courts:

> courts should never permit scientifically indefensible claims such as: "zero,"
> "vanishingly small," "essentially zero," "negligible," "minimal," or "microscopic"
> error rates; "100 percent certainty" or proof "to a reasonable degree of scientific
> certainty;" identification "to the exclusion of all other sources;" or a chance of
> error so remote as to be a "practical impossibility."

**Why it is in this file.** The first passage is the question this project will be
asked and cannot yet answer: how many independent studies measure the error of the
method it implements. `ellipse-accuracy.md` records three rows, two of them from one
research group, and no full text obtained for any of them. The second and third
passages are a list of sentences, and a tool that prints one of them has handed the
cross-examination its opening.

## The National Commission on Forensic Science on "reasonable scientific certainty"

**Source.** National Commission on Forensic Science, "Recommendations to the Attorney
General Regarding Use of the Term `Reasonable Scientific Certainty'", approved
2016-03-22. **Read only as quoted inside the PCAST report**, not from the original
document.

**Jurisdiction.** A recommendation to the United States Attorney General.

**What was found wanting.** That conclusions are testified to as held "to a
reasonable degree of scientific certainty" or "to a reasonable degree of [discipline]
certainty", and that, quoting the Commission through PCAST, "These terms have no
scientific meaning and may mislead factfinders about the level of objectivity
involved in the analysis, its scientific reliability and limitations, and the ability
of the analysis to reach a conclusion."

## The FBI microscopic hair comparison review

**Source.** FBI press release, "FBI Testimony on Microscopic Hair Analysis Contained
Errors in at Least 90 Percent of Cases in Ongoing Review", 2015-04-20. **Read only
through PCAST's citation of it and PCAST's summary of the error categories**, not
from the press release itself. The title carries the headline figure and is quoted
here as a title rather than as a figure this project verified.

**Jurisdiction.** United States federal, and it reached cases in many states.

**What was claimed, and what was wrong with it.** PCAST records three categories of
erroneous statement, in which the examiner:

> (1) stated or implied that evidentiary hair could be associated with a specific
> individual to the exclusion of all others; (2) assigned to the positive association
> a statistical weight or a probability that the evidentiary hair originated from a
> particular source; or (3) cited the number of cases worked in the lab and the
> number of successful matches to support a conclusion that an evidentiary hair
> belonged to a specific individual.

**Why it is in this file.** Category 2 is the one that should frighten this project
most. It is not a false conclusion, it is a *number* attached to a conclusion, where
the number did not come from data that could produce it. A tool whose entire output
is numbers attached to positions is one careless denominator away from that category,
and it will be more persuasive than a sentence because it came out of a computer.

## The Department of Justice review of the shooting of Michael Brown

**Source.** Department of Justice Report Regarding the Criminal Investigation into the
Shooting Death of Michael Brown by Ferguson, Missouri Police Officer Darren Wilson,
2015-03-04. Read from the report PDF.

**Jurisdiction.** United States federal. It is a declination memorandum rather than a
judgment, so nothing in it was tested by cross-examination.

**What the reconstruction claimed.** A shooting incident reconstruction of the
interior driver's door panel established the trajectory of a recovered bullet as
"at a downward angle from left to right". Trajectory findings from the autopsies are
used throughout to corroborate or contradict witness accounts.

**Why it is in this file, and it is not because it was attacked.** It is here because
of what it wrote down as *not* established, in a document whose conclusion was that no
charges would be brought. Four examples, quoted:

> shell casings provide limited evidentiary value relative to the precise location of
> the shooter and bullet trajectory because they tend to bounce and roll unpredictably
> after being ejected from the firearm and before coming to rest.

> With the exception of the two wounds to Brown's right arm, which indicate neither
> bullet trajectory nor the direction in which Brown was moving when he was struck,
> the medical examiners' reports are in agreement [...]

> The order of the remaining shots cannot be determined [...]

> [Crime scene detectives] removed a small section of siding, but were unable to
> determine whether there was a projectile within the wall without causing significant
> structural damage to the building. There is no evidence to indicate what caused the
> hole or when it was made.

Those four sentences are the voice this project's output is supposed to have, written
by an investigative body under maximum public pressure to say something definite. They
also record something duller and more useful: an operator's constraints are partial,
some of the evidence is unrecoverable without destroying the scene, and a hole in a
wall may have nothing to do with the shooting.

## The 2009 National Research Council report

**Not read.** PCAST describes it as the most comprehensive review to date of the
forensic sciences and relies on it for the finding about overstated testimony. It is
recorded here as an entry that is owed rather than left out silently, because a survey
of critiques that omits the most cited one is not thin, it is wrong. What it says
about shooting reconstruction specifically was not established.

## The failure modes, phrased as things this tool must not do

Drawn from the entries above. Each says whether the current plan already prevents it.

**It must not emit a conclusion the method cannot support.** From Abruquah. Prevented:
`../decisions/0011-degenerate-cases.md` fixes that an unconstrained scene returns a
region and a contradictory one returns a contradiction, and issue #19 carries the
property that no call returns a single position or a single line.

**It must not report a region as though its width were derived from the measurement
when it is a convention.** From Abruquah by analogy and from the five-degree cone in
`tools.md`. Prevented in principle by the requirement in #19 that every region carries
the level it was constructed at, and by #45, which asks that every assumption which
narrowed the answer is named. Whether "this width is a convention" is expressible in
that naming is not settled, and #74 is where the cone is being argued.

**It must not attach a probability to a position unless that probability came from
data that can produce it.** From the FBI hair review, category 2. Not prevented by
anything named as a mechanism today. It is the substance of the uncertainty model
decision in #13 rather than a separate property, and it is recorded here so that the
record answers it rather than assuming it.

**It must not cite its own operational history as support for a conclusion.** From the
FBI hair review, category 3. Not applicable: the tool has no case history to cite. It
is listed so that a later feature reporting how many scenes the tool has processed is
recognised as this failure mode arriving.

**It must not state or imply a certainty it cannot support, in words.** From PCAST and
from the National Commission. **Not prevented.** Every property in #19 is about the
value computed, and none is about the text printed. Issue #75 holds this and names what
a mechanism would have to cover.

**It must not present as determined something the inputs leave undetermined.** From the
Department of Justice report, on the order of the shots and on the two arm wounds that
indicate neither trajectory nor direction. Prevented for the whole-scene case by
`../decisions/0011-degenerate-cases.md`, which makes an unconstrained result explicit
rather than empty or default. Not prevented per input: an individual hole that
constrains nothing is a case the plan has decided for the near-normal degeneracy and
for nothing else, and #45 is the nearest issue.

**It must not be unable to say what its own error rate is.** From PCAST's finding on
foundational validity. Not prevented, and not preventable by a property: it is the
whole of milestone 7. It is listed because the first question this tool will be asked
in a hearing is the one PCAST asks of firearms analysis, and because the answer today
is that the underlying relation rests on three abstracts, two of them from one research
group.

**It must not let a validation status go unstated where the output is read.** From the
same finding. This is not opened as an issue: it sits inside the maintainer decision on
what the project says about use as expert evidence, entry 6 of #1, and duplicating it
on the tracker would split one question across two places.
