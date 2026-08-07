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

Derived on 2026-08-07 from the repository itself rather than remembered. A list
retyped from memory would be stale the day that repository changes.

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

Five entries in the workflow listing come from `dynamic/` paths rather than from
the repository's own `.github/workflows`, and are platform-managed rather than
authored: Copilot, the Copilot cloud agent, Dependabot Updates, Automatic
Dependency Submission, and a second CodeQL entry from GitHub code scanning. They
are not controls this repository could take or decline in the same sense, and they
are recorded here rather than given rows.

## One row per control

| Control there | Required there | Status here | The one line |
| --- | --- | --- | --- |
| `build.yml`, Build | yes, as `build` | taken | A build check under a fixed name is the first thing this repository owes, and it is what continuous integration in milestone 3 names `build`. |
| `dotnet.yml`, .NET | no | no counterpart | Language-specific to C#; whatever replaces it is whichever toolchain decision 0002 names, and it is not a second control. |
| `codeql.yml`, CodeQL | yes, as `CodeQL` and `Analyze (csharp)` | adapted | Code scanning is taken, with the analysis appropriate to the chosen language rather than the C# one, and the extended query set where the platform offers a choice. |
| `opengrep.yml`, Repo Invariant Lint (Opengrep) | yes, as `Enforce greppable invariants` | adapted | Pattern-based static analysis over the tree is taken, and the invariants it enforces here are this project's own rather than that project's. |
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
declined and deferred rows are here rather than omitted. The distinction between
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
test over the public interface rather than a linter, and it belongs to this
subject alone.

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
compared on every run, so that a change altering the numbers fails rather than
quietly updating what the documentation claims.

## What this document does not decide

Which of the resulting checks become required before a merge. The two rulesets
above differ in exactly that respect, and the gap has a real cost today: with no
required status check, a red build merges here. Changing that is a repository
setting and a maintainer decision, and this document records the gap rather than
closing it.

Whether the parity repository's controls are the right set at all. The target was
chosen because it is a real, running, stronger set on a repository under the same
hand, not because it was derived from first principles for this subject. The five
controls in the section above are the part that was derived for this subject, and
they are the part that matters most.
