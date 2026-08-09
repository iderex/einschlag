# Privacy

The input to this tool is the geometry of a place where somebody was shot. Around
it sits case material, names, locations, dates, and the identity of the person
doing the work. Some of the people this is built for are documenting a state's use
of force against its own population, and for them the question of what leaves the
machine is not a compliance exercise.

The position, in one sentence: personal data never leaves the host unless the
operator deliberately federates it, and the tool sends nothing on its own.

## The state of this document

This is written before there is anything to leak, which is the point of writing it
now. There is no code in this repository yet.

That means the statements below are commitments about what will be built, not
measurements of what exists. Nothing here is enforced by a check today. The test
that holds the central claim is owed by issue #63, and until it lands, a reader
should treat this document as a stated intention that a person has to verify by
reading the code, not as a guarantee a machine refuses to break.

Saying so is not a hedge on the commitment. It is the difference between a claim
this project can support and one it cannot, and this project exists because that
difference gets collapsed elsewhere.

## What the tool reads

The input file the operator names on the command line.

Any file that input file refers to, such as a scene imported from an exchange
format, and only where the operator's input names it.

Its own configuration, where the operator supplies one, from a path the operator
names.

Nothing else. It does not scan directories, it does not read files it was not
pointed at, and it does not look for input in default locations the operator did
not choose.

## What the tool writes

The output artefact, the report and the visual, to paths the operator names.

Its own diagnostic output, to the standard output and standard error streams,
which go wherever the operator sends them.

Nothing else. No cache, no temporary file outside the output the operator asked
for, no log file in a home directory, no state directory, no recent-files list.

This is a stronger commitment than "no telemetry" and it is the one that follows
from an operator working on hardware that may be searched. It comes from reading
what that practice requires, recorded in `survey/field-practice.md`: an operator
has to be able to account for what is on the disk, and a file the tool wrote
somewhere the operator does not know about is case material they cannot account
for.

Where a run genuinely cannot avoid a temporary file, it is created inside the
output location the operator named, it is deleted when the run ends, and it is
documented here by name before it ships. Today there is no such file, because
there is no code.

## What the tool sends

Nothing.

There is no telemetry. There is no usage reporting. There is no update check.
There is no crash reporting. There is no licence check, no registration, no
analytics and no error submission.

These are absent by design and not yet to be implemented. That distinction is the
whole content of this section: a tool with no network code today and an intention
to add an update check later is a tool that sends nothing today and will send
something later, and an operator reading this document is entitled to know which
of the two this is. It is the first.

## What an operator can choose to send

Nothing, today.

If a route out ever exists, it obeys all three of these rules, and this document
is amended before it ships rather than after:

It is off unless the operator turns it on. Not off by default with a prompt on
first run, which is a prompt people click through. Off, requiring an explicit act.

It names itself when it is on. Every run that has such a route enabled says so in
its own output and in the run manifest that travels inside the artefact, so that
an artefact carries the fact that the run could talk to something.

It states exactly what it would transmit, field by field, in this document, before
it exists. A route described as sending "diagnostic information" is a route whose
contents nobody can check.

## What the tool writes to disk that could identify a case or a person

An operator handling a machine that may be searched needs to know what is on it.
These are the things the tool puts there that could tie the disk to a case or a
person. None of them is the tool's choice; they are the operator's data, and the
list exists so that the operator can find all of it.

The output artefact contains the reconstruction and, inside it, the run manifest.
The manifest carries the input file name as the operator gave it, which is
frequently a case number or a name, and a cryptographic digest of the input bytes.
The digest is not reversible, and it does link an artefact to a specific input
file if both are held.

The output artefact contains the scene geometry, which describes a real place and
can be matched to it.

The report and the visual contain the same, in a form a person can read
immediately.

The input file itself, which the operator wrote, contains everything they put in
it. This is the largest identifying object in the set and the tool neither created
it nor can constrain it.

Nothing in this list is a surprise except the input file name inside the manifest,
and that is there because the manifest exists to make a run reproducible. An
operator who does not want a case number inside an artefact should not name the
input file after the case, and this document says so rather than solving it
silently, because a tool that rewrote the operator's file names would be
destroying the traceability the manifest is for.

## What this project does not control

The names the operator gives to their files and directories.

The case management, evidence system or storage the tool's output is filed into.

Whatever the artefacts are attached to afterwards, including any report, email or
disclosure bundle.

The operator's machine, its disk encryption, its backups and its synchronisation
to any service.

Any network the operator's machine is on, including whatever else on that machine
is talking to it.

A privacy document that claims more than the software can deliver is worth less
than none, so these are named rather than glossed. The tool's commitment stops at
its own reads, writes and sends. Everything above is on the operator and their
organisation, and the published practice this audience already works to, read in
`survey/field-practice.md`, places it there too.

## What refuses a violation of "What the tool sends", and what does not

The claim in "What the tool sends" is the one an adversary would most want to be
false. Until #63 landed it was held by this document and by whoever read the
source. Part of it is now held by a check, and the part that is not is the
larger one.

**What is refused.** `crates/einschlag/tests/nothing_goes_out.rs` reads the
resolved dependency graph and refuses a package that is not on a declared list,
and a package whose name says it opens connections. The judgement is made against
the graph rather than against the source, so a network stack that arrived five
levels down behind a dependency nobody looked past fails the same way a direct one
does. Nothing can enter the build without somebody adding its name to that list
and reading the reason written beside it.

**Two more checks, and neither of them is about a package name.** #96 raised that
gap and these close the part of it that can be closed without a format parser or
an elevated run.

`crates/einschlag-cli/tests/nothing_goes_out_of_the_artefact.rs` reads the binary
Cargo built, which is the thing an operator runs, and refuses the names a socket
is opened through appearing anywhere in its bytes. A linker records the name of
every symbol and every library a program reaches outside itself, and it records
them as text, so a program that can reach a socket says so in the file.

`crates/einschlag/tests/nothing_goes_out.rs` also refuses the standard library's
own networking names in this repository's source. That is the case the graph
check cannot see at all, because the standard library is not a package in the
graph.

**What is still not refused.** The artefact check is a search rather than a
parse: nothing reads an import table or a symbol table, so a statically linked
program reaching a socket through a syscall it makes by hand records no name and
passes. The source check compares substrings, so a name reached through a
re-export somebody wrote to hide it passes. The packages on the declared list are
still not shown to open no socket. And **no run of this tool is observed by
anything**: nothing here starts the binary under a policy that would fail it for
opening a socket, which is the second of the three shapes #96 named and is the
one not taken.

So the position today: a network capability cannot arrive here unnoticed inside
somebody else's crate, it cannot be written here by hand under either of the two
names these checks know, and it is not established by observing the program run.

The graph is read out of `Cargo.lock`, which Cargo writes before it builds and
which is untracked until #26. The check refuses a file it cannot read rather than
reporting an empty graph, because an empty answer reads exactly like a clean one.
