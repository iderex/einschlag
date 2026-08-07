# What human rights documentation practice needs from a reconstruction

Half the stated audience for this tool does not work in a laboratory. They work
at a scene that was not preserved, sometimes weeks after the event, often with a
phone camera, and their output has to survive a state denying it. That is a
different set of constraints from a forensic laboratory, and it is read here
rather than imagined.

## Sources, and what was read of each

### The Minnesota Protocol on the Investigation of Potentially Unlawful Death (2016)

Published by OHCHR, New York and Geneva, 2017. UN symbol HR/PUB/17/4, ISBN
978-92-1-154220-2. Second edition of the 1991 UN Manual on the Effective
Prevention and Investigation of Extra-legal, Arbitrary and Summary Executions.
The copy read is the advance electronic version of the second edition at
<https://www.law.berkeley.edu/wp-content/uploads/2015/04/The-Minnesota-Protocol-on-the-Investigation-of-Potentially-Unlawful-Death-2016-ADVANCE-WEB-VERSION.pdf>,
99 pages.

Read in part, and the parts are named where they are quoted. Sections on
crime-scene investigation, firearms evidence, disinterment, the written report,
and the glossary were read. The autopsy and skeletal sections were not, beyond
what search located.

### Berkeley Protocol on Digital Open Source Investigations

Published by OHCHR and the Human Rights Center at UC Berkeley School of Law, New
York and Geneva, 2022. UN symbol HR/PUB/20/2, ISBN 978-92-1-154233-2. Copy read
at
<https://www.ohchr.org/sites/default/files/documents/publications/OHCHR_BerkeleyProtocol.pdf>.

Read in part. The principles chapter and the sections on infrastructure, devices
and protection measures were read. The chapters on preservation, verification and
investigative analysis were located but not read in full.

### Amnesty International's Evidence Lab and Digital Verification Corps

Read as published web material rather than as a manual, because no manual is
published. The Evidence Lab's own description at
<https://citizenevidence.org/about-us/> and its published account of how it
verifies attacks in Ukraine, dated 18 March 2022, at
<https://www.amnesty.org/en/latest/news/2022/03/a-guide-to-how-amnesty-verifies-military-attacks-in-ukraine/>.

### What was sought and not obtained

The internal methodology of Forensic Architecture. Its own pages did not render
on this route, and secondary descriptions were not treated as a source for what
it does.

Bellingcat's guides on measurement from imagery or on shot and trajectory
analysis. Its how-to index was fetched and carried no guide on 3D reconstruction,
measuring from imagery, or trajectory analysis at the time it was read.

The Istanbul Protocol, the UN guidance on less-lethal weapons, and the
methodology statements of Human Rights Watch, Physicians for Human Rights and
SITU Research. None of these was read, and the answers below are correspondingly
narrower than the field they describe.

## What measurements are realistically available at an unpreserved scene

The Minnesota Protocol is explicit that capability varies and that the floor is
notes, sketches and photographs. Paragraph 62 states that investigations vary in
their capability to examine material scientifically, "but effectively recording
the crime scene using notes, sketch plans and photographs will be necessary".

Paragraph 92, on photographing remains, states that all photographs should
include a reference number, a scale and a direction indicator, that position
should be recorded through notations and measurements in the scene sketch, and
that sketches and diagrams "could be supplemented by details from a GPS and/or a
compass, a baseline, or any photogrammetric programme". It adds that where
available, measurements can be made electronically with a total station
theodolite for later integration into a digital mapping system. The order of that
sentence is the finding: the scale, the compass and the baseline are the
expectation, and the total station is the conditional.

Paragraph 237 says the same thing for a burial: if recording equipment such as a
total station is not available, establish a datum point and map the site with a
grid and standard archaeological techniques.

So the realistic input set is a photograph with a scale in it, a sketch with tape
measurements from a datum, a compass bearing, and a GPS position. Sometimes a
photogrammetric model built from photographs. Rarely a total station or a laser
scan.

Two consequences follow for this project and both are load-bearing.

First, the input format cannot assume survey-grade coordinates. It has to accept
positions with uncertainties of centimetres or worse, and surface orientations
that were assumed rather than measured, because that is what a scene sketch
produces. The precision figures for these routes are in `measurement-practice.md`.

Second, the perforation dimensions this method depends on may have been read off
a photograph rather than with a caliper. The two photogrammetry figures found are
in `measurement-practice.md` and both measure camera position rather than hole
geometry, so the precision of an axis length read off a model is not a number
this project has. It is the largest open measurement question for this audience.

## What the practice says about the status of firearms reconstruction

This is not in the issue's three questions and it belongs here anyway, because it
is the single most relevant sentence found in either protocol.

Minnesota Protocol paragraph 138, on firearms evidence, describes ballistic
information as including the pattern and movement of projectiles from a firearm
after discharge, and then states: "At the time of drafting of this Protocol,
however, toolmark and firearms analysis lacked a precisely defined and universally
accepted process." The footnote cites the 2009 US National Research Council report
Strengthening Forensic Science in the United States: A Path Forward.

Paragraph 142 states that care needs to be taken to ensure that the analysis of
evidence of this kind "is underpinned by a validated scientific method".

The audience this project is built for has been told, in the manual it works to,
that this class of analysis lacks an accepted process and needs a validated
method behind it. That is a stronger argument for milestone 7 than anything this
project has said about itself so far.

