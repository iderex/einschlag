//! Units are written explicitly, converted once, and converted nowhere else.
//!
//! A metre read as a millimetre puts the shooter a thousand times too far away,
//! and the output looks like an ordinary number either way. Nothing downstream
//! can notice it, because nothing downstream is told what unit a value arrived
//! in. `docs/decisions/0006-frame-and-units.md` fixes the internal unit and the
//! closed set a file may be written in, `crates/einschlag/src/input.rs` is the
//! one place a conversion happens, and this file is where that arrangement is
//! held to rather than described.
//!
//! Four properties, and the last is the only one a person could not check by
//! reading.
//!
//! A measured value with no unit is refused rather than read as though it were
//! already in the internal one. Every place the valid fixture writes a unit is
//! swept, not only the one place a fixture on disc takes it away, because a
//! boundary that refuses a bare number under one key and assumes it under
//! another is the same defect with a smaller surface.
//!
//! A unit outside the closed set is refused and the message names what was
//! written. The same sweep covers a length unit written on an angle and an angle
//! unit written on a length, which is the shape that survives a proofread: both
//! spellings are units, both look right, and only one of them belongs there.
//!
//! Two holes written in two units arrive at one internal value.
//! `fixtures/scene/one-hole-in-millimetres-and-one-in-inches.toml` carries the
//! same perforation twice, once off a metric caliper and once off an imperial
//! one. Mixed units in one file are legal; two numbers in different units read
//! as the same number are the failure.
//!
//! No conversion factor appears outside the boundary. The factors are read out
//! of the boundary itself rather than written here, so the two cannot drift, and
//! every other shipped source is searched for the same numbers.
//!
//! # What the factor check reads, and what it does not
//!
//! It reads the shipped part of every Rust source under a crate's `src`, which
//! is everything above that file's `#[cfg(test)]` module. Two in-source test
//! modules convert millimetres in a helper, and they are not the failure this
//! refuses: a helper standing in for the boundary in a unit test cannot ship a
//! value that was converted twice. They are still conversions and they are still
//! there:
//!
//!     git grep -n '/ 1000.0' -- crates/einschlag/src
//!
//! Three shapes go past it, and none of them is hypothetical. A reciprocal
//! written rounded, `39.37` for an inch, is not the reciprocal and is not
//! matched. A factor between two written units rather than between a written
//! unit and the internal one, `25.4` for inches to millimetres, is not in the
//! set. And the reader takes out strings, character literals and line comments
//! but not block comments, so a factor inside `/* */` would be read as code,
//! which errs towards refusing rather than towards passing.
//!
//! The angle half is narrower still and says so at its own test.

use std::fs;
use std::path::{Path, PathBuf};

use einschlag::input::{self, Fault, Input, Refusal, Refusals};
use einschlag::materials::MaterialTable;
use einschlag::measurement::Spread;

/// The file the sweeps are run against, and the one every file under `refused/`
/// is one line away from.
const VALID: &str = "fixtures/scene/two-holes-in-one-wall.toml";

/// The material table the fixtures are read against, which is not the tracked
/// one. `fixtures/scene/a-material-table-with-one-row.toml` says why.
const TABLE: &str = "fixtures/scene/a-material-table-with-one-row.toml";

/// The same perforation written in millimetres and in inches.
const MIXED: &str = "fixtures/scene/one-hole-in-millimetres-and-one-in-inches.toml";

/// An axis whose unit was never typed.
const NO_UNIT: &str = "fixtures/scene/refused/a-measured-value-with-no-unit.toml";

/// An axis in a unit the format does not accept.
const UNKNOWN_UNIT: &str = "fixtures/scene/refused/a-unit-outside-the-closed-set.toml";

/// A unit outside the closed set, spelled the way somebody who has not read the
/// accepted list would spell inches.
const NOT_A_UNIT: &str = "inch";

/// The one file allowed to carry a length conversion factor, because it is the
/// boundary the conversion happens at.
const BOUNDARY: &str = "crates/einschlag/src/input.rs";

/// The one file allowed to carry the angle conversion, because it is the
/// arithmetic the boundary calls rather than a second conversion.
const ANGLE_ARITHMETIC: &str = "crates/einschlag/src/math.rs";

