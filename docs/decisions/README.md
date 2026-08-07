# Decision records

A decision that lives in an issue thread is a decision whose reasons are read
once and then paraphrased from memory. A decision in the tree is read at the
commit that depends on it. This project will be asked, years after the fact, why
the uncertainty is modelled the way it is, and the answer has to be in the
repository rather than in a comment.

This file fixes what a decision record is. `0001-decision-records.md` is this
decision applied to itself.

## Where they live and what they are called

Every record is a file in `docs/decisions/`, named `NNNN-short-slug.md`, where
`NNNN` is a four-digit number and the slug is lower case with hyphens.

Numbers are assigned in order and are never reused, not even for a record that
was superseded, withdrawn or abandoned. A gap in the numbering is a record that
existed; the gap stays and the record stays with it.

The number is allocated when the record is written, not when it is agreed. Two
records written at the same time take different numbers, and where that has to be
coordinated the issue that asks for the record says which number it takes. The
issues in milestone 2 name their numbers for that reason.

One decision per file. A record that answers two questions is two records, for
the same reason a commit carrying two changes has a message describing one of
them.

## What a record has to contain

Every record has these sections, in this order, under these headings.

**Title.** A first-level heading of the form `NNNN. What was decided`, stated as
the decision rather than as the topic. "The input format is a text format" rather
than "Input format".

**Status.** One of `Proposed`, `Accepted`, `Superseded by NNNN`, or `Withdrawn`.
A record with status `Proposed` is a record nothing may depend on yet.

**Date.** The date the record reached its current status, as `YYYY-MM-DD`. Where
a record was proposed on one date and accepted on another, both are given, each
against its status.

**The question.** What had to be decided, and what was at stake in it. A reader
who does not already know the project should understand from this section why the
question could not simply be left open.

**The options considered.** Every option that was genuinely in play, including
the ones that were rejected quickly, each with what it would have cost. An option
list containing only the option taken is a rationalisation, not a record.

**The option taken.** What was decided, stated precisely enough that a reader can
tell whether a given piece of code complies with it.

**The reasons.** Why that option and not the others. This section refers back to
the options by name and says what tipped it.

**What this costs.** What the project gives up by deciding this way. Every real
decision costs something; a record that names no cost has either not found it or
is hiding it. Where the cost falls on a user rather than on the project, say
which user.

**What would falsify this.** What observation, measurement or event would show
the decision to be wrong. This section is not optional and it is not decoration.
A decision that names nothing that would change it is a preference, and a
preference is recorded as one: the section then says so in those words rather
than being left empty or filled with a formality.

**Evidence.** Optional, and present wherever the decision rests on a number, a
measurement or a reading. Each item carries the command, the file or the source
that produced it. A number in a record with nothing behind it is the defect class
this project exists against, appearing in the document that is supposed to
prevent it.

## Superseding, and why records are never edited

A record is added and superseded, never edited in place.

When a decision changes, a new record is written with the next free number. It
carries its own full set of sections and it states in its question section which
record it replaces and why that record failed. The old record keeps its text
unchanged and gains exactly two things: its status becomes `Superseded by NNNN`
and its date becomes the date of the supersession.

Nothing else in a superseded record is touched. Not a typo, not a broken link,
not a statement that later turned out to be wrong. A reader of an old commit has
to be able to see what was believed then, and a record quietly corrected after
the fact is a record that lies about what the code was built on.

The one exception is mechanical and is stated so that it is not confused with the
rule: if a record's file has to move because the directory moves, the file moves
and its content does not change.

## The relationship between records and issues

An issue asks for a decision. A record is the decision. The issue closes when the
record lands, and after that the record is the authority. Where the two disagree,
the record is right and the issue is history.

A record may name an issue where an open question that the decision does not
settle is tracked. It may not defer a section to an issue. A record whose "what
would falsify this" section says "to be determined in issue N" has not made a
decision.

Some questions belong to the maintainer and to nobody else, and those are
collected in one place on the tracker rather than being scattered. A record does
not answer one of those. Where a decision touches such a question, the record
says which question it is, states what the decision does under each possible
answer if that is knowable, and is written so that the maintainer's answer
supersedes it cleanly.

## The voice

The same voice as everything else here. A measured thing is called measured and
an assumed thing is called assumed, and the two are different words in the same
sentence rather than the same word in two places. Where a record states a number,
the number carries the command or the source that produced it. Where a record
states something the project has not verified, it says so.
