# Contributing

## The rule this project works to

**Every asserted fact carries the command that produced it**, run against the
commit being pushed rather than against a working tree.

Where a claim cannot be backed by a command, it is written as a claim and says so.
Where something was not measured, the words are "not measured", and where it was
not checked on a given route, the words are "not evaluated on this route". Those
are different epistemic states and they get different words in the sentence.

This applies to the code, to the documents, to the issues and to the pull-request
bodies. It is the same rule the tool itself is built to enforce on its own output:
a reconstruction states what it cannot exclude rather than presenting a conclusion
as a fact, and a project that made that claim about its software while asserting
unbacked numbers about itself would be worth nothing.

The most common way to break it is not inventing a number. It is quoting a number
from the nearest thing to hand: reading a working checkout and reporting it as the
mainline, or quoting a figure from a document instead of from the thing the
document describes. Run the command against what the reader will have.

## Which parts of that rule a check refuses, and which parts a person has to read

This section exists so that nobody reads the section above as a guarantee.

**Nothing in this repository refuses a claim made without the command that
produced it.** No check reads a pull-request body, an issue body, a commit message
or a document and judges whether an assertion in it is backed. The rule above is
held by whoever reads the change and by nobody else.

What is actually refused, on 2026-08-07, is this. Every non-merge commit in a pull
request must carry a `Signed-off-by:` trailer matching its author, enforced by
`.github/workflows/dco.yml`, which fails closed. A newly introduced or upgraded
dependency with a known vulnerability at any severity is refused by
`.github/workflows/dependency-review.yml`. Bidirectional and invisible Unicode
control characters in tracked text are refused by
`.github/workflows/unicode-guard.yml`, which fails closed on a scanner error
rather than reading a broken scan as a clean tree. Actionable security findings in
the workflow files themselves are refused by `.github/workflows/zizmor.yml`.
Supply-chain hygiene is scored, not gated, by `.github/workflows/scorecard.yml`.

And this is what the branch protection does with those results:

```
$ gh api repos/iderex/einschlag/rulesets --jq '.[] | {name, id}'
{"id":20532905,"name":"gate"}
$ gh api repos/iderex/einschlag/rulesets/20532905 --jq '{enforcement, bypass: [.bypass_actors[]?.actor_type], rules: [.rules[].type], required_checks: [.rules[] | select(.type=="required_status_checks") | .parameters.required_status_checks[].context]}'
{"bypass":[],"enforcement":"active","required_checks":[],"rules":["deletion","non_fast_forward","pull_request"]}
```

The ruleset is active and has no bypass actor. It refuses deletion of the branch,
refuses a non-fast-forward push, and requires a pull request. It requires no
status check, so **a pull request whose checks are red can be merged today**. The
checks listed above run and report; nothing makes their verdict a precondition of
the merge.

That is the gap in one sentence, and closing it is a repository setting and a
maintainer decision rather than a change anybody can make in a pull request. The
comparison against a repository that does require its checks is written up in
`docs/QUALITY-PARITY.md`.

So: read the checks, do not assume the merge read them for you.

## No work without an issue

Every change starts as an issue and lands as a pull request. Direct pushes to
`main` are refused by the ruleset quoted above, for everybody.

An issue says what is wrong, what the evidence is, and what "done" means. If the
evidence is a number, it carries the command that produced it. Nothing enforces
that shape; it is a convention this repository's issues follow and a person has to
notice when one does not.

## Decisions

Every decision that shapes the architecture is written down before the code that
depends on it exists. `docs/decisions/README.md` states what a decision record is,
what sections it has, and why records are added and superseded rather than edited
in place.

If you find yourself making an architectural choice inside a pull request, that is
a decision record that has not been written yet.

## Commits

Commits are signed off. Use `git commit -s`, or add the trailer by hand; it has to
match the commit author exactly, because that is what the gate compares. A
retroactive fix on an existing branch is `git rebase --signoff <base>`.

What the trailer asserts is the Developer Certificate of Origin 1.1, and the text
is `DCO` at the root of this repository. Adding the trailer is a statement that
you wrote the contribution or have the right to submit it, and that you
understand the contribution and the record of it are public and kept
indefinitely. Read the file rather than this paragraph; this paragraph is a
summary and the file is the text. Whether contributions from outside are wanted
at all is a separate question and is not answered by the presence of a sign-off
mechanism, which the last section of this document says at more length.

Commit messages state what changed and what failure it prevents. Where a
correction is being made, they say what was wrong and how it was found. One topic
per commit and per pull request: a commit carrying two unrelated changes has a
message that describes one of them.

Nothing judges what a message says. The route reads messages only for the sign-off
trailer.

## Style

English in artefacts.

State the residual rather than drawing the conclusion. Cite the command. Keep
"verified", "not measured" and "not evaluated on this route" as three different
things.

A negative disclosure never becomes a positive assurance. If a passage admits
something was not done, the admission survives every edit and if anything gets
sharper. A line saying a check was not run, rewritten into a line saying it
passed, is worse than the absence it replaced.

Documents do not enumerate what a command can print. A list of checks written into
a document drifts against the checks, and the drift is invisible. Where this
document quotes a list, it quotes it with the command and the date, which is a
different thing from restating it.

## Pull requests

Use `.github/pull_request_template.md`. It asks for the issue the change closes,
what changed, the commands that were run with their output, and, where the issue
asked for a guard to be shown to bite, the demonstration that it does.

The body is where everything about the change lives. If the body is wrong,
incomplete or out of date, edit the body rather than adding a comment underneath
it.

Where an issue asks for a guard to be proven, the proof is the guard deleted or
the violation introduced on a temporary commit, the suite shown going red, and the
output quoted. A guard nobody has watched fail is a guard nobody knows bites.

## Whether contributions from outside are accepted

Not answered yet. The sign-off gate runs on every pull request, which is the
mechanism for accepting outside work, but the presence of a mechanism is not an
answer to the question, and this document does not turn it into one by implication.

The question is open, together with the licence question it depends on, and it is
tracked with the other maintainer decisions on the issue tracker. Until it is
answered, somebody arriving here should know that they are reading a repository
whose terms for inbound work do not exist yet, rather than finding that out after
spending the effort.

Nothing above is a discouragement and nothing above is an invitation. It is the
state.