/// The names this crate's own angle conversion is reached through.
///
/// The platform's methods of the same names are a different subject, and
/// `crates/einschlag/tests/platform_math_stays_out_of_the_core.rs` is what
/// refuses those.
const ANGLE_CONVERSIONS: [&str; 2] = ["to_radians", "to_degrees"];

/// How close a number has to be to the reciprocal of a factor to be read as one.
///
/// A reciprocal is not exactly representable. `1000.0` is what a person writes
/// for the reciprocal of `0.001`, and dividing one by the stored factor does not
/// produce it. The product is compared against one instead, which is exact for
/// every factor the boundary carries and leaves a rounded reciprocal outside, as
/// the module documentation says.
const RECIPROCAL_TOLERANCE: f64 = 1e-12;

/// How close two lengths have to be to be the same length.
///
/// Tight enough that any wrong factor is caught, since the smallest ratio
/// between two accepted units is ten, and loose enough that the last place of a
/// multiplication is not a failure.
const SAME_LENGTH: f64 = 1e-12;

#[test]
fn a_measured_value_with_no_unit_is_refused_rather_than_read_as_the_internal_one() {
    let valid = text(&at(VALID));
    let file = text(&at(NO_UNIT));
    let changed = differing_line(&valid, &file);

    let refusals = refusal_of(&file, NO_UNIT);
    assert_eq!(refusals.list().len(), 1, "{}", listed(&refusals));
    let absent = only_matching(&refusals, |fault| matches!(fault, Fault::UnitAbsent { .. }));
    assert_eq!(absent.line, Some(changed));
    assert!(
        absent.message().contains("major_axis"),
        "the message does not say which value it was: {:?}",
        absent.message()
    );

    // The digits are unobjectionable, and that is the whole difficulty. The same
    // line parses under either unit and means two different lengths, so a
    // boundary that guessed would be guessing between these two.
    let site = site_on(&valid, changed);
    same_length(
        major_axis_of(&with_the_unit(&valid, &site, "m")),
        14.8,
        "the axis read as metres",
    );
    same_length(
        major_axis_of(&valid),
        0.0148,
        "the axis read as the millimetres it was written in",
    );
}

#[test]
fn a_unit_outside_the_closed_set_is_refused_and_the_message_names_what_was_written() {
    let valid = text(&at(VALID));
    let file = text(&at(UNKNOWN_UNIT));
    let changed = differing_line(&valid, &file);

    let refusals = refusal_of(&file, UNKNOWN_UNIT);
    assert_eq!(refusals.list().len(), 1, "{}", listed(&refusals));
    let unknown = only_matching(&refusals, |fault| {
        matches!(fault, Fault::UnitNotKnown { .. })
    });
    assert_eq!(unknown.line, Some(changed));

    let Fault::UnitNotKnown { found, .. } = &unknown.fault else {
        unreachable!("the refusal was selected on that variant")
    };
    assert_eq!(found, NOT_A_UNIT);

    let message = unknown.message();
    assert!(
        message.contains(NOT_A_UNIT),
        "the message does not name what was written: {message:?}"
    );
    for (accepted, _) in accepted_length_units() {
        assert!(
            message.contains(&accepted),
            "the message does not offer {accepted}, which the boundary accepts: \
             {message:?}"
        );
    }
}

#[test]
fn every_place_the_format_writes_a_unit_refuses_the_value_when_the_unit_is_gone() {
    let valid = text(&at(VALID));
    let sites = unit_sites(&valid);
    assert_eq!(
        sites.iter().map(|site| site.line).collect::<Vec<usize>>(),
        lines_carrying_a_unit(&valid),
        "the sweep and the file disagree about where the units are, so the sweep \
         covers something other than the file"
    );

    for site in &sites {
        let file = without_the_unit(&valid, site);
        let refusals = refusal_of(&file, &format!("{VALID} with line {} bare", site.line));
        let absent = only_matching(&refusals, |fault| matches!(fault, Fault::UnitAbsent { .. }));
        assert_eq!(
            absent.line,
            Some(site.line),
            "the unit taken off line {} was refused at another line",
            site.line
        );
    }
}

