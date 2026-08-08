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

## What a test is for

A test that could not have failed proves nothing. Where a test is written to
discharge an issue's Done-when, the pull request shows it failing for the reason
it names, by inverting the assertion or by breaking what it guards on a temporary
commit that is reverted in the same branch.

That is not a convention this repository can enforce. Nothing reads a pull request
body here, which `CONTRIBUTING.md` states with the command that produced it.

## What this document does not yet say

**Whether the suite needs a display or elevated rights is not stated here, and no
test asserts it.** Two of this project's stated users work on borrowed or
restricted machines, and a suite that quietly grows a test needing either one
excludes them and excludes the continuous integration runner too. Issue #25 adds
the rule and the assertion to this document.

**Nothing outside a developer's machine runs this suite.** The workflows in
`.github/workflows/` check sign-off, dependencies, Unicode and the workflow files;
none of them compiles this code or runs a test. Issue #24.

**No coverage figure is produced and no threshold exists.** Issues #28 and #55.
