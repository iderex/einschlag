# The parity target

The public repository `iderex/jellyfin-plugin-sso` runs a considerably stronger
set of automated controls than this project has. That set is the target here. It
is not copied, because it was built for a different language and a different kind
of risk, but every control it has is either taken, adapted, or declined with a
reason.

This document writes the target down. The issues after it in milestone 8 carry out
what it decides.

Which of the resulting checks become required before a merge is a maintainer
decision and is not decided here.

## Where the list comes from

Derived on 2026-08-07 from the repository itself. A list retyped from memory
goes stale the day that repository changes.

```
$ gh api repos/iderex/jellyfin-plugin-sso/contents/.github/workflows --jq '.[].name'
build.yml
codeql.yml
dco.yml
dependency-review.yml
dotnet.yml
e2e-login.yml
fuzz.yml
manifest-freshness.yml
nightly-betas.yml
opengrep.yml
pr-hygiene.yml
prettier.yml
publish-beta.yml
publish-failure-alert.yml
publish-jf12-beta.yml
publish-jf12-stable.yml
publish.yml
regenerate-manifest.yml
scorecard.yml
stryker-mutation.yml
unicode-guard.yml
wiki-lint.yml
zizmor.yml
```

```
$ gh api repos/iderex/jellyfin-plugin-sso/rulesets --jq '.[] | {name, id}'
{"id":18802863,"name":"Protect main and 5.0"}
```

Reading the ruleset is the point of the second command. The difference between a
control that runs and a control that must be green before a merge is what
distinguishes the two repositories most sharply.

```
$ gh api repos/iderex/jellyfin-plugin-sso/rulesets/18802863 --jq '{enforcement, bypass: [.bypass_actors[]?.actor_type], rules: [.rules[].type], required_checks: [.rules[] | select(.type=="required_status_checks") | .parameters.required_status_checks[].context]}'
{"bypass":[],"enforcement":"active","required_checks":["build","ABI floor build","Package (JPRM) / Build package","Package (JPRM) / Generate SBOM","CodeQL","Analyze (csharp)","DCO sign-off","Deterministic PR-hygiene checks","Enforce greppable invariants","Reject Trojan Source Unicode","Audit workflows (zizmor)","prettier","dependency-review"],"rules":["deletion","non_fast_forward","required_status_checks","pull_request"]}
```

The same commands against this repository, run at the same time, are what the
table below is measured against:

```
$ gh api repos/iderex/einschlag/contents/.github/workflows --jq '.[].name'
dco.yml
dependency-review.yml
scorecard.yml
unicode-guard.yml
zizmor.yml
```

```
$ gh api repos/iderex/einschlag/rulesets --jq '.[] | {name, id}'
{"id":20532905,"name":"gate"}
$ gh api repos/iderex/einschlag/rulesets/20532905 --jq '{enforcement, bypass: [.bypass_actors[]?.actor_type], rules: [.rules[].type], required_checks: [.rules[] | select(.type=="required_status_checks") | .parameters.required_status_checks[].context]}'
{"bypass":[],"enforcement":"active","required_checks":[],"rules":["deletion","non_fast_forward","pull_request"]}
```

Thirteen required checks there, none here. Both rulesets are active and neither
has a bypass actor. That is the gap in one number, and it is a gap in what is
required rather than only in what exists.

The display names below come from

```
$ gh api repos/iderex/jellyfin-plugin-sso/actions/workflows --paginate --jq '.workflows[] | "\(.path)\t\(.name)\t\(.state)"'
```

and its output is not repeated in full here; the name column is quoted per row.

## What was read and what was not

The workflow files themselves were not read. Every row below is derived from the
file name, the workflow display name, and whether a check of a matching name
appears in the required list. Where a row makes a claim about what a workflow
does beyond that, the claim is marked as inferred from its name. This is a weaker
reading than opening each file and it is stated so that the table is not taken for
more than it is.

Five entries in the workflow listing come from `dynamic/` paths, outside the
repository's own `.github/workflows`. The platform manages them and nobody here
authored them: Copilot, the Copilot cloud agent, Dependabot Updates, Automatic
Dependency Submission, and a second CodeQL entry from GitHub code scanning. They
are not controls this repository could take or decline in the same sense. They are
recorded here and given no rows.

## One row per control