#[test]
fn every_place_the_format_writes_a_unit_refuses_one_outside_the_closed_set() {
    let valid = text(&at(VALID));

    for site in &unit_sites(&valid) {
        let file = with_the_unit(&valid, site, NOT_A_UNIT);
        let refusals = refusal_of(
            &file,
            &format!("{VALID} with {NOT_A_UNIT} on line {}", site.line),
        );
        let unknown = only_matching(&refusals, |fault| {
            matches!(fault, Fault::UnitNotKnown { .. })
        });
        assert_eq!(unknown.line, Some(site.line));
        assert!(
            unknown.message().contains(NOT_A_UNIT),
            "the message does not name what was written: {:?}",
            unknown.message()
        );
    }
}

/// The mistake a proofread does not catch: a real unit, written on the wrong
/// kind of quantity.
#[test]
fn a_length_unit_on_an_angle_and_an_angle_unit_on_a_length_are_both_refused() {
    let valid = text(&at(VALID));
    let lengths = accepted_length_units();
    let mut angles = 0;
    let mut sites = 0;

    for site in &unit_sites(&valid) {
        let is_a_length = lengths.iter().any(|(name, _)| *name == site.unit);
        let wrong_kind = if is_a_length { "deg" } else { "mm" };
        sites += 1;
        if !is_a_length {
            angles += 1;
        }

        let file = with_the_unit(&valid, site, wrong_kind);
        let refusals = refusal_of(
            &file,
            &format!("{VALID} with {wrong_kind} on line {}", site.line),
        );
        let unknown = only_matching(&refusals, |fault| {
            matches!(fault, Fault::UnitNotKnown { .. })
        });
        assert_eq!(unknown.line, Some(site.line));
    }

    // Both directions were exercised. A fixture carrying only one kind of
    // quantity would leave half of this passing on nothing.
    assert!(angles > 0, "{VALID} writes no angle");
    assert!(sites > angles, "{VALID} writes no length");
}

#[test]
fn two_holes_written_in_two_units_arrive_at_one_internal_value() {
    let input = parsed(&text(&at(MIXED)), MIXED);
    let scene = input.scene();
    assert_eq!(scene.holes().len(), 2);

    let millimetres = scene.holes()[0].perforation();
    let inches = scene.holes()[1].perforation();

    for (what, written, converted, wanted) in [
        (
            "the major axis",
            millimetres.major_axis(),
            inches.major_axis(),
            0.0127,
        ),
        (
            "the minor axis",
            millimetres.minor_axis(),
            inches.minor_axis(),
            0.008_89,
        ),
    ] {
        let from_millimetres = written.value().expect("the axis has a value");
        let from_inches = converted.value().expect("the axis has a value");
        same_length(from_millimetres, from_inches, what);
        same_length(from_millimetres, wanted, &format!("{what} in metres"));

        match (written.spread(), converted.spread()) {
            (Spread::StandardDeviation(one), Spread::StandardDeviation(other)) => {
                same_length(one, other, &format!("{what} deviation"));
                same_length(one, 0.000_635, &format!("{what} deviation in metres"));
            }
            other => panic!("{what} came back with the spreads {other:?}"),
        }
    }

    // The centre of the second hole is in centimetres, so the mixing is across
    // keys and not only across holes.
    same_length(
        scene.holes()[1]
            .centre()
            .x
            .value()
            .expect("the centre has a value"),
        4.905,
        "the centre in metres",
    );
}

#[test]
fn no_length_conversion_factor_appears_outside_the_boundary() {
    let factors = conversion_factors();
    let mut offences = Vec::new();

    for (name, shipped) in shipped_sources() {
        if name == BOUNDARY {
            continue;
        }
        for (number, line) in shipped.lines().enumerate() {
            for value in numbers_in(&without_strings_or_comments(line)) {
                if let Some(factor) = matching_factor(&factors, value) {
                    offences.push(format!(
                        "{name}:{}: {value} converts a length to or from {factor} \
                         metres: {}",
                        number + 1,
                        line.trim()
                    ));
                }
            }
        }
    }

    assert!(
        offences.is_empty(),
        "a length conversion factor appears outside {BOUNDARY}, at {offences:#?}. \
         Everything on this side of the boundary is already in the internal unit \
         docs/decisions/0006-frame-and-units.md fixes, so a second conversion \
         produces a value wrong by a whole factor that reads as an ordinary \
         number. Convert at the boundary. If the number is not a conversion at \
         all, it is a bare literal nobody can read the meaning of, and it needs a \
         name either way."
    );
}

