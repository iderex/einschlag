//! What the parser accepts and what it refuses, against files a person can read.
//!
//! Every refusal issue #33 names has a file under `fixtures/scene/refused/`, and
//! every one of those files is `fixtures/scene/two-holes-in-one-wall.toml` with
//! one line changed. The one-line property is asserted here rather than claimed
//! in a comment: a fixture that could not plausibly have been written by a person
//! proves less than one that could, and nothing else in this repository would
//! notice a fixture drifting into an implausible one.
//!
//! The fixtures are read off disc rather than embedded, because
//! `fixtures/README.md` says a fixture is a file the same suite, an end-to-end
//! run and a person checking by hand all read. Reading them here is what makes
//! that true.

use std::fs;
use std::path::{Path, PathBuf};

use einschlag::input::{self, Fault};
use einschlag::materials::MaterialTable;
use einschlag::measurement::{self, Deformation, Spread};
use einschlag::scene;

/// The file every other one under `refused/` is one line away from.
const VALID: &str = "fixtures/scene/two-holes-in-one-wall.toml";

/// The material table the fixtures are read against, which is not the tracked
/// one. `fixtures/scene/a-material-table-with-one-row.toml` says why.
const TABLE: &str = "fixtures/scene/a-material-table-with-one-row.toml";

/// The worked example out of `docs/decisions/0007-input-format.md`.
const WORKED_EXAMPLE: &str = "fixtures/scene/the-worked-example-from-decision-0007.toml";

/// The directory holding one file per refusal.
const REFUSED: &str = "fixtures/scene/refused";

#[test]
fn a_valid_two_hole_scene_parses_into_the_types_the_reconstruction_is_built_from() {
    let input = read(VALID).unwrap_or_else(|why| panic!("{}", why.join("\n")));

    assert_eq!(input.format_version(), input::FORMAT_VERSION);
    assert_eq!(input.name(), Some("synthetic example, not a real case"));
    assert!(
        input.origin().contains("north and west walls"),
        "the origin description did not survive: {:?}",
        input.origin()
    );
    assert!(
        input.reference_direction().contains("grid north"),
        "the reference direction did not survive: {:?}",
        input.reference_direction()
    );

    let scene = input.scene();
    assert_eq!(scene.surfaces().len(), 1);
    assert_eq!(scene.holes().len(), 2);
    assert_eq!(scene.surfaces()[0].outline().len(), 4);
    assert_eq!(scene.surfaces()[0].contour(), scene::Contour::Flat);
    assert_eq!(scene.holes()[0].id(), "A1");
    assert_eq!(scene.holes()[0].deformation(), Deformation::Moderate);
    assert_eq!(scene.holes()[1].deformation(), Deformation::Severe);
    assert_eq!(
        scene.surface_of(&scene.holes()[0]).map(scene::Surface::id),
        Some("wall-north")
    );

    // The trajectory key is carried and nothing infers it. Issue #38 is where
    // holes on one trajectory are combined; this only has to survive the read.
    assert_eq!(
        input.trajectories(),
        [
            ("A1".to_owned(), "T1".to_owned()),
            ("A2".to_owned(), "T1".to_owned())
        ]
    );
}

#[test]
fn every_measured_value_arrives_in_metres_and_radians_whatever_it_was_written_in() {
    let input = read(VALID).unwrap_or_else(|why| panic!("{}", why.join("\n")));
    let scene = input.scene();
    let hole = &scene.holes()[0];

    // 14.8 mm, written in millimetres, read as metres.
    close(
        hole.perforation()
            .major_axis()
            .value()
            .expect("the major axis has a value"),
        0.0148,
        "the major axis in metres",
    );
    match hole.perforation().major_axis().spread() {
        Spread::StandardDeviation(sd) => close(sd, 0.0006, "the axis deviation in metres"),
        other => panic!("the axis spread came back as {other:?}"),
    }

    // 63 degrees, written in degrees, read as radians.
    close(
        hole.perforation()
            .bearing()
            .value()
            .expect("the bearing has a value"),
        1.099_557_428_756_427_6,
        "the bearing in radians",
    );

    // The surface normal was written as an interval and stays one.
    match scene.surfaces()[0].normal_azimuth().spread() {
        Spread::Interval { low, high } => {
            close(low, 3.106_686_068_549_907, "the lower bound in radians");
            close(high, 3.176_499_238_629_679_8, "the upper bound in radians");
        }
        other => panic!("the azimuth spread came back as {other:?}"),
    }

    // The extent is in metres and is exactly what was written, never widened.
    close(scene.extent().x.high(), 12.0, "the extent in metres");
}

