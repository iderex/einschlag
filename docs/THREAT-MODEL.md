# Threat model

This tool produces evidence in disputes where one side may be a state. That is a
threat model worth writing down rather than assuming.

Nothing in this document is a control that exists today. There is no code in this
repository yet, so every mitigation below is either a pointer to an issue that
owes it or a plan with no pointer, and each is labelled. A mitigation with a
pointer is a thing somebody has committed to build. A mitigation labelled a plan
is a thing nobody has yet.

## Adversary 1: supplies a crafted input file to make the tool produce a favourable region

The input file is written by whoever is doing the reconstruction, and in an
evidential dispute that is not always a neutral party. An adversary who can choose
the input can try to make the tool produce a region that suits them: one that
excludes a position, one that is narrow enough to look authoritative, or one that
is so wide the reconstruction is useless.

They can also attack the parser itself, with a file designed to crash it, hang it,
or make it read a scene the operator did not write. The last of those is the
interesting one. A crash is visible. A file that parses into a different scene
than it appears to describe on paper is a wrong reconstruction that looks right.

What addresses it:

- The format refuses a bare number where a measured value is expected, refuses a
  hole with no uncertainty declaration, and refuses an unknown format version
  outright rather than reading it optimistically. Decision record
  `decisions/0007-input-format.md`, and the parser that implements it is issue #33.
- Fuzzing the parser, and the scene import path, with the stated aim of finding an
  input that parses into a scene the operator did not write rather than only one
  that crashes. Issue #58.
- Every assumption that narrowed the answer is named in the output, so a region
  narrowed by a choice in the input carries that choice with it. Issue #45.
- Nothing infers which holes belong to one projectile or which shots came from one
  position; those are declared by the operator and are visible in the file. Issue
  #38 asserts that no code path groups holes on its own.

What does not address it. Nothing here can tell a crafted measurement from a
mistaken one, or from a correct one. An adversary who writes plausible numbers
gets a plausible answer, and no property of this tool changes that. What the tool
can do is make the input auditable by a reader who did not write it, which is why
the format was chosen for line-by-line checkability rather than for compactness.
That shifts the defence to disclosure and to the other side reading the file, and
this document says so rather than claiming a control.

## Adversary 2: modifies a released artefact so that it reports a narrower answer

A modified build is the highest-value attack on this tool, because a narrower
region is exactly what one side of a dispute wants and because nothing in the
output of a modified build looks wrong. It reports a region, with a level, with a
manifest, and the region is a lie.

What addresses it:

- The run manifest records the tool version and the commit it was built from, and
  travels inside the artefact. Decision record `decisions/0009-determinism.md`.
  This detects a build that honestly reports being a different build. It does not
  detect one that lies about it.
- Determinism means that a second party with the same input, the same seed and a
  build they compiled themselves gets byte-identical output, and a discrepancy is
  detectable. Same record. This is the strongest defence available and it depends
  entirely on the other side actually re-running it.
- Supply chain hygiene on the build, and code scanning over it. Issues #59 and
  #56, with the target written in `QUALITY-PARITY.md`.
- The dependency lock is committed and a build that would need to change it fails.
  Issue #26.

What does not address it. There is no signing of release artefacts today, and
whether releases are signed and by whom is a maintainer decision that is open.
Until it is answered, a released artefact cannot be told from a modified one, and
that is the plain statement of the position rather than a risk this document has
accepted on anyone's behalf. Reproducibility of the build itself, as distinct from
determinism of the run, is a plan with no pointer.

## Adversary 3: has access to the operator's machine

This adversary covers a seized laptop, a border search, a compromised host, and a
colleague who should not be reading the case. They are grouped because what this
tool can do about them is the same.

What addresses it:

- The tool sends nothing, so there is no channel it opens on the operator's
  behalf. `PRIVACY.md`, with the test owed by issue #63.
- The tool writes nothing outside the output the operator named: no cache, no
  temporary file elsewhere, no log in a home directory, no state directory.
  `PRIVACY.md`.
- Every identifying thing the tool does write to disk is named in `PRIVACY.md`, so
  an operator can account for what is on the machine and remove it.

What does not address it. Everything else. The tool does not encrypt anything and
will not: the platform's full-disk encryption is what the published practice this
audience works to already requires, and a tool that invented its own protection on
top of it would be adding an unaudited layer. The tool does not authenticate a
user, does not manage evidence custody, and cannot protect the input file the
operator wrote. An adversary with the machine unlocked has the case.

The one thing this project can add that is not on the list above is restraint, and
it is worth naming as a mitigation rather than as an absence: every feature that
would write more to disk, remember more between runs, or make the tool more
convenient by caching is a feature that arms this adversary.

## Adversary 4: does not attack the software at all, and attacks the reconstruction in argument

This is the adversary this tool will actually meet. Counsel for a state, in court,
arguing that the reconstruction is unreliable, that the method has no accepted
error rate, that the operator is not qualified, and that the output is a picture
produced by a program nobody has validated.

What addresses it:

- Calibration figures: does a region stated at a given level contain the truth at
  about that rate, broken down rather than aggregated. Issues #50 and #52. This is
  the answer to the question about error rate, and today the answer does not
  exist.
- The refusal to summarise, enforced rather than promised, so that the tool cannot
  be quoted as having named a single position. Issue #46.
- Every assumption that narrowed the answer named in the output. Issue #45.
- Determinism and the run manifest, so the other side can re-run the case and get
  the same answer. `decisions/0009-determinism.md`.
- The decision records, which state what was decided, why, what it costs and what
  would falsify it, at the commit the build came from. `decisions/README.md`.
- The surveys, which record what the published error figures for the underlying
  method are and, more importantly, which materials have none.
  `survey/ellipse-accuracy.md`.

What does not address it, and this is the honest part. Most of the above is
documentation and a rule about output shape rather than code that refuses
anything. A tool cannot make an argument in court. What it can do is arrive with
its assumptions written down, its numbers attached to the commands that produced
them, and its width intact, so that the person defending it is defending something
that was built to be attacked.

One reading in the surveys bears directly on this adversary and is recorded here
because it cuts both ways. `survey/field-practice.md` quotes the Minnesota
Protocol stating that at the time of its drafting, toolmark and firearms analysis
lacked a precisely defined and universally accepted process. An adversary will
quote that sentence at any reconstruction, including one produced by this tool. It
is also the strongest available argument for why milestone 7 exists.

## What is out of scope

The correctness of the operator's measurements. This tool takes the numbers it is
given. It cannot check that a wall was measured where the operator says it was,
that the axis lengths came off the hole they name, or that the holes are from the
incident under investigation. It does not try, and an output is only as true as
its input.

The identification of a firearm, a calibre or an ammunition type. Nothing here
does that.

Anything about who fired. A region of positions is not a person, and the tool
neither knows nor infers who was standing in one.

The chain of custody of any evidence, including the input file. That is the
operator's system and this tool is not part of it.

The operator's network, host security, disk encryption and backups.

Whether the reconstruction should have been done at all, or whether the result
should be disclosed. Those are decisions for the people doing the work.

## What this document is not

It is not a security assessment, because there is nothing to assess yet. It is the
list of adversaries this project intends to be judged against, written before the
code so that the code can be built for them, and it will be wrong in places that
only become visible once something exists to attack. When that happens, this
document is corrected in place rather than defended, and what was wrong is said
plainly in the commit that corrects it.