/// The angle direction of the same rule, and it is narrower than the length one.
///
/// What is refused is a call to this crate's own conversion outside the boundary
/// and outside the arithmetic itself. The outward direction, radians back to
/// degrees for something a person reads, has no second boundary yet, because
/// nothing renders anything at this commit. When one exists it will trip this,
/// and the exemption belongs there with its reason rather than being written
/// loose now against a caller nobody has yet had to design.
#[test]
fn no_angle_conversion_happens_outside_the_boundary() {
    let mut offences = Vec::new();

    for (name, shipped) in shipped_sources() {
        if name == BOUNDARY || name == ANGLE_ARITHMETIC {
            continue;
        }
        for (number, line) in shipped.lines().enumerate() {
            let code = without_strings_or_comments(line);
            if ANGLE_CONVERSIONS
                .iter()
                .any(|conversion| code.contains(conversion))
            {
                offences.push(format!("{name}:{}: {}", number + 1, line.trim()));
            }
        }
    }

    assert!(
        offences.is_empty(),
        "an angle is converted outside {BOUNDARY}, at {offences:#?}. Every angle \
         on this side of the boundary is already in radians."
    );
}

/// The readers are only worth anything if they are looking at something. Any one
/// of these left unasserted would leave a check above passing on an empty list,
/// which reads exactly like a clean tree.
#[test]
fn the_readers_found_what_they_are_reading() {
    let units = accepted_length_units();
    assert!(
        units.iter().any(|(name, _)| name == "mm"),
        "no millimetre among the units read out of {BOUNDARY}: {units:?}"
    );
    assert_eq!(
        conversion_factors().len() + 1,
        units.len(),
        "the internal unit is not the one accepted unit with no factor: {units:?}"
    );

    let sources = shipped_sources();
    let names: Vec<&String> = sources.iter().map(|(name, _)| name).collect();
    for owed in [BOUNDARY, ANGLE_ARITHMETIC] {
        assert!(
            names.iter().any(|name| *name == owed),
            "{owed} is not among the sources this file reads: {names:?}"
        );
    }
    assert!(
        sources.len() > 2,
        "only the two exempt files were read, so both checks passed on nothing"
    );

    let boundary = sources
        .iter()
        .find(|(name, _)| name == BOUNDARY)
        .map(|(_, shipped)| shipped)
        .expect("the boundary was read");
    assert!(
        boundary.contains("LENGTH_UNITS"),
        "the shipped part of {BOUNDARY} does not carry the units, so the \
         truncation at the test module has cut off code that ships"
    );
}

/// The mistakes the readers exist for, written as somebody would write them, so
/// that the matchers are known to recognise the shape rather than only the
/// shapes that happen not to be in the tree.
#[test]
fn the_matchers_recognise_the_mistakes_they_exist_for() {
    let factors = conversion_factors();

    for line in [
        "        let metres = millimetres / 1000.0;",
        "    let across = inches * 0.0254;",
        "        Measured::deviation(value / 1000.0, sd / 1000.0)",
        "    let centimetres = metres * 100.0;",
        "        let feet = metres / 0.3048;",
    ] {
        assert!(
            numbers_in(&without_strings_or_comments(line))
                .into_iter()
                .any(|value| matching_factor(&factors, value).is_some()),
            "the reader does not recognise {line:?}"
        );
    }

    for line in [
        "    const SAME_LENGTH: f64 = 1e-12;",
        "        let ratio = minor / major;",
        "    let scaled = base.mul_add(factor, offset);",
        "    // one millimetre is 0.001 metres and the factor lives at the boundary",
        "        detail: \"1000 samples were drawn\".to_owned(),",
        "    let widened = extent * 2.0;",
        "        '\"' => inside = !inside,",
    ] {
        assert!(
            numbers_in(&without_strings_or_comments(line))
                .into_iter()
                .all(|value| matching_factor(&factors, value).is_none()),
            "the reader refuses {line:?}, which it should not"
        );
    }

    for line in [
        "        let radians = math::to_radians(written);",
        "    rendered.push(to_degrees(azimuth));",
    ] {
        let code = without_strings_or_comments(line);
        assert!(
            ANGLE_CONVERSIONS.iter().any(|name| code.contains(name)),
            "the reader does not recognise {line:?}"
        );
    }
}