| Control there | Required there | Status here | The one line |
| --- | --- | --- | --- |
| `build.yml`, Build | yes, as `build` | taken | A build check under a fixed name is the first thing this repository owes, and it is what continuous integration in milestone 3 names `build`. |
| `dotnet.yml`, .NET | no | no counterpart | Language-specific to C#; whatever replaces it is whichever toolchain decision 0002 names, and it is not a second control. |
| `codeql.yml`, CodeQL | yes, as `CodeQL` and `Analyze (csharp)` | adapted, and running | Taken in `.github/workflows/codeql.yml`, analysing Rust rather than C#, with `security-extended`. The section below carries what it found. |
| `opengrep.yml`, Repo Invariant Lint (Opengrep) | yes, as `Enforce greppable invariants` | adapted | Pattern-based static analysis over the tree is taken, and the invariants it enforces here are this project's own. |
| `dco.yml`, DCO | yes, as `DCO sign-off` | already present | Running here already, in `.github/workflows/dco.yml`, and not yet required before a merge. |
| `dependency-review.yml`, Dependency review | yes, as `dependency-review` | already present | Running here already and not required. |
| `scorecard.yml`, Scorecard supply-chain security | no | already present | Running here already; it publishes a score rather than gating, there and here. |
| `unicode-guard.yml`, unicode-guard | yes, as `Reject Trojan Source Unicode` | already present | Running here already and not required. |
| `zizmor.yml`, Workflow Security Analysis | yes, as `Audit workflows (zizmor)` | already present | Running here already and not required. |
| `pr-hygiene.yml`, PR Hygiene | yes, as `Deterministic PR-hygiene checks` | taken | The pull-request body is where this project's evidence lives, so a deterministic check on the body is worth more here than there. |
| `prettier.yml`, Prettier Lint | yes, as `prettier` | adapted | Formatting refused rather than reported is already owed by milestone 3, with whatever formatter the chosen language has. |
| `fuzz.yml`, Fuzz (SharpFuzz) | no | adapted | Taken with a different target: there the surface is a login flow, here it is the parser that reads a file supplied from outside, so it is taken and pointed elsewhere. |
| `stryker-mutation.yml`, Stryker mutation testing | no | adapted | Mutation testing is taken with whatever the chosen language offers, because a coverage number without it says only that lines ran. |
| `e2e-login.yml`, E2E Login Harness | no | declined | There is no login and no service here; the equivalent end-to-end artefact is the worked example checked by the suite. |
| `manifest-freshness.yml`, Manifest freshness | no | declined | Guards a plugin manifest that has no counterpart in a tool with no plugin repository. |
| `regenerate-manifest.yml`, Regenerate manifest | no | declined | Same subject as the row above. |
| `publish.yml`, Publish Release | no | deferred | Release publishing belongs to milestone 10 and to a maintainer decision about signing and about publishing before validation exists. |
| `publish-beta.yml`, Publish Beta | no | deferred | Same subject as the row above. |
| `publish-jf12-beta.yml`, Publish JF12 Beta | no | declined | Publishes against a specific host application version; there is no host application here. |
| `publish-jf12-stable.yml`, Publish JF12 Stable | no | declined | Same subject as the row above. |
| `nightly-betas.yml`, Nightly betas | no | deferred | A scheduled publish, and this project publishes nothing until milestone 10 decides it may. |
| `publish-failure-alert.yml`, Publish failure alert | no | deferred | Guards the publish pipeline, so it arrives with the pipeline or not at all. |
| `wiki-lint.yml`, Wiki Lint | no | declined | There is no wiki here, and the operator documentation lives in the tree where the suite can reach it. |
| Ruleset `Protect main and 5.0`: deletion, non-fast-forward, pull request | active, no bypass actors | already present | The `gate` ruleset here carries the same three rules with no bypass actor. |
| Ruleset: required status checks, thirteen contexts | active | not present | The only structural difference between the two rulesets, and the reason a red build merges here today. |

Controls absent because they do not apply and controls absent because nobody got
to them look identical in a table that only lists what was done, which is why the
declined and deferred rows are here at all. The distinction between
them is that a declined control has no counterpart in this subject and a deferred
one has a counterpart that a later milestone owes.

## Controls this project needs that the parity repository does not have

The risks differ, so parity in both directions would be the wrong target. These
belong to this subject and not to that one.

**A calibration job.** A reconstruction tool's central claim is that a region
stated at a given level contains the truth at about that rate. That number drifts
against the code silently, and the drift is invisible precisely because the number
looks authoritative. Nothing in the parity repository has this shape, because
nothing there produces a number whose correctness is statistical. It is the single
most important control this project will have and it is not on the target list
above.

