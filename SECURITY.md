# Security policy

## What this document covers

This is a tool that produces evidence about where a person was standing when a
shot was fired. A defect here is not only a defect in software. It can put a
region in the wrong place, or make a region smaller than the measurements
support, and a reader of the output has no way to tell that from a correct
answer. `docs/THREAT-MODEL.md` is the longer statement of who this project
expects to be attacked by; this document is the route a finding takes to get
here.

## How to report

Use GitHub's private vulnerability reporting on this repository. Open the
Security tab and choose "Report a vulnerability". The route is enabled:

The form is here, without navigating:

<https://github.com/iderex/einschlag/security/advisories/new>

```
$ gh api repos/iderex/einschlag/private-vulnerability-reporting --jq '.enabled'
true
```

Do not open a public issue for something you believe falls in the classes below.
An ordinary defect is welcome as a public issue, and if you are unsure which one
you are holding, report it privately and it will be moved.

There is no other reporting route. No security contact address is published, and
this document does not invent one. If the route above is unavailable to you,
that is a real gap and the way to say so is a public issue that describes the
obstacle rather than the finding.

Include the input file or the fragment of it that reproduces the behaviour where
you can. If the input is real case material, do not send it. Send the geometry
that reproduces the problem with the case removed, or describe the shape of the
input and the output you got. Nothing in this project is worth attaching a
person's death to an issue tracker.

## What is in scope

Anything in this repository: the library, the command line front end, the input
parser, the scene import path, the build, and the workflow files.

Two classes below are in scope specifically, and a reader who skips the rest of
this document should read those.

## Two classes of report that are treated as security reports here

**A defect that causes a reconstruction to be narrower than the input supports.**
A region that is too small excludes positions the measurements do not exclude.
Somebody standing in the excluded space is placed outside the answer by an
arithmetic error, and the output carries a level that says how confident it is
while being wrong about it. This is treated as a security report rather than as
an ordinary defect because the failure is silent and because the party it harms
is not the party running the tool. It has no visible symptom: the tool does not
crash, the artefact is well formed, and the number looks like the number a
correct run would have produced.

That includes an interval that is too tight, a region computed at a stated level
that does not hold at that level, an occlusion test that excludes an origin it
should have kept, a sampling defect that understates the spread, and a material
row applied outside the angle range it was measured over.

The mirror case, a region wider than the input supports, is an ordinary defect.
It is a real defect and it should be reported, and it is not in this class,
because a wide answer fails safe. The asymmetry is deliberate and it is the same
asymmetry the rest of the project is built on.

**An assumption applied without being recorded.** The project's condition is
that every assumption which narrowed the answer is named in the output, and
issue #45 is where that mechanism is built. A path that applies a prior, a
default, a grouping of holes, or a material row without that appearing in the
artefact defeats the condition, and it does it invisibly: the artefact reads as
though the narrowing came from the measurements. A reader who cannot see the
assumption cannot argue with it, and the people this tool is built for are
arguing against a state.

Both classes are treated this way whether or not the defect is reachable by an
attacker. An honest mistake in the geometry and a crafted input that triggers
the same path produce the same wrong region, and the person it is used against
does not care which one it was.

## What a reporter should expect

An acknowledgement that the report was received and read. No response time is
promised. This repository has one maintainer, and a policy naming a number of
days it does not have the capacity to hold would be a promise made to look
serious.

An answer saying whether the finding is accepted, and where it is accepted,
which of the classes above it falls in or that it falls in neither. Where it is
not accepted, the reason.

Where a fix lands, credit in the advisory unless you ask for none. Where the
finding turns out to be correct behaviour that is badly documented, the document
is fixed and you are told that is what happened.

There is no bounty and there is no payment.

## What happens to a released version found to carry such a defect

There are no releases yet:

```
$ gh api repos/iderex/einschlag/releases --jq 'length'
0
$ gh api repos/iderex/einschlag/tags --jq 'length'
0
```

So this section is the policy written before it is needed rather than a record
of how anything was handled. Writing it now is the point: the sentences below
are much harder to write after a specific reconstruction, with a specific person
in it, is the thing they apply to.

A released version found to carry a defect in either class above gets a security
advisory on this repository naming the affected versions, what the defect did to
the output, and which direction the error went.

**Reconstructions produced by an affected version are announced as affected.**
The advisory says so plainly, in the words that a reconstruction produced by a
listed version cannot be relied on and should be re-run. It does not say the
defect is unlikely to have mattered, and it does not leave the question to the
reader by describing only the code. An operator who ran the tool has no way of
knowing from their own output whether they hit the path, so the announcement
covers every run of the affected version rather than the runs somebody believes
were affected.

That is the hardest sentence in this document and it is written down in advance
on purpose. A reconstruction produced by an affected version may already be in a
case file, in a report, or in front of a court, and the cost of announcing it
falls on the person who used the tool in good faith. The alternative is that the
same person defends a number this project already knew was wrong.

Where a fix exists, the advisory names the version carrying it. Where none
exists yet, the advisory is published anyway with the workaround if there is one
and with the plain statement that there is none if there is not. A defect that
narrows a region is not made safer by being kept quiet until it is fixed; it is
made safer by the people relying on the output knowing they should not.

## What this policy does not do

It does not promise that anybody is monitoring for these defects. The mechanism
that would catch a narrowing defect automatically is the calibration work in
milestone 7, and the figures it produces do not exist. Until they do, a report
from a reader is the main route by which this project would learn that its
output is too narrow, which is a weak position and is stated rather than
covered.

It does not cover the correctness of measurements handed to the tool, the
security of the operator's machine, or anything else `docs/THREAT-MODEL.md`
places out of scope.