/// One place a unit is written: the line it is on counting from one, the unit
/// written there, and the byte range of the whole `unit = "..."` pair.
struct Site {
    line: usize,
    unit: String,
    at: usize,
    to: usize,
}

/// Every place the file writes a unit, in the order they appear.
fn unit_sites(text: &str) -> Vec<Site> {
    let marker = "unit = \"";
    let mut found = Vec::new();
    let mut from = 0;

    while let Some(offset) = text[from..].find(marker) {
        let at = from + offset;
        let opening = at + marker.len();
        let closing = opening
            + text[opening..]
                .find('"')
                .expect("a unit is written as a closed string");
        found.push(Site {
            line: text[..at].lines().count(),
            unit: text[opening..closing].to_owned(),
            at,
            to: closing + 1,
        });
        from = closing + 1;
    }
    found
}

/// The one site on a given line.
fn site_on(text: &str, line: usize) -> Site {
    unit_sites(text)
        .into_iter()
        .find(|site| site.line == line)
        .unwrap_or_else(|| panic!("no unit is written on line {line}"))
}

/// The lines of the file that write a unit, derived from the file rather than
/// from the reader above, so that the two can be compared.
fn lines_carrying_a_unit(text: &str) -> Vec<usize> {
    text.lines()
        .enumerate()
        .filter(|(_, line)| line.contains("unit = \""))
        .map(|(at, _)| at + 1)
        .collect()
}

/// The one line two files differ on.
///
/// Compared line by line rather than byte by byte, because the line ending is a
/// property of the checkout rather than of the fixture.
fn differing_line(valid: &str, other: &str) -> usize {
    let differing: Vec<usize> = valid
        .lines()
        .zip(other.lines())
        .enumerate()
        .filter(|(_, (one, two))| one != two)
        .map(|(at, _)| at + 1)
        .collect();
    assert_eq!(
        differing.len(),
        1,
        "the fixture is not one line from {VALID}, it differs on {differing:?}"
    );
    differing[0]
}

/// The file with one unit taken away, and the comma that held it in place.
///
/// The comma goes with it, on whichever side it sits, so that what is left is a
/// table this format could hold and the refusal is about the missing unit rather
/// than about the punctuation.
fn without_the_unit(text: &str, site: &Site) -> String {
    let before = &text[..site.at];
    let after = &text[site.to..];
    if let Some(rest) = after.strip_prefix(", ") {
        format!("{before}{rest}")
    } else {
        let before = before
            .strip_suffix(", ")
            .expect("a unit is written after a comma or before one");
        format!("{before}{after}")
    }
}

/// The file with one unit replaced by another.
fn with_the_unit(text: &str, site: &Site, unit: &str) -> String {
    format!("{}unit = \"{unit}\"{}", &text[..site.at], &text[site.to..])
}

/// The refusals of a file that has to be refused.
fn refusal_of(file: &str, what: &str) -> Refusals {
    match input::read(file, &table()) {
        Ok(_) => panic!("{what} was accepted"),
        Err(refusals) => refusals,
    }
}

/// The input of a file that has to parse.
fn parsed(file: &str, what: &str) -> Input {
    input::read(file, &table())
        .unwrap_or_else(|why| panic!("{}", why.messages_from(what).join("\n")))
}

/// The major axis of the first hole, in the internal unit.
fn major_axis_of(file: &str) -> f64 {
    parsed(file, "a file under test").scene().holes()[0]
        .perforation()
        .major_axis()
        .value()
        .expect("the major axis has a value")
}

/// The one refusal of a kind, refusing a set that holds two of them.
///
/// A sweep that changed one line and accepted any number of refusals of the kind
/// it was looking for would pass against a parser refusing every value in the
/// file.
fn only_matching(refusals: &Refusals, wanted: fn(&Fault) -> bool) -> &Refusal {
    let matching: Vec<&Refusal> = refusals
        .list()
        .iter()
        .filter(|refusal| wanted(&refusal.fault))
        .collect();
    assert_eq!(
        matching.len(),
        1,
        "one line was changed and the refusals are {}",
        listed(refusals)
    );
    matching[0]
}