## What form of output is usable in the reports these organisations publish

The Minnesota Protocol's report requirements are about the investigation, not
about a computed artefact. Paragraph 51 lists what a written report should carry:
the identity and official status of the person making the initial report, the
circumstances under which it was made, the identity of the victims where known,
dates, times and locations, the method of causing death, those believed
responsible, and the underlying reasons.

Two things in the protocol bear directly on the shape of a reconstruction
artefact.

The glossary defines reliability as "the stability of a result when the test is
undertaken by different observers in different places at different times". A
reconstruction that gives a different answer on a different machine fails that
definition on its face, which is a requirement on determinism rather than a
preference.

The glossary defines reviewability as the aim that "another forensic doctor or
pathologist at another time can independently come to his or her own conclusions",
and states that this is what enables conclusions to be drawn about the reliability
of the work. Applied to a reconstruction, reviewability means the other side must
be able to re-run it from the same input and get the same output, and must be able
to see what the assumptions were. That is an argument for shipping the input file
and the run manifest as part of the output, not for shipping a picture.

The Berkeley Protocol is more direct, because it governs work that is already
computational. Paragraph 25 requires that the steps of an investigation, from
identification of material through collection, analysis and reporting, be
consistently and clearly documented, and states that "automated processes and
software must be understood by users and be explainable in court either by users
or developers", and that investigators "should record any tools or software used
in the course of their work". Paragraph 26 requires accurately reporting data
"including acknowledging any gaps". Paragraph 27 requires objectivity through
"developing and deploying multiple working hypotheses and not favouring any
particular theory".

Read against this project, those three are requirements on the artefact rather
than on the operator. A tool whose output cannot be explained in court by the
person using it is a liability to them. A tool that reports a region without the
assumptions that narrowed it fails the requirement to acknowledge gaps. A tool
that returns one position rather than the set of positions consistent with the
data has favoured a theory on the operator's behalf.

Amnesty's published verification method is a corroboration workflow rather than a
measurement workflow: geolocation and chronolocation against satellite imagery
and ground-level photography, remote sensing, weapons identification by arms
experts from photographs and remnants, corroboration against eyewitness
testimony, and preservation of the material for accountability mechanisms. Its
own account of that method does not state confidence levels and does not describe
what it does when verification is not achieved.

That last observation is a reading of one published page and not a
characterisation of the organisation's internal practice, which was not obtained.
It is recorded because it points at the same gap as `standards.md`: the published
statements of method in this field describe how a finding is reached and are
mostly silent on how firmly it is held.

The output shapes that follow, as a claim this project is making rather than a
finding read off a source: a machine-readable artefact carrying the distribution
and the assumptions, an input file that travels with it, and a picture that shows
the extent of the described space rather than being cropped to its own data. The
first two are what reviewability needs. The third is what a report can print.

## Constraints from working on a laptop with no network, on hardware that may be searched

The Berkeley Protocol paragraph 97 requires that hardware and equipment be
password protected, have full-disk encryption enabled, and ideally use multifactor
authentication; that all equipment be regularly backed up; that hardware be stored
securely with access restricted when not in use; and that personal equipment not
be used for work-related activities and investigation equipment not be used for
personal ones. Paragraph 80 lists protection measures including physical locks,
passwords, encryption and access controls, and policy measures such as rules
against sending internal work product to a personal email account.

The methodological principles the protocol names include accuracy, data
minimization, data preservation and security by design.

Minnesota Protocol paragraph 54 requires an information management system that is
comprehensive, consistent and secure, taking account of security concerns, and
states that such a system "does not need to be complex or technologically
advanced".

What this project has to do about that:

Run with no network at all, and be able to demonstrate it rather than assert it.
Data minimization is a principle these operators are already working to, and a
tool that phones home breaks their compliance and not only their preference.

Write nothing outside the paths the operator chose. On a machine that may be
searched, a cache, a temporary file or a log in a home directory is case material
in a place the operator did not know about and will not think to wipe.

Name every file the tool writes, in the documentation, so that an operator can
account for what is on the disk. This is a stronger requirement than "no
telemetry" and it is the one that follows from hardware being searched rather
than from hardware being online.

Work under full-disk encryption without needing anything of its own, and not
attempt any encryption of its own. A tool that invents its own protection on top
of the platform's is a tool whose protection nobody has audited.

Be runnable on a laptop rather than needing a server or a graphics card, and
produce its visual output without a display server, which is already what
milestone 6 asks for and is here reinforced by a second reason.

## What this feeds into milestone 9, and what it does not

The reading is stronger than a plain no-telemetry position in one respect: the
requirement that follows from paragraph 97 and from a machine that may be searched
is not only that nothing leaves, but that everything written to disk is named and
accounted for. That belongs in the privacy document as a stated commitment, and it
is the kind of commitment a test can hold.

It is not stronger in the other direction. Nothing read here asks this project to
encrypt anything, to authenticate anyone, or to manage evidence custody. The
protocols place those on the operator and their organisation, and a tool that took
them on would be claiming a control it cannot deliver.

No issue is opened from this file, because the requirement above is inside what
milestone 9 already covers rather than beyond it. If reading the sources that were
not obtained changes that, the issue is opened then.