#[test]
fn a_bearing_nothing_was_established_about_carries_no_value() {
    let input = read(VALID).unwrap_or_else(|why| panic!("{}", why.join("\n")));
    let bearing = input.scene().holes()[1].perforation().bearing();
    assert_eq!(bearing.value(), None);
    assert_eq!(bearing.spread(), Spread::Unestablished);
}

/// One refusal the issue names: the file that carries it, what it has to be
/// refused as, and what to call it in a message.
type Case = (&'static str, fn(&Fault) -> bool, &'static str);

/// Each refusal issue #33 names, against the file that carries it, asserting
/// what was refused and that the message names the file and the line.
#[test]
fn each_refusal_the_issue_names_has_a_fixture_that_produces_it() {
    let cases: &[Case] = &[
        (
            "unknown-format-version.toml",
            |fault| matches!(fault, Fault::FormatVersionNotRead { found } if found == "2"),
            "a format version this build does not read",
        ),
        (
            "a-hole-with-no-uncertainty.toml",
            |fault| matches!(fault, Fault::UncertaintyAbsent { .. }),
            "a measured value with none of the three uncertainty forms",
        ),
        (
            "a-minor-axis-longer-than-its-major-axis.toml",
            |fault| {
                matches!(
                    fault,
                    Fault::Record {
                        refusal: measurement::Refusal::MinorAxisExceedsMajor,
                        ..
                    }
                )
            },
            "a minor axis longer than its major axis",
        ),
        (
            "a-hole-in-a-surface-the-scene-does-not-contain.toml",
            |fault| {
                matches!(
                    fault,
                    Fault::Scene {
                        refusal: scene::Refusal::UnknownSurface { .. }
                    }
                )
            },
            "a hole naming a surface the scene does not contain",
        ),
        (
            "a-material-the-table-has-no-row-for.toml",
            |fault| matches!(fault, Fault::MaterialHasNoRow { .. }),
            "a hole in a material the table has no row for",
        ),
        (
            "a-duplicate-identifier.toml",
            |fault| {
                matches!(
                    fault,
                    Fault::Scene {
                        refusal: scene::Refusal::DuplicateIdentifier(_)
                    }
                )
            },
            "a duplicate identifier",
        ),
        (
            "a-value-that-is-not-a-number.toml",
            |fault| matches!(fault, Fault::ValueNotOfKind { .. }),
            "a value that is not a number",
        ),
        (
            "a-negative-length.toml",
            |fault| {
                matches!(
                    fault,
                    Fault::Record {
                        refusal: measurement::Refusal::NegativeLength,
                        ..
                    }
                )
            },
            "a negative length",
        ),
    ];

    for (file, wanted, what) in cases {
        let path = format!("{REFUSED}/{file}");
        let refusals = input::read(&text(&at(&path)), &table())
            .err()
            .unwrap_or_else(|| panic!("{file} was accepted, and it is {what}"));
        let messages = refusals.messages_from(&path);
        assert_eq!(
            refusals.list().len(),
            1,
            "{file} is one line from a valid file and produced more than the one \
             refusal that line is for:\n  {}",
            messages.join("\n  ")
        );
        let refusal = &refusals.list()[0];
        assert!(
            wanted(&refusal.fault),
            "{file} was refused, and not as {what}: {:?}",
            refusal.fault
        );
        assert_eq!(
            refusal.line,
            Some(differing_line(file)),
            "{file} was refused at a line other than the one it changed"
        );
        assert!(
            messages[0].starts_with(&format!("{path}:{}: ", differing_line(file))),
            "the message does not name the file and the line: {:?}",
            messages[0]
        );
    }

    // The reader is only worth anything if it looked at every file that is
    // there. A fixture added under refused/ and left out of the list above
    // would otherwise be a file nothing reads.
    let on_disc = files_in(&at(REFUSED));
    let listed: Vec<&str> = cases.iter().map(|(file, _, _)| *file).collect();
    for name in &on_disc {
        assert!(
            listed.contains(&name.as_str()),
            "{name} is under {REFUSED} and no case above reads it"
        );
    }
    assert_eq!(on_disc.len(), cases.len());
}