**Honesty conformance tests.** The parity repository has no equivalent because it
has no output a reader can quote out of context as an answer. Here somebody will
want a point, and a machine has to refuse to return one. That is a conformance
test over the public interface, not a linter, and it belongs to this subject
alone.

**A monotonicity property test.** Widening a stated input uncertainty must never
narrow the reported region. No amount of reading finds a violation of this
reliably and no static analysis expresses it. It is the strongest guard available
against a subtle error in the propagation, and its subject exists only in a tool
that propagates uncertainty.

**A no-network test.** The privacy position is that nothing leaves the host. The
parity repository is a plugin inside a server whose whole purpose is network
traffic, so the control is meaningless there and load-bearing here, for an
audience that may be working on a machine that will be searched.

**An example fixed by the suite.** A worked example whose stored artefact is
compared on every run, so that a change altering the numbers fails instead of
quietly updating what the documentation claims.

## Supply chain hygiene, and what the reading found

Five guards run here. A score that nobody has read is a badge, so this section is
the triage: what the supply chain score reports, check by check, and for each one
either that it is satisfied or that it is accepted with the reason and the date.

### The score, with the command and the date

The scoring job publishes to the OpenSSF API, so the number a reader would find
is the number quoted here, at the commit it was computed on and never at a
working tree.

```
$ curl -s "https://api.securityscorecards.dev/projects/github.com/iderex/einschlag" | python -c "
import json,sys
d=json.load(sys.stdin)
print('%s  %s  aggregate %s' % (d['date'], d['repo']['commit'], d['score']))
for c in sorted(d['checks'], key=lambda c: c['name']):
    print('%-24s %3s  %s' % (c['name'], c['score'], c['reason']))
"
2026-08-08T19:44:36Z  e38fc4294a0b60eb4d4700ad994c6f7f8cd47b74  aggregate 5.6
Binary-Artifacts          10  no binaries found in the repo
Branch-Protection          3  branch protection is not maximal on development and all release branches
CI-Tests                  10  13 out of 13 merged PRs checked by a CI test -- score normalized to 10
CII-Best-Practices         0  no effort to earn an OpenSSF best practices badge detected
Code-Review                0  Found 0/13 approved changesets -- score normalized to 0
Contributors               0  project has 0 contributing companies or organizations -- score normalized to 0
Dangerous-Workflow        10  no dangerous workflow patterns detected
Dependency-Update-Tool     0  no update tool detected
Fuzzing                    0  project is not fuzzed
License                   10  license file detected
Maintained                 0  project was created within the last 90 days. Please review its contents carefully
Packaging                 -1  packaging workflow not detected
Pinned-Dependencies       10  all dependencies are pinned
SAST                      10  SAST tool is run on all commits
Security-Policy            4  security policy file detected
Signed-Releases           -1  no releases found
Token-Permissions         10  GitHub workflow tokens follow principle of least privilege
Vulnerabilities           10  0 existing vulnerabilities detected
```

A score of `-1` is the tool saying the check did not apply here, which is a third
state and not a zero. The rows below keep it as one.

The aggregate is the tool's own weighting of those eighteen numbers and it is not
this project's measure of anything. It is quoted because a reader will find it,
and the rows are what the reading is actually about.

### Every check it reports, satisfied or accepted

