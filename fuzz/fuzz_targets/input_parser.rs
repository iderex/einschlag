//! The input parser, driven with bytes nobody wrote.
//!
//! Issue #58 says what the interesting outcome is here, and it is not a crash:
//! it is a file that parses into a scene the operator did not write. So this
//! target does not stop at "it did not panic". Where the reader returns an
//! `Input`, every property the refusals claim to guarantee is asserted against
//! the value that came back, and an assertion failing is a finding of exactly
//! the shape the issue asks for.
//!
//! `docs/TESTING.md` states the time bound the continuous integration job runs
//! this under, and what a run of that length does and does not cover.

#![no_main]

use std::collections::BTreeSet;
use std::sync::OnceLock;

use einschlag::input::{self, FORMAT_VERSION};
use einschlag::materials::MaterialTable;
use einschlag::measurement::{Measured, Spread};
use libfuzzer_sys::fuzz_target;

/// The material table the target reads against.
///
/// The fixture rather than the tracked table, and for the reason
/// `fixtures/scene/a-material-table-with-one-row.toml` gives: a run judged
/// against the real table would prove the state of that table on the day it ran
/// rather than anything about the parser. It is also the table every fixture in
/// the corpus was written against, so a corpus entry that parsed when it was
/// written still parses here.
const TABLE: &str = include_str!("../../fixtures/scene/a-material-table-with-one-row.toml");

fn table() -> &'static MaterialTable {
    static TABLE_ONCE: OnceLock<MaterialTable> = OnceLock::new();
    TABLE_ONCE
        .get_or_init(|| MaterialTable::parse(TABLE).expect("the fixture material table parses"))
}

fuzz_target!(|data: &[u8]| {
    // The reader takes text. Bytes that are not UTF-8 never reach it through
    // any route a person has, because the front end reads a file as text and
    // refuses one that is not, so feeding it a lossy conversion here would
    // fuzz a decoder this project does not ship.
    let Ok(text) = std::str::from_utf8(data) else {
        return;
    };

    match input::read(text, table()) {
        Err(refusals) => {
            // The reader's own documentation says a refusal list is never
            // empty. An empty one would be a refusal a caller cannot report,
            // and the caller would have nothing to print.
            assert!(
                !refusals.list().is_empty(),
                "the reader refused the input and gave no reason"
            );
            // Every refusal has to be able to say what it is about. A message
            // built from a line number the source does not have is the shape a
            // reader would meet as a panic in the front end rather than here.
            let _ = refusals.messages_from(text);
        }
        Ok(parsed) => accepted_input_is_one_an_operator_could_have_written(&parsed),
    }
});

/// Every property the refusals claim to guarantee, asserted against a value the
/// reader accepted.
///
/// A failure here is the outcome issue #58 names: not a crash, but a file that
/// parsed into a scene nobody wrote.
fn accepted_input_is_one_an_operator_could_have_written(parsed: &input::Input) {
    assert_eq!(
        parsed.format_version(),
        FORMAT_VERSION,
        "a file was accepted at a format version this build does not read"
    );

    let scene = parsed.scene();

    let mut surface_ids = BTreeSet::new();
    for surface in scene.surfaces() {
        assert!(
            surface_ids.insert(surface.id()),
            "two surfaces were accepted under the identifier {:?}, so a hole \
             naming it points at either",
            surface.id()
        );
        assert!(
            table()
                .rows()
                .iter()
                .any(|row| row.name() == surface.material()),
            "the surface {:?} was accepted with the material {:?}, which the \
             table has no row for",
            surface.id(),
            surface.material()
        );
        measured_is_readable(surface.normal_azimuth(), "a surface normal azimuth");
        measured_is_readable(surface.normal_elevation(), "a surface normal elevation");
    }

    let mut hole_ids = BTreeSet::new();
    for hole in scene.holes() {
        assert!(
            hole_ids.insert(hole.id()),
            "two holes were accepted under the identifier {:?}, and the \
             trajectory keys name holes by it",
            hole.id()
        );
        assert!(
            scene.surface_of(hole).is_some(),
            "the hole {:?} was accepted in the surface {:?}, which the scene \
             does not contain",
            hole.id(),
            hole.surface()
        );

        let centre = hole.centre();
        measured_is_readable(centre.x, "a hole centre x");
        measured_is_readable(centre.y, "a hole centre y");
        measured_is_readable(centre.z, "a hole centre z");

        let perforation = hole.perforation();
        let major = perforation.major_axis();
        let minor = perforation.minor_axis();
        measured_is_readable(major, "a perforation major axis");
        measured_is_readable(minor, "a perforation minor axis");
        measured_is_readable(perforation.bearing(), "a perforation bearing");

        // The axis ratio is an arcsine argument. A minor axis longer than its
        // major one is outside the domain, and the reader refuses it by name,
        // so an accepted pair that is the wrong way round is the arithmetic
        // failure arriving through the door marked "valid file".
        if let (Some(major), Some(minor)) = (major.value(), minor.value()) {
            assert!(
                minor <= major,
                "a perforation was accepted with a minor axis of {minor} against \
                 a major axis of {major}, which no ellipse has"
            );
            assert!(
                major > 0.0,
                "a perforation was accepted with a major axis of {major}"
            );
        }
    }

    // A trajectory names holes, and a key naming a hole the scene does not hold
    // is a constraint that cannot be built later.
    for (hole, trajectory) in parsed.trajectories() {
        assert!(
            hole_ids.contains(hole.as_str()),
            "the trajectory {trajectory:?} names the hole {hole:?}, which the \
             scene does not contain"
        );
    }
}

/// One measured quantity, checked for the two shapes that would reach the
/// arithmetic as something other than a number.
fn measured_is_readable(measured: Measured, what: &str) {
    match (measured.value(), measured.spread()) {
        (None, Spread::Unestablished) => {}
        (Some(value), Spread::StandardDeviation(sd)) => {
            assert!(value.is_finite(), "{what} was accepted as {value}");
            assert!(
                sd.is_finite() && sd >= 0.0,
                "{what} was accepted with a standard deviation of {sd}"
            );
        }
        (Some(value), Spread::Interval { low, high }) => {
            assert!(value.is_finite(), "{what} was accepted as {value}");
            assert!(
                low.is_finite() && high.is_finite() && low <= high,
                "{what} was accepted with the interval [{low}, {high}]"
            );
        }
        (value, spread) => panic!(
            "{what} was accepted as {value:?} with the spread {spread:?}, and \
             a value is present exactly when a spread was established"
        ),
    }
}