/// The claim the fixtures rest on, measured rather than asserted.
#[test]
fn every_refused_fixture_is_the_valid_one_with_exactly_one_line_changed() {
    let valid: Vec<String> = text(&at(VALID)).lines().map(str::to_owned).collect();

    for name in files_in(&at(REFUSED)) {
        let other: Vec<String> = text(&at(&format!("{REFUSED}/{name}")))
            .lines()
            .map(str::to_owned)
            .collect();
        assert_eq!(
            valid.len(),
            other.len(),
            "{name} has a different number of lines from {VALID}, so it is not one \
             line away from it"
        );
        let differing: Vec<usize> = valid
            .iter()
            .zip(&other)
            .enumerate()
            .filter(|(_, (a, b))| a != b)
            .map(|(at, _)| at + 1)
            .collect();
        assert_eq!(
            differing.len(),
            1,
            "{name} differs from {VALID} on lines {differing:?}, and a fixture is one \
             line away from a valid file or it is proving something else"
        );
    }
}

/// The worked example in the record is a fixture, it is refused, and it is
/// refused for the reason the record itself names.
#[test]
fn the_worked_example_from_the_record_is_refused_where_the_record_says_it_is() {
    let refusals = read(WORKED_EXAMPLE).expect_err(
        "the worked example was accepted, and docs/decisions/0007-input-format.md \
         says a value beside unknown = true is refused",
    );

    let bearing: Vec<&String> = refusals
        .iter()
        .filter(|message| message.contains("major_axis_bearing"))
        .collect();
    assert_eq!(
        bearing.len(),
        1,
        "the refusal the record points at is not there, or is there twice:\n  {}",
        refusals.join("\n  ")
    );
    let message = bearing[0];
    for named in ["major_axis_bearing", "value", "unknown"] {
        assert!(
            message.contains(named),
            "the record requires the message to name the key and both fields, and it \
             does not name {named}: {message:?}"
        );
    }

    // The example is also short of everything the scene model requires and the
    // example does not show. That is not a defect in the parser and it is not
    // hidden: the file is refused for those too, and fixtures/scene/README.md
    // is where a reader is sent.
    for owed in [
        "origin",
        "reference_direction",
        "ground",
        "contour",
        "outline",
    ] {
        assert!(
            refusals.iter().any(|message| message.contains(owed)),
            "nothing refused the absence of {owed}:\n  {}",
            refusals.join("\n  ")
        );
    }
}

/// The example in the record and the file here are the same text, so that a
/// record edited without the fixture following it reddens the suite.
///
/// Compared line by line rather than byte by byte, because the line ending is a
/// property of the checkout: `.gitattributes` decides what a clone gets, and a
/// comparison that read a carriage return as a difference would fail on one
/// platform and pass on another for a reason neither file is about.
#[test]
fn the_worked_example_fixture_is_the_record_line_for_line() {
    let record = text(&at("docs/decisions/0007-input-format.md"));
    let lines: Vec<&str> = record.lines().collect();
    let opening = lines
        .iter()
        .position(|line| *line == "```toml")
        .expect("the record has no fenced example");
    let closing = opening
        + 1
        + lines[opening + 1..]
            .iter()
            .position(|line| *line == "```")
            .expect("the example is not closed");

    let fixture = text(&at(WORKED_EXAMPLE));
    assert_eq!(
        lines[opening + 1..closing],
        fixture.lines().collect::<Vec<&str>>(),
        "the worked example in docs/decisions/0007-input-format.md and the fixture \
         taken from it have drifted apart. A record is superseded rather than edited, \
         so this is either a new record owed or a fixture owed."
    );
}

#[test]
fn every_length_unit_the_frame_record_accepts_arrives_as_the_same_length() {
    for (unit, written) in [
        ("m", 0.0148),
        ("cm", 1.48),
        ("mm", 14.8),
        ("in", 0.582_677_165_354_330_8),
        ("ft", 0.048_556_430_446_194_225),
    ] {
        let file = text(&at(VALID)).replace(
            "major_axis = { value = 14.8, unit = \"mm\", sd = 0.6 }",
            &format!("major_axis = {{ value = {written}, unit = \"{unit}\", sd = 0.6 }}"),
        );
        let input = input::read(&file, &table())
            .unwrap_or_else(|why| panic!("{unit} was refused: {:?}", why.messages_from(unit)));
        close(
            input.scene().holes()[0]
                .perforation()
                .major_axis()
                .value()
                .expect("the major axis has a value"),
            0.0148,
            &format!("the major axis written in {unit}"),
        );
    }
}

