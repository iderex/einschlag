<!--
Everything about this change goes in this body. If the body turns out to be
wrong, incomplete or out of date, edit the body rather than adding a comment
underneath it.

The rule this repository works to: every asserted fact carries the command that
produced it, run against the commit being pushed rather than against a working
tree. Nothing checks that. A reader does. See CONTRIBUTING.md.
-->

## Closes

<!-- One issue. If this closes more than one, say why they could not land
separately. -->

Closes #

## What changed

<!-- What the change does, and what failure it prevents. Not a diff summary; a
reader can read the diff. -->

## Why this means, and why not something else

<!-- The reasoning. If this involved an architectural choice, say which decision
record covers it, or say that a record is owed and name the issue. -->

## Commands run, with their output

<!-- Quoted verbatim, including the command line. Run against the commit being
pushed. If a command was run against a working tree and not re-run against the
pushed commit, say so here rather than letting the output imply otherwise. -->

```
$
```

## The guard shown to bite

<!-- Where the issue asked for a guard to be proven, this section carries the
proof: the guard deleted or the violation introduced on a temporary commit, the
suite going red, and the output quoted. Then the temporary commit removed.

Where the issue asked for no guard, write "Not asked for by the issue." rather
than deleting the heading, so that a reader can tell a guard that was not
required from one that was skipped. -->

## What was not done

<!-- What is still owed, what was not measured, what was not evaluated on this
route, and what was skipped and why. A skipped test is disclosed here, never
worked around silently.

A negative disclosure never becomes a positive assurance. If this section says
something was not done, it stays saying so. -->

## Checks

<!-- The checks that ran and what they said. Note that the branch protection on
this repository requires no status check, so a red pull request can be merged;
read the results rather than assuming the merge read them. CONTRIBUTING.md
quotes the ruleset with the command that produced it. -->

## Second reader

<!-- Who read this besides its author, or a plain statement that nobody did. -->
