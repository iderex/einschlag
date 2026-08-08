# 0012. The tool refuses a named list of certainty phrases in everything it emits and ships

## Status

Accepted.

## Date

2026-08-08.

## The question

`0010-honesty-rule.md` constrains the shape of the result. No call returns a
single position, every region carries a level, every prior that narrowed the
answer is named. All six of its properties are about values the tool computes.

None of them is about the words the tool prints. A report generator, an error
message, a help text, an example scene or a page of documentation can carry the
sentence "to a reasonable degree of scientific certainty" while every computed
value is impeccable, and the sentence is what gets read into a court record.
`../survey/challenges.md` found that reconstructions are not attacked on their
numbers; they are attacked on the distance between what a method supports and
what the witness said. That is a finding about words.

So the question is whether a list of words can be a property this project
enforces, what it covers, and what it may not be allowed to imply.

## The options considered

**Nothing, and rely on the person writing the text.** Costs nothing to build and
holds for exactly as long as one person writes every string. The failure it
leaves is the one `../survey/challenges.md` documents, and it arrives at the
worst moment: in a report, in front of a court, in a sentence nobody reviewed
because it was boilerplate.

**A style note in the contributing document.** Cheap, readable, and refused by
nothing. This repository's own rule is that a sentence in a document is an
explanation of a rule rather than a rule, and #75 asks for a test.

**A list of single words, refused anywhere.** The strongest-looking option and
the one that fails in practice. PCAST names `minimal`, `negligible`, `zero` and
`microscopic` as words appearing inside a claim about an error rate. Refusing
`minimal` on its own refuses `profile = "minimal"` in this repository's own
toolchain file, which is a true statement about a compiler. A check that fires on
true statements is a check somebody switches off, and a switched-off check is
worse than none because the tree still looks guarded.

**A list of phrases, refused in everything the tool emits and ships.** Narrower
than single words and wide enough to catch the claims the sources actually name.
What it cannot do is catch a phrasing nobody has written yet.

**A list of phrases, refused only in generated report text.** Cheaper, and #75
refuses it by name: a property that covers only the report and is described as
covering the output is worse than none, because the reader then believes the help
text and the documentation were checked.

## The option taken

**A list of phrases, held in one place in the source, refused in every string the
tool can emit and in every text file this repository ships, with a small register
of named exemptions that fails in both directions.**

**Where the list lives.** `crates/einschlag/src/vocabulary.rs`. That file is the
authority and this record does not restate the phrases, because a list in two
places drifts and the drift is invisible. Print it:

    git grep -n 'text: "' -- crates/einschlag/src/vocabulary.rs

**What the list came from.** Two sources and no invention. PCAST, "Forensic
Science in Criminal Courts: Ensuring Scientific Validity of Feature-Comparison
Methods", 2016, in the passage `../survey/challenges.md` quotes from the
recommendations to the courts:

> courts should never permit scientifically indefensible claims such as: "zero,"
> "vanishingly small," "essentially zero," "negligible," "minimal," or "microscopic"
> error rates; "100 percent certainty" or proof "to a reasonable degree of scientific
> certainty;" identification "to the exclusion of all other sources;" or a chance of
> error so remote as to be a "practical impossibility."

And the National Commission on Forensic Science, quoted in the same report, on
"a reasonable degree of scientific certainty" and its relatives having no
scientific meaning. Every entry in the list carries which of the two it came
from, so adding one later is an argued change against a source rather than a
preference.

**How a phrase is matched.** Case is ignored and runs of whitespace are collapsed
to a single space before the comparison. A phrase broken across two lines of a
wrapped document is found, which is the ordinary way one hides from a search in
this repository's own prose.

**What the property covers, precisely.** #75 requires this to be decided rather
than left to be discovered.

Every string the tool can emit. Today that is the usage banner and the version
line. As the tool grows it is the report, the output artefact, every error
message, every warning and every help text.

Every file this repository ships, walked from the workspace root. Documentation,
decision records, the survey, the workflow files, the manifests, the source, and
the worked example and fixtures when they exist. The scan skips `.git` and
`target`, which are not shipped.

It does not cover what the operator writes. The operator writes what they write;
this constrains what the tool puts in their hands with its name on it.

