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
runner and not of this repository, and it matters because four "0 passed" lines
and one real one look at a glance like a suite that ran. The per-binary lines:

```
$ cargo test 2>&1 | grep -E '^running |^test result:'
running 1 test
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
running 1 test
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
running 3 tests
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.20s
running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

The total, derived from those lines rather than counted by hand:

```
$ cargo test 2>&1 | awk '/^test result:/ { n += $4 } END { print n }'
5
```

The fourth binary is the documentation tests, and it runs zero because no public
item carries an example yet. That is a real zero and it is left visible rather
than filtered out of the command above.

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
wherever its absence matters. Issue #53 is the first of those, for the hardware
harness.

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
