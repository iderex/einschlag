# Testing

## The one command

```
cargo test
```

Run it from the root of a clone. It builds both crates, builds the binary the
end-to-end tests start, and runs every test in the workspace.

`docs/BUILD.md` is the prerequisite: the toolchain pin and the build are there,
and this document assumes a clone that already builds.

## Reading the count

Every later milestone states its Done-when in terms of a test. A harness that
silently runs zero tests satisfies all of them, so the number that ran is the
first thing to read, before whether it was green.

**Cargo prints a count per test binary and no total.** That is a property of the
runner and not of this repository, and it matters because a handful of "0 passed"
lines and a few real ones look at a glance like a suite that ran. How many
binaries there are, how many of them run nothing, and the total no line of the
output carries:

```
$ cargo test 2>&1 | grep -c '^running '
15
$ cargo test 2>&1 | grep -c '^running 0 tests'
3
$ cargo test 2>&1 | awk '/^test result:/ { n += $4 } END { print n }'
72
```

Those three numbers move every time a test is added, so re-run the commands
rather than quoting these. They are here to say what the shape of the output is,
not to be a figure anybody cites.

The three zeros are real and are left visible rather than filtered out. Two are
the documentation tests of the two libraries, which run nothing because no public
item carries an example yet. The third is the unit tests of the one run under
`src/bin` in the hardware harness, and that line is the harness being compiled
without being executed, which the section on it below is about.

**On a failing run the total is short, and it is short in the safe direction.**
Cargo stops after the binary that failed, so the binaries behind it never run and
never print a line for the sum to pick up. Measured while proving the harness
bites: the same suite that sums to 5 green summed to 4 with one test failing, the
missing one being the documentation tests that would have contributed zero.
Reading a smaller number after a red run is the runner stopping, not tests
disappearing. Read the exit status first and the count second.

**Nothing refuses a run that executed no tests.** The count is printed and a
person reads it. Making a zero-test run fail is a check nobody has written; it is
not part of #23 and no issue holds it today.

## Where a test goes

Three places, and which one a test belongs in is decided by what it needs.

**Beside the code it tests**, in a `#[cfg(test)] mod tests` block in the same
file. For anything that can be checked by calling a function. These reach private
items, which is the reason to use them, and they compile out of a release build.

**In `crates/<crate>/tests/`**, one file per subject. These see only the crate's
public interface, exactly as a caller does, and Cargo builds the binary before
running them so they can start it as a process. `crates/einschlag-cli/tests/cli.rs`
is the first: it runs the built artefact and reads its exit status and its output.

**In `fixtures/`**, the input a test reads and the output it is compared against.
Material goes there rather than beside the code so that the same file can be read
by a unit test, by an end-to-end run and by a person checking the tool by hand.
It holds nothing yet; the first inputs arrive with the parser in #33.

A run that needs equipment is none of these three, because it is not a test. The
next section is where it goes.

## The hardware harness

Some things cannot be checked without firing at material and measuring what it
did, and some cannot be checked without a total station or a scanner. Those runs
are worth making. They are not tests: they need a person, a bench and an
instrument, they cannot be repeated on demand, and a suite containing one cannot
run on a borrowed machine or on an unattended runner.

They live in `crates/einschlag-hardware-harness`, whose name says what it needs.
**A run there is a binary under `src/bin` and never a test.** That is the whole
separation and it is Cargo's rather than a rule somebody has to remember: the
test runner has no way to start a binary.

### What is in it, what it needs, and what it costs to run

One run so far.

**`record-figure`.** Writes one figure measured with equipment into the line the
calibration report in #52 will read, refusing a figure that does not say which
day it was measured and with what instrument.

```
cargo run -p einschlag-hardware-harness --bin record-figure -- \
    --quantity "perforation major axis" \
    --value 12.4 --unit mm \
    --measured-on 2026-08-08 \
    --equipment "digital caliper, bench 1"
```

What it requires is a measurement already made: an instrument, a bench, and a
person who read a number off it. The program itself drives nothing and invents
nothing. What that costs is the bench work, and no figure for it has been
measured, so none is quoted here.

**No run in this harness drives an instrument today**, and none compares anything
against the core, because the core computes nothing yet. The harness exists now
so that the first run that does has somewhere to go other than the default suite.
Nothing reads a recorded figure either: #52 is the report that would, and it does
not exist.