| Check | Score | Satisfied, or accepted with the reason |
| --- | --- | --- |
| `Binary-Artifacts` | 10 | Satisfied. Nothing executable is tracked. |
| `Branch-Protection` | 3 | Accepted, 2026-08-08. The three warnings about approvers, stale review dismissal, codeowners and last-push approval all describe a second person, and this repository has one committer; a required approver it cannot supply would stop every merge. The fourth warning, no status checks, is the real gap and it is not accepted here: it is entry 3 of #1, a maintainer decision and a repository setting, and this document says elsewhere that a red build merges today. |
| `CI-Tests` | 10 | Satisfied by the guards that already run, and it should be read narrowly: the check counts merged pull requests that had some check attached, not pull requests whose checks were required to be green. |
| `CII-Best-Practices` | 0 | Accepted, 2026-08-08. Earning that badge is a self-assessment questionnaire about a project's practices, answered by the maintainer. It is not refused, it is unstarted, and nothing in this repository changes by scoring it. |
| `Code-Review` | 0 | Accepted, 2026-08-08. It counts approved changesets, and an approval needs a second account. `CONTRIBUTING.md` requires that a pull-request body carry its evidence for exactly this reason, and where a change has had no second reader its body says so. That is weaker than an approval and it is what is available. |
| `Contributors` | 0 | Accepted, 2026-08-08. It counts contributing companies or organisations across the last commits. One person wrote this repository. The number is a fact about the project's size and there is no action behind it. |
| `Dangerous-Workflow` | 10 | Satisfied. |
| `Dependency-Update-Tool` | 0 | Accepted, 2026-08-08, and the least comfortable row here. An automated update tool is not configured. The direct dependency this project has is pinned to an exact version on purpose, argued in `decisions/0013-platform-math-out-of-the-numeric-core.md`, so an automatic bump would be a change to a determinism decision rather than routine hygiene. What is not accepted is going without any route by which a vulnerable dependency becomes visible: `dependency-review.yml` refuses a newly introduced or upgraded dependency with a known vulnerability at any severity, and it only sees a change somebody makes. A vulnerability disclosed against a version already in the tree is caught by neither, and nothing here closes that. |
| `Fuzzing` | 0 | Accepted, 2026-08-08, as a thing owed rather than declined. The surface worth fuzzing is the input parser, which does not exist; #58 carries it and #33 carries the parser. |
| `License` | 10 | Satisfied since entry 1 of #1 was answered and #92 landed the file. |
| `Maintained` | 0 | Accepted, 2026-08-08. The check reports that the repository is less than ninety days old. Nothing can be done about that except waiting, and the warning is correct to make a reader look. |
| `Packaging` | -1 | Not applicable today. It looks for a publishing workflow, and this project publishes nothing until milestone 10 and entry 5 of #1. |
| `Pinned-Dependencies` | 10 | Satisfied, and checked below by its own command. The score is not taken on trust. |
| `SAST` | 10 | Satisfied by the workflow audit running on every commit. Read narrowly: what runs today analyses the workflow files, and static analysis over this project's own source is #56 and does not exist. The check scoring 10 says a tool runs, not that this project's code is analysed. |
| `Security-Policy` | 4 | Accepted, 2026-08-08. `SECURITY.md` exists and the tool found the disclosure text in it. The four points it withheld are for "no linked content found", which wants a hyperlink or an address in the file. The reporting route here is GitHub's private vulnerability reporting, and there is no address. Inventing a mailto to score points would put a contact in the tree that nobody reads, so the four points are declined. |
| `Signed-Releases` | -1 | Not applicable today. There are no releases. Whether artefacts are signed and by whom is entry 5 of #1. |
| `Token-Permissions` | 10 | Satisfied, and checked below by its own command. The score is not taken on trust. |
| `Vulnerabilities` | 10 | Satisfied at the commit above. It is a statement about a moment, not a property, and it is the row most likely to have changed by the time somebody reads this. |

Six rows are accepted rather than satisfied and two are not applicable. None of
them is closed by this document, and three of them, `Branch-Protection`,
`Fuzzing` and `SAST`, name the issue or the decision that would move them.

### Every action pinned, checked by a command

Every action reference in every workflow file, so the set is visible:

```
$ grep -rhoE 'uses: [^[:space:]]+' .github/workflows/ | sort | uniq -c
     11 uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1
      1 uses: actions/dependency-review-action@a1d282b36b6f3519aa1f3fc636f609c47dddb294
      1 uses: actions/upload-artifact@043fb46d1a93c77aae656e7c1c64a875d1fc6a0a
      1 uses: astral-sh/setup-uv@c771a70e6277c0a99b617c7a806ffedaca235ff9
      1 uses: github/codeql-action/analyze@e4fba868fa4b1b91e1fdab776edc8cfbe6e9fb81
      1 uses: github/codeql-action/init@e4fba868fa4b1b91e1fdab776edc8cfbe6e9fb81
      2 uses: github/codeql-action/upload-sarif@e4fba868fa4b1b91e1fdab776edc8cfbe6e9fb81
      1 uses: ossf/scorecard-action@2d1146689b8cda280b9bc96326124645441f03bc
```

And the check itself, which prints any reference that is not a forty-character
commit digest carrying the version in a trailing comment:

```
$ grep -rn 'uses:' .github/workflows/ | grep -vE '@[0-9a-f]{40} # v[0-9]'; echo "exit=$?"
exit=1
```

No output and an exit status of 1 from the second `grep` is the whole result:
nothing failed the pattern. A tag or a branch in place of a digest is a
dependency that changes under the same name, and the trailing comment is what
makes the digest readable to a person, so the pattern requires both.

**This command is not enforcement.** Nothing runs it. It is here so a reader can run it, and
the check the workflow audit does make is the one below.

### Permissions declared narrowly, per file and per job

Every workflow file declares its permissions at the top, and every one of those
declarations is read-only or empty:

```
$ for f in .github/workflows/*.yml; do printf '%-40s ' "$f"; awk '/^permissions:/{f=1; print; next} f&&/^[[:space:]]+/{print "  " $0; next} f{exit}' "$f" | tr '\n' ' '; echo; done
.github/workflows/ci.yml                 permissions:     contents: read
.github/workflows/codeql.yml             permissions: {}
.github/workflows/dco.yml                permissions:     contents: read
.github/workflows/dependency-review.yml  permissions:     contents: read
.github/workflows/scorecard.yml          permissions:     contents: read
.github/workflows/unicode-guard.yml      permissions:     contents: read
.github/workflows/zizmor.yml             permissions: {}
```

The three files that need a write scope grant it on the job rather than on the
file: `scorecard.yml` takes `security-events: write` and `id-token: write` in its
job, `zizmor.yml` takes `security-events: write` in its job, and `codeql.yml`
takes `security-events: write`, `contents: read` and `actions: read` in its job,
each with the reason written beside it.

The workflow security audit runs over the whole repository, at low severity and
above, and it fails closed if any workflow fails to
parse. It passed on the commit the score above was computed at:

```
$ gh run list --repo iderex/einschlag --workflow zizmor.yml --branch main --limit 1 --json databaseId,conclusion,headSha --jq '.[] | "\(.databaseId) \(.conclusion) \(.headSha)"'
31275172113 success e38fc4294a0b60eb4d4700ad994c6f7f8cd47b74
```

### What the code scanning analysis found

The query set is `security-extended` rather than the default, which the table
above records as the choice and `.github/workflows/codeql.yml` configures. It
costs analysis time and a longer findings list, and a findings list nobody
triages trains people to ignore the tab. So the count belongs here too.

On the first run of it, against the branch that landed the workflow:

```
$ gh api "repos/iderex/einschlag/code-scanning/analyses?ref=refs/pull/108/merge" --jq '.[0] | {tool: .tool.name, results: .results_count, rules: .rules_count, category}'
{"category":"/language:rust","results":0,"rules":27,"tool":"CodeQL"}
$ gh api "repos/iderex/einschlag/code-scanning/alerts?ref=refs/pull/108/merge&per_page=100" --jq 'length'
0
```

**Nothing to triage, and that is a weaker statement than it looks.** Twenty-seven
rules ran over about a thousand lines of Rust that parse nothing, open no socket
and take no input from outside this repository. An analysis that finds nothing on
a program with no attack surface has not been shown to find anything; what it has
been shown to do is run, report, and put its result where a person can read it.
The run that will say something is the one after #33 lands a parser.

The extractor version is not pinned by this repository. It comes with the
platform's CodeQL bundle, which moves, so the rule count above is a fact about
the day it ran. It is not a property of the tree:

```
$ gh run view --repo iderex/einschlag --job 93203546914 --log | grep -o 'CodeQL/[0-9.]*'
CodeQL/2.26.2
```

### The dependency licences against the licence this project took

Entry 1 of #1 was answered on 2026-08-08 and `LICENSE` carries AGPL-3.0, so the
check that was waiting on it can be made. The direct dependencies and their
licences, read from the resolved packages:

```
$ cargo metadata --format-version 1 | python -c "import json,sys; d=json.load(sys.stdin); print(sorted((p['name'], p['version'], p['license']) for p in d['packages'] if p['name'] not in ('einschlag', 'einschlag-cli')))"
[('libm', '0.2.16', 'MIT')]
```

One dependency, MIT. MIT code may be combined into a work distributed under
AGPL-3.0; the obligation it carries is that its notice and permission text travel
with any distribution, which is a thing a release has to do rather than a thing
this check settles. `DEPENDENCIES.md` holds the entry.

**Read narrowly.** The licence string is what the package's own manifest declares.
Nothing verifies that the declaration matches the files in the package, and a
crate that changes its licence between releases would leave this reading stale.
The exact version requirement is what bounds that here: the version cannot move
without somebody editing the manifest.

## What this document does not decide

Which of the resulting checks become required before a merge. The two rulesets
above differ in exactly that respect, and the gap has a real cost today: with no
required status check, a red build merges here. Changing that is a repository
setting and a maintainer decision, and the gap is written down here, not
closed.

Whether the parity repository's controls are the right set at all. The target was
chosen because it is a real, running, stronger set on a repository under the same
hand, not because it was derived from first principles for this subject. The five
controls in the section above are the part that was derived for this subject, and
they are the part that matters most.
