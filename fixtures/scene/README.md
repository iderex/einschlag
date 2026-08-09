# Scene files

Input files in the format `docs/decisions/0007-input-format.md` fixes, read by
`crates/einschlag/tests/the_input_parser.rs`.

`two-holes-in-one-wall.toml` is the one file here that parses. Everything under
`refused/` is that file with **one line changed**, and the test asserts that
rather than trusting this sentence: a fixture that could not plausibly have been
written by a person proves less than one that could, and the cheapest way for
that claim to rot is for somebody to edit two lines while fixing something else.

`a-material-table-with-one-row.toml` is the material table those files are read
against. It is a fixture and its row is not a reading; the file says so at more
length. The tracked table is `data/materials.toml` and it has no rows, so with
it every file naming any material is refused, which is the state the project is
in rather than a property of the parser.

`the-worked-example-from-decision-0007.toml` is the worked example out of that
record, byte for byte, and a test asserts it still is. **It is refused, and that
is what it is here for.** The record leaves a value beside `unknown = true` on
the bearing of `A2` deliberately, says in as many words that it is refused, and
says the message has to name the key and both fields. The test holds the parser
to that. The same file is also missing everything
`docs/decisions/0006-frame-and-units.md` requires of a scene and the example does
not show, and the test quotes the whole list rather than only the refusal it was
written for.

Nothing here is real case material. Whether real case material may ever enter
this repository was decided on entry 4 of issue #1 and the answer is that it may
not, in any form, including anonymised.