### What refuses a violation, and what it does not cover

The unit tests in `crates/einschlag-hardware-harness/src/lib.rs` refuse the four
shapes that would undo the separation. Three would put a run into the default
suite: a `tests` directory in the crate, a `[[test]]` target in its manifest, and
a `#[test]` attribute inside a run, which `cargo test` would execute as a unit
test of that binary. The fourth would remove the compilation instead: a
`default-members` list in the workspace manifest that leaves this crate out.

The compilation is the other half of the arrangement. This crate is an ordinary
workspace member, so `cargo test` builds every target in it and a change to the
core that breaks a run is caught by the default suite even though no run is
executed by it. The line

```
Running unittests src\bin\record-figure.rs
```

in a `cargo test` run is that compilation, and the count printed under it is
zero.

**What the guards do not cover.** They read the shape of the crate, not the
meaning of what is in it. A run written into the library's own unit tests rather
than into a binary would be executed by the default suite and nothing here would
refuse it, because nothing can tell a unit test of the recording types from one
that expects a bench. They also say nothing about a continuous integration job
invoking the harness deliberately: no such job exists yet, #24 is where the first
one is written, and what a job runs is decided in the job.

**A figure from a run nobody can repeat is still evidence, and it is weaker
evidence.** The date and the equipment are what keep it readable as the weaker
kind, which is why the record refuses to exist without them. That the report
carrying them says which kind it is holding is #52's half and is not done.

## Headless and unprivileged

**Every test in the default suite runs with no display available and with no
elevated rights. Anything that cannot is not in the default suite.**

Two of this project's stated users work on borrowed or restricted machines, which
`survey/field-practice.md` records. A test that opens a window or asks for
administrator rights excludes them, and it excludes an unattended runner too,
which means the suite reported green is not the suite that ran. It is cheap to
hold now and expensive after the first test that needs a window.

A test that genuinely needs hardware, a display or a privilege is not deleted. It
goes outside the default suite, is named for what it is, and is disclosed
wherever its absence matters. The hardware harness is the first of those and the
section above says where it is and what keeps it out.

### What refuses a violation, and what does not

`crates/einschlag/tests/headless_and_unprivileged.rs` reads every Rust source
file and every manifest in the workspace and refuses two lists of names: the ones
a display or window system is reached through, and the ones an elevation request
is made through. It skips documentation, because a document naming a display
variable is describing it rather than requiring it, and refusing that would stop
this page stating its own rule.

`crates/einschlag-cli/tests/cli.rs` starts the built artefact with an empty
environment and requires it to produce the same output. Nothing it does can
depend on a session, a display variable or anything else a restricted machine
might not provide.

**Neither of those asserts that the run was unprivileged or that no display was
available, and no test here does.** Rust has no declaration of what a test
requires, so there is nothing for a check to read. Reading the running process's
privilege level needs a foreign function call into the platform, and the
alternatives are probes with side effects, such as opening a file only an
administrator can write. Neither belongs in a check that runs on every developer
machine, and #88 holds the mechanism if one is wanted.

So what stands here is a floor. The marker lists hold what somebody would
actually write, and they will not catch a route nobody has written yet. The
dependency budget in `DEPENDENCIES.md` is the other half: a windowing crate
cannot arrive without an entry saying what it is for, which is the point at which
a person sees it.

**Nothing outside a developer's machine runs this suite at all**, so the claim
that it is headless has not been tested by running it somewhere headless. Issue
#24 creates the job, and #25 asks that the job be configured to run the tests as
an unprivileged user with no display server, visible in the configuration rather
than inherited from whatever the runner image happens to be. Until that lands,
the rule above is held by the two checks and by whoever reads a change.

## What a test is for

A test that could not have failed proves nothing. Where a test is written to
discharge an issue's Done-when, the pull request shows it failing for the reason
it names, by inverting the assertion or by breaking what it guards on a temporary
commit that is reverted in the same branch.

That is not a convention this repository can enforce. Nothing reads a pull request
body here, which `CONTRIBUTING.md` states with the command that produced it.

## What this document does not yet say

**Nothing outside a developer's machine runs this suite.** The workflows in
`.github/workflows/` check sign-off, dependencies, Unicode and the workflow files;
none of them compiles this code or runs a test. Issue #24.

**No coverage figure is produced and no threshold exists.** Issues #28 and #55.