#[test]
fn a_unit_outside_the_closed_set_is_refused_naming_what_is_accepted() {
    let file = text(&at(VALID)).replace("unit = \"mm\", sd = 0.6 }", "unit = \"rad\", sd = 0.6 }");
    let refusals = input::read(&file, &table()).expect_err("radians were accepted on a length");
    let messages = refusals.messages_from("a file");
    assert!(
        messages
            .iter()
            .any(|message| message.contains("rad") && message.contains("ft")),
        "the refusal does not say what is accepted: {messages:?}"
    );
}

#[test]
fn a_key_this_build_does_not_know_is_refused_rather_than_ignored() {
    let file = text(&at(VALID)).replace("deformation = \"moderate\"", "defromation = \"moderate\"");
    let refusals = input::read(&file, &table()).expect_err("a mistyped key was ignored");
    let messages = refusals.messages_from("a file");
    assert!(
        messages
            .iter()
            .any(|message| message.contains("defromation")),
        "nothing named the key that was not read: {messages:?}"
    );
}

#[test]
fn with_the_tracked_table_every_file_naming_a_material_is_refused() {
    // Not a defect and not a claim about the parser. data/materials.toml has no
    // rows, docs/materials.md argues why, and issue #76 is where that changes.
    // Asserted here so that the day a row lands, this test says so.
    let tracked = MaterialTable::tracked().expect("the tracked table parses");
    let refusals = input::read(&text(&at(VALID)), &tracked)
        .expect_err("a material resolved against a table with no rows");
    let messages = refusals.messages_from(VALID);
    assert!(
        messages
            .iter()
            .all(|message| message.contains("has no row in the material table")),
        "something other than the empty table refused this file: {messages:?}"
    );
    assert_eq!(
        tracked.rows().len(),
        0,
        "the tracked table has rows now, and this test is the one that has to change"
    );
}

/// Read a fixture and give back either the input or one message per refusal,
/// each naming the file it came from.
fn read(relative: &str) -> Result<input::Input, Vec<String>> {
    let text = text(&at(relative));
    input::read(&text, &table()).map_err(|refusals| refusals.messages_from(relative))
}

fn table() -> MaterialTable {
    MaterialTable::parse(&text(&at(TABLE))).expect("the fixture table parses")
}

/// The line a refused fixture changed, derived from the files rather than
/// written down twice.
fn differing_line(name: &str) -> usize {
    let valid = text(&at(VALID));
    let other = text(&at(&format!("{REFUSED}/{name}")));
    valid
        .lines()
        .zip(other.lines())
        .position(|(a, b)| a != b)
        .map(|at| at + 1)
        .unwrap_or_else(|| panic!("{name} is the same as {VALID}"))
}

fn files_in(directory: &Path) -> Vec<String> {
    let mut found: Vec<String> = fs::read_dir(directory)
        .unwrap_or_else(|why| panic!("cannot read {}: {why}", directory.to_string_lossy()))
        .map(|entry| {
            entry
                .expect("the directory entry is readable")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .filter(|name| name.ends_with(".toml"))
        .collect();
    found.sort();
    found
}

fn at(relative: &str) -> PathBuf {
    workspace_root().join(relative)
}

fn text(path: &Path) -> String {
    fs::read_to_string(path)
        .unwrap_or_else(|why| panic!("cannot read {}: {why}", path.to_string_lossy()))
}

fn workspace_root() -> PathBuf {
    let here = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let root = here
        .parent()
        .and_then(Path::parent)
        .expect("this crate sits two levels under the workspace root")
        .to_path_buf();
    assert!(
        root.join("Cargo.toml").is_file(),
        "no workspace manifest at {}",
        root.to_string_lossy()
    );
    root
}

/// Two lengths agreeing to within a tolerance that is tight enough to catch a
/// conversion factor and loose enough not to depend on the last place.
fn close(found: f64, wanted: f64, what: &str) {
    assert!(
        (found - wanted).abs() <= 1e-12 * wanted.abs().max(1.0),
        "{what} came back as {found} and not {wanted}"
    );
}