fn listed(refusals: &Refusals) -> String {
    refusals
        .list()
        .iter()
        .map(Refusal::message)
        .collect::<Vec<String>>()
        .join("\n  ")
}

/// The length units the boundary accepts, read out of the boundary itself.
///
/// Written here they would be a copy that drifts the day a unit is added, and a
/// check comparing against a stale copy passes for the wrong reason.
fn accepted_length_units() -> Vec<(String, f64)> {
    let source = text(&at(BOUNDARY));
    let opening = source
        .find("const LENGTH_UNITS")
        .expect("the boundary declares the units it accepts");
    let closing = opening
        + source[opening..]
            .find("];")
            .expect("the unit table is closed where it is declared");

    let mut found = Vec::new();
    for line in source[opening..closing].lines() {
        let Some(rest) = line.trim().strip_prefix('(') else {
            continue;
        };
        let Some(rest) = rest.strip_suffix("),") else {
            continue;
        };
        let (name, factor) = rest.split_once(',').expect("a unit and its factor");
        found.push((
            name.trim().trim_matches('"').to_owned(),
            factor.trim().parse::<f64>().expect("a factor is a number"),
        ));
    }
    assert!(!found.is_empty(), "no units were read out of {BOUNDARY}");
    found
}

/// The factors that convert, which is every accepted unit but the internal one.
fn conversion_factors() -> Vec<f64> {
    accepted_length_units()
        .into_iter()
        .map(|(_, factor)| factor)
        .filter(|factor| factor.to_bits() != 1.0f64.to_bits())
        .collect()
}

/// The factor a number written in the source converts by, in either direction.
///
/// Compared by bit pattern rather than by value, which is the comparison a
/// tolerance of zero would make and is not read as a floating-point equality
/// test by anything that would object to one.
fn matching_factor(factors: &[f64], value: f64) -> Option<f64> {
    factors.iter().copied().find(|factor| {
        value.to_bits() == factor.to_bits() || (value * factor - 1.0).abs() <= RECIPROCAL_TOLERANCE
    })
}

/// Every numeric literal on a line, as the value it parses to.
///
/// A run of digits following a letter, an underscore or a full stop is part of
/// something else: the `64` of `f64`, the second half of a range, a field of a
/// version.
fn numbers_in(line: &str) -> Vec<f64> {
    let characters: Vec<char> = line.chars().collect();
    let mut found = Vec::new();
    let mut at = 0;

    while at < characters.len() {
        let joined_to_something = at > 0
            && (characters[at - 1].is_ascii_alphanumeric()
                || characters[at - 1] == '_'
                || characters[at - 1] == '.');
        if !characters[at].is_ascii_digit() || joined_to_something {
            at += 1;
            continue;
        }

        let from = at;
        at = past_digits(&characters, at);
        if characters.get(at) == Some(&'.')
            && characters.get(at + 1).is_some_and(char::is_ascii_digit)
        {
            at = past_digits(&characters, at + 1);
        }
        if matches!(characters.get(at), Some('e' | 'E')) {
            let mut ahead = at + 1;
            if matches!(characters.get(ahead), Some('+' | '-')) {
                ahead += 1;
            }
            if characters.get(ahead).is_some_and(char::is_ascii_digit) {
                at = past_digits(&characters, ahead);
            }
        }

        let written: String = characters[from..at]
            .iter()
            .filter(|character| **character != '_')
            .collect();
        if let Ok(value) = written.parse::<f64>() {
            found.push(value);
        }
    }
    found
}

fn past_digits(characters: &[char], mut at: usize) -> usize {
    while at < characters.len() && (characters[at].is_ascii_digit() || characters[at] == '_') {
        at += 1;
    }
    at
}