**The exemptions.** Four files have to carry the phrases in order for the
phrases to be refused: the module that holds the list, this record,
`0010-honesty-rule.md`, which declares the property and has to name the phrase to
say what property 6 is about, and `../survey/challenges.md`, which is the survey
the phrases were read out of. Each is named individually in
`crates/einschlag/tests/certainty_vocabulary.rs` with its reason, and the
register fails in both directions. An exemption naming a
file that is not there fails as dangling. An exempt file that no longer carries a
phrase fails as stale and has to be removed rather than left sitting outside the
check.

## The reasons

**Phrases rather than words, because the alternative was measured against this
tree and lost.** `profile = "minimal"` is in `rust-toolchain.toml` at the commit
this record was written on. A word list would have refused the repository's own
toolchain pin on the day it landed.

**The list in the source rather than in this record, because this record is
prose and the source is what runs.** It also keeps the record honest about its
own role: it argues the list and names the sources, and the reader who wants the
list runs the command above.

**The scan covers shipped files and not just emitted strings, because the
documentation is the text most likely to be copied.** An operator reaching for a
sentence to put in a report will take it from the worked example or the
documentation before they take it from a generated string.

**The exemption register fails closed in both directions, because a one-way
register is a place to hide.** An exemption that only had to name a path would
let a file be added to the list and never checked again. Requiring an exempt file
to still contain a phrase means the exemption expires the moment it stops being
needed.

## What this costs

**It cannot be complete, and passing it does not mean the output is honest.**
A phrase list catches the phrasings somebody has already written down. "There is
no doubt the shot came from the window" carries none of the twelve phrases and is
exactly the claim the sources are about. This is the largest cost and it is
stated here rather than added as a caveat: a green run means no known phrasing
was found, and nothing more. Whoever reads a report is still the mechanism for
the rest.

**Four files sit outside the check**, and each one is a real hole: a refused
phrase could be introduced into `../survey/challenges.md`, into
`0010-honesty-rule.md` or into this record and nothing would catch it. The
register bounds the holes by name and count; it does not remove them. Two of the
four are decision records, which are never edited in place, so an exemption on one
of those is effectively permanent.

**It will refuse a legitimate quotation one day.** Somebody will need to quote
what a court was told, in a file that is not exempt, and will meet a red test.
The repair is a new exemption with a reason, which is the correct outcome and is
still an interruption.

**The scan reads every file in the tree on every test run.** Cheap now. If it
becomes slow, the repair is to narrow it, and narrowing it is what #75 refuses
by name, so the argument has to be made properly.

**It says nothing about other languages.** Every phrase is English. A German
report generator would ship unguarded, and the German equivalents of these
claims are not in the list and were not looked for.

## What would falsify this

A phrase in the list turning out to be ordinary technical English somewhere this
project needs it. `essentially zero` is the candidate: a sentence about a
numerical residual could reach for it honestly. If that happens the entry is
wrong as stated and needs narrowing, not an exemption for the file.

The list catching nothing over the life of the project while the reports still
carry overstated claims. That is the phrasing gap above turning out to be the
whole problem, and it would mean the mechanism is theatre. What would show it is
a review of shipped reports finding overstatement that this check passed.

A jurisdiction whose courts refuse a different vocabulary. `../survey/challenges.md`
records that its reading is United States material throughout and that much of
the intended audience is elsewhere. If a European source names different phrases,
this list is the wrong list rather than an incomplete one.

## Evidence

**The words in the list are not this project's.** Both sources are quoted above,
and `../survey/challenges.md` carries the passage with the query that found it.

**The single-word option was tested against the tree rather than argued.** On the
commit this record was written on:

    $ git grep -n 'minimal' -- rust-toolchain.toml
    rust-toolchain.toml:14:profile = "minimal"

**No study, court decision or report was read in full for this record.** It rests
entirely on `../survey/challenges.md`, which states its own bound: the PCAST
passage is quoted from the report PDF, and the National Commission's finding is
quoted at second hand from inside that report rather than from the Commission's
own document. Nobody obtained the latter.

**Nothing measured how often these phrases appear in real reconstruction
reports.** The list is derived from what courts were told to refuse, not from a
count of what practitioners write. No such count was found.
