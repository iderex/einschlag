# 0001. Decisions of record live in this directory as numbered, superseded-not-edited files

## Status

Accepted.

## Date

2026-08-07.

## The question

Every other record in milestone 2 needs somewhere to land and a shape to land in.
This one fixes both, and it lands first so that the rest have somewhere to go.

The question is not merely clerical. This project's central claim is that a
reconstruction should carry its uncertainty rather than presenting a conclusion as
a fact, and the same claim applies to the project's own reasoning. In several
years somebody will ask why the uncertainty is modelled the way it is, and the
answer will either be in the repository at the commit that depends on it, or it
will be reconstructed from memory by whoever is still available. The second is
how a project acquires beliefs whose reasons nobody can state.

There is a second thing at stake. A decision whose reasons are written down can be
shown to be wrong. A decision that exists only as a habit cannot, because there is
nothing to test. Writing the reasons down is what makes the decision falsifiable,
and that is the property this project needs most.

## The options considered

**Decisions live in issue threads on the tracker.** Zero new machinery, and the
discussion and the outcome sit together. Costs: the tracker is a separate system
from the tree, so a reader of a commit cannot see the decisions that commit was
built on without leaving the repository and reconstructing which comment was the
conclusion. Threads are edited and re-read; a decision expressed across fourteen
comments has no canonical text. If the tracker becomes unavailable, the reasons
are gone and the code remains.

**Decisions live in one long document.** One file to read, easy to search. Costs:
one file means every decision is edited in the same place, so the history of any
one decision is buried in the history of all of them, and a superseded decision is
either deleted, which destroys the record, or left in place with a note, which
makes the document unreadable. Two people writing two decisions collide on one
file every time.

**Decisions live in code comments and commit messages, where the code is.** The
reason sits next to the thing it explains. Costs: a decision that spans several
files has no home, a decision that is about not doing something has nowhere to
live at all, and a commit message is not read by anybody looking at the current
tree. The boundary decisions this project most needs recorded, about what the
model deliberately excludes, are exactly the ones with no code to attach to.

**Decisions live in numbered files in the tree, added and superseded rather than
edited.** The form generally known as architecture decision records. Costs: a
directory that grows, a numbering scheme that has to be coordinated when two
records are written at once, and the standing discipline of not editing a landed
file even when it is wrong, which is the part people find hardest and which is
the part that carries the value.

## The option taken

Numbered files in `docs/decisions/`, added and superseded rather than edited, with
the sections and rules set out in `docs/decisions/README.md` in this directory.

## The reasons

The decisive property is that the record sits at the commit. A reader checking out
any commit gets that commit's code and that commit's decisions together, and
neither can drift from the other without the drift being visible in a diff. None
of the three alternatives has this. The tracker is a different system, one long
document loses the per-decision history, and code comments have nowhere to put a
decision about an absence.

Superseding rather than editing follows from the same property. If a record could
be edited, then checking out an old commit would give the old code with the new
reasons, which is worse than having no record: it would show a decision that was
never made at the time the code was written.

The falsifier section is the reason this form was chosen over a lighter one.
Nothing else in the shape forces the writer to say what would change their mind,
and without it a record is an argument for what was already going to happen.
Requiring it also gives a cheap test for a decision that is really a preference,
since a preference cannot fill the section and the record then has to say so.

Numbered files were preferred to dated ones because the decisions in milestone 2
refer to each other by number before they are written, and a number that is
allocated in advance can be referred to in an issue. A date-based name cannot.

## What this costs

A directory that grows without bound, and the reader has no index other than the
filenames. This is accepted for now. If the count reaches the point where the
filenames stop being a usable index, that is a later decision and it supersedes
this one.

The numbering has to be coordinated where two records are written at the same
time. The milestone 2 issues name their own numbers, which handles the current
case and does not handle the general one.

The discipline of not editing a landed record has a real cost the first time a
record is found to contain a plain mistake. The record stays wrong and a new
record corrects it, which reads badly and is right.

The cost falls on a contributor rather than on a user: every decision now takes
longer to make, because it has to be written in a form that names its own cost and
its own falsifier. That is intended. A decision that is not worth the time to
write in that form was not a decision that needed recording.

## What would falsify this

Records being written and then not read at the commits that depend on them. If a
change lands whose reasoning contradicts a record in this directory and nobody
notices during review, the records are not doing the job claimed for them and a
different mechanism is needed, most likely one that a check can read rather than a
person.

Records that consistently cannot fill the falsifier section. If most records end
up saying that nothing would change the decision, then either the section is
wrong or the things being recorded are not decisions, and either way the form is
not fitting the work.

The supersession rule producing a directory nobody can navigate, where the current
position on a question requires reading four records to establish. That would be
the numbering and status scheme failing rather than the tree-based location
failing, and it would supersede this record rather than abandoning the approach.

## Evidence

None. This record rests on an argument about where a reader will look, not on a
measurement, and no number in it was measured. Recorded here as an absence rather
than left to be inferred.