/// A line with its string literals, character literals and line comment taken
/// out, so that a number somebody wrote about is not read as a number the code
/// multiplies by.
///
/// A character literal is skipped whole rather than toggled on, because
/// `crates/einschlag/src/materials.rs` holds a quotation mark inside one, and a
/// reader that toggled there would read the rest of that line as text and stop
/// looking at it.
fn without_strings_or_comments(line: &str) -> String {
    let characters: Vec<char> = line.chars().collect();
    let mut kept = String::new();
    let mut inside_a_string = false;
    let mut at = 0;

    while at < characters.len() {
        let character = characters[at];
        if inside_a_string {
            at += if character == '\\' { 2 } else { 1 };
            if character == '"' {
                inside_a_string = false;
            }
            continue;
        }
        if character == '\'' {
            if characters.get(at + 1) == Some(&'\\') && characters.get(at + 3) == Some(&'\'') {
                at += 4;
                continue;
            }
            if characters.get(at + 2) == Some(&'\'') {
                at += 3;
                continue;
            }
        }
        if character == '"' {
            inside_a_string = true;
            at += 1;
            continue;
        }
        if character == '/' && characters.get(at + 1) == Some(&'/') {
            break;
        }
        kept.push(character);
        at += 1;
    }
    kept
}

/// Every Rust source that ships, as its path from the workspace root and the
/// part of it above its test module.
fn shipped_sources() -> Vec<(String, String)> {
    let root = workspace_root();
    let mut found = Vec::new();

    for crate_root in directories_in(&root.join("crates")) {
        let source = crate_root.join("src");
        if !source.is_dir() {
            continue;
        }
        let mut pending = vec![source];
        while let Some(directory) = pending.pop() {
            for entry in fs::read_dir(&directory)
                .unwrap_or_else(|why| panic!("cannot read {}: {why}", shown(&directory)))
            {
                let path = entry.expect("the directory entry is readable").path();
                if path.is_dir() {
                    pending.push(path);
                } else if path.extension().is_some_and(|suffix| suffix == "rs") {
                    let name = relative(&root, &path);
                    let shipped = above_the_test_module(&name, &text(&path));
                    found.push((name, shipped));
                }
            }
        }
    }
    found.sort();
    found
}

/// The part of a source above its `#[cfg(test)]` module.
///
/// Every file in this workspace puts that module last and puts nothing after it,
/// which is what makes the truncation safe. The two assertions are what keep it
/// safe: a second marker, or an item that ships written below the first one,
/// would mean this reader is quietly looking at less of the file than it says.
fn above_the_test_module(name: &str, source: &str) -> String {
    let lines: Vec<&str> = source.lines().collect();
    let markers: Vec<usize> = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.trim() == "#[cfg(test)]")
        .map(|(at, _)| at)
        .collect();
    assert!(
        markers.len() <= 1,
        "{name} carries {} test modules, and this reader assumes the one at the \
         end of the file. Reading it as it stands would skip code that ships.",
        markers.len()
    );
    let Some(&first) = markers.first() else {
        return source.to_owned();
    };

    let below: Vec<&&str> = lines[first + 1..]
        .iter()
        .filter(|line| {
            ["pub ", "fn ", "impl ", "const ", "static "]
                .iter()
                .any(|item| line.starts_with(item))
        })
        .collect();
    assert!(
        below.is_empty(),
        "{name} writes something that ships below its test module, at {below:?}, \
         so truncating there would hide it from every check in this file"
    );

    lines[..first].join("\n")
}

fn directories_in(under: &Path) -> Vec<PathBuf> {
    let mut found: Vec<PathBuf> = fs::read_dir(under)
        .unwrap_or_else(|why| panic!("cannot read {}: {why}", shown(under)))
        .map(|entry| entry.expect("the directory entry is readable").path())
        .filter(|path| path.is_dir())
        .collect();
    found.sort();
    found
}

fn table() -> MaterialTable {
    MaterialTable::parse(&text(&at(TABLE))).expect("the fixture table parses")
}

fn at(relative: &str) -> PathBuf {
    workspace_root().join(relative)
}

fn relative(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .expect("every walked path is under the workspace root")
        .to_string_lossy()
        .replace('\\', "/")
}

fn shown(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

fn text(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|why| panic!("cannot read {}: {why}", shown(path)))
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
        shown(&root)
    );
    root
}

/// Two lengths agreeing to within the tolerance this file states.
fn same_length(found: f64, wanted: f64, what: &str) {
    assert!(
        (found - wanted).abs() <= SAME_LENGTH * wanted.abs().max(1.0),
        "{what} came back as {found} and not {wanted}"
    );
}
