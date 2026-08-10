//! The parser: an operator's file into the types a reconstruction is built from.
//!
//! This is the boundary. Everything on the far side of it is text somebody typed
//! at a scene or transcribed from a measurement sheet, and everything on this
//! side is a value in the internal units
//! `docs/decisions/0006-frame-and-units.md` fixes. The parser is where a wrong
//! reconstruction is cheapest to prevent and most expensive to miss: a field
//! read into the wrong slot produces a result that looks entirely ordinary.
//!
//! `docs/decisions/0007-input-format.md` fixes the format and
//! `docs/decisions/0015-unknown-keys-in-an-input-file.md` answers the one
//! question 0007 left to a later record, which is what happens to a key this
//! build does not know. It is refused.
//!
//! # What this module refuses, and what it does not
//!
//! It refuses a file whose shape is wrong, a value stated without its
//! uncertainty, a unit outside the closed set, a quantity that is not a number,
//! and every refusal the types it builds carry: a minor axis longer than a major
//! one, a negative length, a repeated identifier, a hole naming a surface the
//! scene does not contain. Each refusal carries the line it was written on, and
//! [`Refusal::message_from`] puts the file in front of it.
//!
//! **It refuses everything it can find rather than the first thing.** An
//! operator fixing a file one refusal per run stops reading the messages, and a
//! file with eight mistakes in it is the ordinary case for a format somebody is
//! writing for the first time. The one exception is the format version: a file
//! at a version this build does not read is refused and nothing else in it is
//! looked at, which is 0007's rule rather than a choice made here.
//!
//! **It converts once, here.** A length arrives in `m`, `cm`, `mm`, `in` or
//! `ft` and leaves in metres; an angle arrives in `deg` and leaves in radians.
//! Nothing downstream converts anything, and nothing downstream can tell what
//! unit a number arrived in, which is why the conversion is in one file with the
//! factors written next to the record that sourced them.
//!
//! **It does not check geometry.** No hole is checked for lying on the surface
//! it names, no outline for being planar, no obstacle for closing. Those need
//! milestone 5.
//!
//! **It does not read a figure out of the material table.** What it does is
//! refuse a hole in a material the table has no row for, which is the check the
//! table's own emptiness makes into the ordinary outcome today: with the tracked
//! table carrying no rows at all, every file naming any material is refused
//! here. That is the state of the project rather than a property of this code,
//! `docs/materials.md` argues it, and issue #76 is where it changes.

mod document;

use crate::materials::MaterialTable;
use crate::math;
use crate::measurement::{self, Centre, Deformation, Hole, Measured, Perforation};
use crate::scene::{self, Contour, Extent, Ground, Obstacle, Point, Scene, Span, Surface};

use document::{Entry, Node, Table};

/// The format version this build reads.
///
/// One version rather than a set, because there has only ever been one. A file
/// at any other version is refused whole:
/// `docs/decisions/0007-input-format.md` refuses a partial read in as many
/// words, because a field whose meaning changed between versions is read into
/// the right slot with the wrong meaning and nothing downstream can tell.
pub const FORMAT_VERSION: i64 = 1;

/// The length units accepted at the boundary, and what one of each is in metres.
///
/// The set and the factors are `docs/decisions/0006-frame-and-units.md`'s, which
/// carries the source of each. They are here rather than in the numeric core
/// because this is the boundary the record puts them at, and
/// `crates/einschlag/tests/units_carried_explicitly.rs` reads this table out of
/// this file and refuses the same numbers everywhere else that ships.
const LENGTH_UNITS: [(&str, f64); 5] = [
    ("m", 1.0),
    ("cm", 0.01),
    ("mm", 0.001),
    ("in", 0.0254),
    ("ft", 0.3048),
];

/// The one angle unit accepted at the boundary.
///
/// Radians are not accepted, which is the record's decision and its reason: an
/// operator who wrote a scene angle in radians has almost certainly made a
/// mistake, and one who has not loses nothing by writing degrees.
const ANGLE_UNIT: &str = "deg";

/// What the keys of a `[scene]` table are.
const SCENE_KEYS: [&str; 5] = ["name", "origin", "reference_direction", "extent", "ground"];

/// What the keys of a `[[scene.surface]]` table are.
const SURFACE_KEYS: [&str; 6] = [
    "id",
    "material",
    "contour",
    "outline",
    "normal_azimuth",
    "normal_elevation",
];

/// What the keys of a `[[scene.obstacle]]` table are.
const OBSTACLE_KEYS: [&str; 2] = ["id", "faces"];

/// What the keys of a `[[hole]]` table are.
const HOLE_KEYS: [&str; 9] = [
    "id",
    "surface",
    "material",
    "trajectory",
    "centre",
    "major_axis",
    "minor_axis",
    "major_axis_bearing",
    "deformation",
];

/// The three uncertainty forms, in the order a message lists them.
const UNCERTAINTY_FORMS: [&str; 3] = ["sd", "interval", "unknown"];

/// What a measured value's table may carry.
const MEASURED_KEYS: [&str; 5] = ["value", "unit", "sd", "interval", "unknown"];

/// Whether a quantity is a length or an angle, which decides what units it may
/// be written in and what it is converted to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Quantity {
    /// Converted to metres.
    Length,
    /// Converted to radians.
    Angle,
}

impl Quantity {
    /// The units this quantity may be written in, for a message that has to say
    /// what was expected.
    fn accepted(self) -> String {
        match self {
            Self::Length => {
                let names: Vec<&str> = LENGTH_UNITS.iter().map(|(name, _)| *name).collect();
                names.join(", ")
            }
            Self::Angle => ANGLE_UNIT.to_owned(),
        }
    }

    /// One number in the unit named, as the internal unit.
    ///
    /// The angle conversion goes through `crate::math` rather than the
    /// platform's, which `docs/decisions/0013-platform-math-out-of-the-numeric-core.md`
    /// requires and `crates/einschlag/tests/platform_math_stays_out_of_the_core.rs`
    /// refuses the other route of.
    fn convert(self, unit: &str, number: f64) -> Option<f64> {
        match self {
            Self::Length => LENGTH_UNITS
                .iter()
                .find(|(name, _)| *name == unit)
                .map(|(_, metres)| number * metres),
            Self::Angle => (unit == ANGLE_UNIT).then(|| math::to_radians(number)),
        }
    }
}

/// Why a file, or something in it, was refused.
///
/// One variant per reason rather than a string, so that a caller can act on the
/// reason and so that everything this module refuses is readable in one place.
/// It is the same shape `crate::materials` uses for the same reason.
#[derive(Debug, Clone, PartialEq)]
pub enum Fault {
    /// The text is not TOML, or it is TOML in a shape this format does not
    /// have: a header nobody declared, a dotted key, a date where no key is one.
    Shape {
        /// What was wrong, in the words somebody meeting it needs.
        detail: String,
    },
    /// No `format_version` at the top level. It is required, and a file without
    /// it is not read as version 1: a file predating the key and a file whose
    /// author forgot it are indistinguishable.
    FormatVersionAbsent,
    /// A `format_version` this build does not read. The message names what was
    /// found, what this build reads and what the tool is called, so that
    /// somebody meeting it knows whether they need a newer tool or an older
    /// file.
    FormatVersionNotRead {
        /// What the file stated.
        found: String,
    },
    /// No `[scene]` table. Every file describes one scene.
    SceneAbsent,
    /// A key that is required and is not there.
    KeyAbsent {
        /// Where it was wanted, as the operator would name it.
        table: String,
        /// The key.
        key: &'static str,
    },
    /// A key this build does not know.
    ///
    /// Refused rather than ignored, which is
    /// `docs/decisions/0015-unknown-keys-in-an-input-file.md`. A mistyped key
    /// name that is ignored leaves the value it was meant to carry unstated,
    /// and the run then proceeds on a default or a refusal somewhere the
    /// operator cannot connect to what they typed.
    KeyNotKnown {
        /// Where it was written, as the operator would name it.
        table: String,
        /// The key.
        key: String,
    },
    /// A key written twice in one table. Which of the two applied would be
    /// decided by whichever the reader reached first.
    KeyRepeated {
        /// Where it was written.
        table: String,
        /// The key.
        key: String,
    },
    /// A value written in a form the key does not take.
    ValueNotOfKind {
        /// The key, with the table in front of it.
        key: String,
        /// What the key takes.
        wanted: &'static str,
        /// What was written.
        found: &'static str,
    },
    /// An array of the wrong length: a coordinate that is not three numbers, an
    /// interval that is not two.
    ArrayNotOfLength {
        /// The key, with the table in front of it.
        key: String,
        /// How many are wanted.
        wanted: usize,
        /// How many were written.
        found: usize,
    },
    /// A number that is not finite. An infinity or a nothing arriving here would
    /// carry through every draw made from it and reach the reported region as a
    /// value nothing downstream can reduce.
    NotFinite {
        /// The key, with the table in front of it.
        key: String,
    },
    /// A measured value with no `unit`.
    UnitAbsent {
        /// The key, with the table in front of it.
        key: String,
    },
    /// A unit outside the closed set for this quantity, including an angle unit
    /// on a length and a length unit on an angle.
    UnitNotKnown {
        /// The key, with the table in front of it.
        key: String,
        /// What was written.
        found: String,
        /// What this quantity may be written in.
        accepted: String,
    },
    /// A measured value stating none of the three uncertainty forms.
    ///
    /// This is the refusal `docs/decisions/0007-input-format.md` argues at
    /// length. It is not a dead end: `unknown = true` is always available, and
    /// what is refused is silence rather than ignorance.
    UncertaintyAbsent {
        /// The key, with the table in front of it.
        key: String,
    },
    /// A measured value stating more than one of the three forms. They are three
    /// different statements about one quantity and two of them cannot both hold.
    UncertaintyStatedTwice {
        /// The key, with the table in front of it.
        key: String,
        /// The first form found.
        first: &'static str,
        /// The second.
        second: &'static str,
    },
    /// `unknown = false`, which states nothing.
    UnknownStatedAsFalse {
        /// The key, with the table in front of it.
        key: String,
    },
    /// A value stated beside `unknown = true`.
    ///
    /// `docs/decisions/0007-input-format.md` refuses this by name and requires
    /// the message to name the key and both fields: a number and a declaration
    /// that nothing was established cannot both be asserted about one quantity.
    ValueBesideUnknown {
        /// The key, with the table in front of it.
        key: String,
    },
    /// A word outside the closed set a key takes.
    WordNotKnown {
        /// The key, with the table in front of it.
        key: String,
        /// What was written.
        found: String,
        /// What the key takes.
        accepted: &'static [&'static str],
    },
    /// A hole or a surface in a material the table has no row for. This project
    /// has read no error figure for it, and there is no default behind the
    /// refusal because a default here would be an invented uncertainty handed
    /// to a reader as a measured one.
    MaterialHasNoRow {
        /// The material as it was written.
        material: String,
        /// The hole or surface that named it.
        named_by: String,
    },
    /// A refusal from the hole record, carried out with what it was about.
    Record {
        /// The hole or surface the refusal is about.
        about: String,
        /// What the record refused.
        refusal: measurement::Refusal,
    },
    /// A refusal from the scene, carried out with the line of the element it is
    /// about.
    Scene {
        /// What the scene refused.
        refusal: scene::Refusal,
    },
}

impl Fault {
    /// What was wrong, in the words somebody meeting it needs.
    #[must_use]
    pub fn message(&self) -> String {
        match self {
            Self::Shape { detail } => detail.clone(),
            Self::FormatVersionAbsent => format!(
                "no format_version at the top level of the file. It is required, and a file \
                 without it is not read as version {FORMAT_VERSION}, because a file predating \
                 the key and a file whose author forgot it look the same."
            ),
            Self::FormatVersionNotRead { found } => format!(
                "format_version {found}, and {} {} reads {FORMAT_VERSION}. Nothing is read out \
                 of a file at a version this build does not know, so this is either a newer \
                 file than the tool or an older tool than the file.",
                crate::TOOL_NAME,
                crate::VERSION
            ),
            Self::SceneAbsent => {
                "no [scene] table. Every file describes one scene, and its extent and ground \
                 are stated there."
                    .to_owned()
            }
            Self::KeyAbsent { table, key } => {
                format!("{table} states no {key}, and it is required")
            }
            Self::KeyNotKnown { table, key } => format!(
                "{table} carries the key {key}, which this build does not know. A key that is \
                 ignored is a value the operator stated and the tool did not read."
            ),
            Self::KeyRepeated { table, key } => format!(
                "{table} states {key} twice. Which one applied would be decided by whichever \
                 was read first, which is a choice nobody made."
            ),
            Self::ValueNotOfKind { key, wanted, found } => {
                format!("{key} takes {wanted} and {found} was written")
            }
            Self::ArrayNotOfLength { key, wanted, found } => {
                format!("{key} takes {wanted} numbers and {found} were written")
            }
            Self::NotFinite { key } => format!(
                "{key} is not a finite number. An infinity or a nothing here would carry \
                 through every draw made from it."
            ),
            Self::UnitAbsent { key } => format!(
                "{key} states no unit. Every measured value carries its own, so that a file \
                 may mix them and a reader can check one line without reading a header."
            ),
            Self::UnitNotKnown {
                key,
                found,
                accepted,
            } => format!("{key} is in {found}, and the units accepted here are {accepted}"),
            Self::UncertaintyAbsent { key } => format!(
                "{key} states a value and nothing about how well it is known. One of {} is \
                 required. Where nothing was established, unknown = true says so and is \
                 accepted; what is refused is silence rather than ignorance.",
                UNCERTAINTY_FORMS.join(", ")
            ),
            Self::UncertaintyStatedTwice { key, first, second } => format!(
                "{key} states both {first} and {second}. They are different statements about \
                 one quantity and exactly one of them belongs here."
            ),
            Self::UnknownStatedAsFalse { key } => format!(
                "{key} states unknown = false, which says nothing. State sd or interval, or \
                 write unknown = true, which is a statement that nothing was established."
            ),
            Self::ValueBesideUnknown { key } => format!(
                "{key} states both value and unknown = true. A number and a declaration that \
                 no uncertainty was established cannot both be asserted about one quantity."
            ),
            Self::WordNotKnown {
                key,
                found,
                accepted,
            } => format!(
                "{key} is {found}, and the words it takes are {}",
                accepted.join(", ")
            ),
            Self::MaterialHasNoRow { material, named_by } => format!(
                "{named_by} is in {material}, which has no row in the material table, so this \
                 project has read no error for it. docs/materials.md says what would add a row."
            ),
            Self::Record { about, refusal } => {
                format!("{about}: {}", record_message(refusal))
            }
            Self::Scene { refusal } => scene_message(refusal),
        }
    }
}

/// What a hole record refused, in the words somebody meeting it needs.
///
/// Matched exhaustively rather than with a catch-all, so that a refusal added to
/// the record reddens this build instead of arriving at an operator as a word
/// nobody chose.
fn record_message(refusal: &measurement::Refusal) -> String {
    match refusal {
        measurement::Refusal::NotFinite => "a value here is not a finite number".to_owned(),
        measurement::Refusal::NegativeStandardDeviation => {
            "a standard deviation here is below zero".to_owned()
        }
        measurement::Refusal::IntervalInverted => {
            "an interval here has its bounds the wrong way round".to_owned()
        }
        measurement::Refusal::ValueOutsideItsInterval => {
            "a value here lies outside the interval stated for it, which is a data entry \
             mistake rather than a wide measurement"
                .to_owned()
        }
        measurement::Refusal::NegativeLength => {
            "a length here is below zero, as stated or as the lower bound of its interval. A \
             perforation does not have a negative axis."
                .to_owned()
        }
        measurement::Refusal::AxisNotMeasured => {
            "an axis here states that nothing was established. The impact angle comes out of \
             the ratio of the two axes and there is nothing here to take a ratio of, so \
             unknown = true is not available on an axis."
                .to_owned()
        }
        measurement::Refusal::MinorAxisExceedsMajor => {
            "the minor axis is longer than the major axis. The impact angle comes out of the \
             ratio of the two, so a swapped pair gives either no answer at all or a plausible \
             wrong angle, and this is the only place a machine can catch the second."
                .to_owned()
        }
        measurement::Refusal::CentreNotMeasured => {
            "the centre states that nothing was established on one of its coordinates. A \
             position that was not established is not a position."
                .to_owned()
        }
        measurement::Refusal::EmptyIdentifier(which) => {
            format!("the {which} is empty or is only whitespace")
        }
    }
}

/// What the scene refused, in the words somebody meeting it needs.
fn scene_message(refusal: &scene::Refusal) -> String {
    match refusal {
        scene::Refusal::NotFinite => {
            "a coordinate or a bound here is not a finite number".to_owned()
        }
        scene::Refusal::SpanInverted => {
            "an extent bound here has its lower value above its upper one".to_owned()
        }
        scene::Refusal::EmptyIdentifier(which) => {
            format!("the {which} is empty or is only whitespace")
        }
        scene::Refusal::DuplicateIdentifier(id) => format!(
            "the identifier {id} is used twice. A reference to it would resolve to whichever \
             came first, which is a choice nobody made."
        ),
        scene::Refusal::UnknownSurface { hole, surface } => format!(
            "the hole {hole} is in the surface {surface}, which this scene does not contain. A \
             reference that resolves to nothing would fail somewhere further along, in a place \
             that no longer knows which hole was wrong."
        ),
        scene::Refusal::OutlineNotAPolygon => {
            "an outline here has fewer than three vertices, and two points bound no area".to_owned()
        }
        scene::Refusal::SolidNotClosable => {
            "an obstacle here has fewer than four faces, and nothing closed can be built out of \
             three planar faces"
                .to_owned()
        }
        scene::Refusal::GroundNotMeasured => {
            "the ground states that nothing was established. The ground is where a person \
             stands; a scene that does not say where it is has not described the space."
                .to_owned()
        }
    }
}

/// A refusal, with the line it was refused at.
#[derive(Debug, Clone, PartialEq)]
pub struct Refusal {
    /// The line in the file, counting from one. `None` where the refusal is
    /// about the file as a whole rather than about a line in it.
    pub line: Option<usize>,
    /// What was wrong.
    pub fault: Fault,
}

impl Refusal {
    /// One sentence saying which line was refused, where there is one, and what
    /// was wrong.
    ///
    /// It names no file, because this type is produced from text and the text is
    /// not always a file on disc. [`Refusal::message_from`] is for a caller that
    /// knows where the text came from. A plain method rather than the standard
    /// formatting trait, for the reason `crate::materials::Refusal::message`
    /// gives.
    #[must_use]
    pub fn message(&self) -> String {
        match self.line {
            Some(line) => format!("line {line}: {}", self.fault.message()),
            None => self.fault.message(),
        }
    }

    /// The same sentence, with the file the text came from in front of it.
    #[must_use]
    pub fn message_from(&self, source: &str) -> String {
        match self.line {
            Some(line) => format!("{source}:{line}: {}", self.fault.message()),
            None => format!("{source}: {}", self.fault.message()),
        }
    }
}

/// Everything wrong with one file.
///
/// Never empty: a file that produced no refusal was read, and the reader returns
/// what it read instead.
#[derive(Debug, Clone, PartialEq)]
pub struct Refusals {
    refusals: Vec<Refusal>,
}

impl Refusals {
    /// Every refusal, in the order they were found, which is the order the lines
    /// are written in.
    #[must_use]
    pub fn list(&self) -> &[Refusal] {
        &self.refusals
    }

    /// One message per refusal, each naming the file it came from.
    #[must_use]
    pub fn messages_from(&self, source: &str) -> Vec<String> {
        self.refusals
            .iter()
            .map(|refusal| refusal.message_from(source))
            .collect()
    }
}

/// One file, read.
///
/// The scene is the thing a reconstruction is built from. The rest of it is what
/// the file said about itself, carried so that a run can record which file it
/// read and under what declarations.
#[derive(Debug, Clone, PartialEq)]
pub struct Input {
    format_version: i64,
    name: Option<String>,
    origin: String,
    reference_direction: String,
    scene: Scene,
    trajectories: Vec<(String, String)>,
}

impl Input {
    /// The version the file declared, which is always [`FORMAT_VERSION`].
    #[must_use]
    pub fn format_version(&self) -> i64 {
        self.format_version
    }

    /// What the operator called the scene, where they named it.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// How the operator described the origin of the frame, well enough that a
    /// person standing at the scene could find it again.
    ///
    /// `docs/decisions/0006-frame-and-units.md` requires this and it is the only
    /// part of the frame a reader on the other side can check against anything.
    #[must_use]
    pub fn origin(&self) -> &str {
        &self.origin
    }

    /// What the operator named as the reference direction, which is +Y.
    #[must_use]
    pub fn reference_direction(&self) -> &str {
        &self.reference_direction
    }

    /// The scene, in metres and radians.
    #[must_use]
    pub fn scene(&self) -> &Scene {
        &self.scene
    }

    /// Which trajectory each hole was declared to belong to, for the holes whose
    /// operator declared one.
    ///
    /// Nothing infers a trajectory, which
    /// `docs/decisions/0007-input-format.md` states, and nothing consumes this
    /// yet: issue #38 is where holes on one trajectory are combined.
    #[must_use]
    pub fn trajectories(&self) -> &[(String, String)] {
        &self.trajectories
    }
}

/// Read an input file, or say everything that is wrong with it.
///
/// `materials` is the table a hole's material is resolved against. It is a
/// parameter rather than the tracked table read from inside, so that a guard can
/// be shown biting against a fixture: a check that judged against the real table
/// would prove the state of the tree on the day it ran and not the guard.
///
/// # Errors
///
/// [`Refusals`], never empty, carrying every refusal the reader could find. The
/// exception is a file whose `format_version` is absent or is not the one this
/// build reads, where that refusal is the only one returned and nothing else in
/// the file is looked at.
pub fn read(text: &str, materials: &MaterialTable) -> Result<Input, Refusals> {
    let document = match document::read(text) {
        Ok(document) => document,
        Err(faults) => {
            return Err(Refusals {
                refusals: faults
                    .into_iter()
                    .map(|fault| Refusal {
                        line: Some(fault.line),
                        fault: Fault::Shape {
                            detail: shape_message(&fault.value),
                        },
                    })
                    .collect(),
            });
        }
    };

    let mut reader = Reader {
        materials,
        refusals: Vec::new(),
    };

    // The version first and alone. A file at a version this build does not read
    // is refused whole rather than read as far as it goes.
    let version = reader.format_version(&document.top);
    if version.is_none() || !reader.refusals.is_empty() {
        return Err(reader.into_refusals());
    }

    let input = reader.read_input(&document, version.unwrap_or(FORMAT_VERSION));
    match input {
        Some(input) if reader.refusals.is_empty() => Ok(input),
        _ => Err(reader.into_refusals()),
    }
}

/// What a refusal about the shape of the file says.
fn shape_message(fault: &document::Fault) -> String {
    match fault {
        document::Fault::Syntax { detail } => {
            format!("this is not a file this format can read: {detail}")
        }
        document::Fault::SectionNotKnown { header } => format!(
            "the header [{header}] is not one this format has. A header this build does not \
             know would put every key under it somewhere nobody intended."
        ),
        document::Fault::SectionRepeated { header } => {
            format!("the header [{header}] is written twice, and this format writes it once")
        }
        document::Fault::SectionBracketedWrongly { header, wanted } => format!(
            "the header for {header} is written with the wrong number of brackets, and this \
             format writes it {wanted}. The two forms mean different things."
        ),
        document::Fault::KeyIsDotted => {
            "a key here is written with a dot in it, and this format has no dotted key".to_owned()
        }
        document::Fault::ValueIsADate => {
            "a value here is a date or a time, and no key in this format is one".to_owned()
        }
        document::Fault::NotANumber { found } => format!(
            "{found} is where a number belongs and is not one this format reads. Numbers are \
             written in ordinary decimal."
        ),
        document::Fault::NotUnderstood => {
            "this line is in a form the reader does not know. It refuses what it cannot read \
             rather than reading it as nothing."
                .to_owned()
        }
    }
}

/// The walk from the document into this project's types.
struct Reader<'a> {
    materials: &'a MaterialTable,
    refusals: Vec<Refusal>,
}

impl Reader<'_> {
    fn into_refusals(self) -> Refusals {
        Refusals {
            refusals: self.refusals,
        }
    }

    fn refuse(&mut self, line: usize, fault: Fault) {
        self.refusals.push(Refusal {
            line: Some(line),
            fault,
        });
    }

    /// The version, and the refusals about the keys standing beside it.
    fn format_version(&mut self, top: &Table) -> Option<i64> {
        self.check_keys("the top level of the file", top, &["format_version"]);

        let Some(entry) = top.find("format_version") else {
            self.refusals.push(Refusal {
                line: None,
                fault: Fault::FormatVersionAbsent,
            });
            return None;
        };
        let found = match &entry.node.value {
            Node::Integer(number) => *number,
            other => {
                self.refuse(
                    entry.node.line,
                    Fault::FormatVersionNotRead {
                        found: other.kind().to_owned(),
                    },
                );
                return None;
            }
        };
        if found == FORMAT_VERSION {
            Some(found)
        } else {
            self.refuse(
                entry.node.line,
                Fault::FormatVersionNotRead {
                    found: found.to_string(),
                },
            );
            None
        }
    }

    /// Refuse a repeated key and a key this build does not know, in one place so
    /// that every table gets both checks rather than whichever its reader
    /// remembered.
    fn check_keys(&mut self, table: &str, entries: &Table, known: &[&str]) {
        for entry in entries.repeated() {
            self.refuse(
                entry.line,
                Fault::KeyRepeated {
                    table: table.to_owned(),
                    key: entry.key.clone(),
                },
            );
        }
        for entry in entries.unknown(known) {
            self.refuse(
                entry.line,
                Fault::KeyNotKnown {
                    table: table.to_owned(),
                    key: entry.key.clone(),
                },
            );
        }
    }

    /// The whole file, after the version.
    fn read_input(&mut self, document: &document::Document, version: i64) -> Option<Input> {
        let Some(scene_table) = document.scene.as_ref() else {
            self.refusals.push(Refusal {
                line: None,
                fault: Fault::SceneAbsent,
            });
            return None;
        };
        self.check_keys("[scene]", scene_table, &SCENE_KEYS);

        let name = self.text("[scene]", scene_table, "name", false);
        let origin = self.text("[scene]", scene_table, "origin", true);
        let reference_direction = self.text("[scene]", scene_table, "reference_direction", true);
        let extent = self.extent(scene_table);
        let ground = self.ground(scene_table);

        // The lines are kept beside the elements because the scene refuses on an
        // identifier rather than on a position, and a message pointing at the
        // top of the file would send an operator looking for what they changed.
        let mut surfaces = Vec::new();
        let mut surface_lines = Vec::new();
        for table in &document.surfaces {
            let named = self.identifier(table, "id").unwrap_or_default();
            surface_lines.push((named, line_of(table, "id")));
            if let Some(surface) = self.surface(table) {
                surfaces.push(surface);
            }
        }

        let mut obstacles = Vec::new();
        let mut obstacle_lines = Vec::new();
        for table in &document.obstacles {
            let named = self.identifier(table, "id").unwrap_or_default();
            obstacle_lines.push((named, line_of(table, "id")));
            if let Some(obstacle) = self.obstacle(table) {
                obstacles.push(obstacle);
            }
        }

        let mut holes = Vec::new();
        let mut hole_lines = Vec::new();
        let mut hole_surface_lines = Vec::new();
        let mut trajectories = Vec::new();
        for table in &document.holes {
            let named = self.identifier(table, "id").unwrap_or_default();
            hole_lines.push((named.clone(), line_of(table, "id")));
            hole_surface_lines.push((named.clone(), line_of(table, "surface")));
            let about = format!("the hole {named}");
            if let Some(trajectory) = self.text(&about, table, "trajectory", false) {
                trajectories.push((named, trajectory));
            }
            if let Some(hole) = self.hole(table) {
                holes.push(hole);
            }
        }

        if !self.refusals.is_empty() {
            return None;
        }

        let scene = match Scene::new(surfaces, obstacles, ground?, extent?, holes) {
            Ok(scene) => scene,
            Err(refusal) => {
                let line = line_for(
                    &refusal,
                    &surface_lines,
                    &obstacle_lines,
                    &hole_lines,
                    &hole_surface_lines,
                )
                .unwrap_or(scene_table.line);
                self.refuse(line, Fault::Scene { refusal });
                return None;
            }
        };

        Some(Input {
            format_version: version,
            name,
            origin: origin?,
            reference_direction: reference_direction?,
            scene,
            trajectories,
        })
    }

    /// One surface, or nothing where something in it was refused.
    ///
    /// Every key is read before anything is given up on, so that a surface
    /// missing four of them is refused four times rather than once. An operator
    /// who fixes one key per run is an operator who stops reading the messages.
    fn surface(&mut self, table: &Table) -> Option<Surface> {
        self.check_keys("a [[scene.surface]]", table, &SURFACE_KEYS);
        let id = self.text("a [[scene.surface]]", table, "id", true);
        let named = id
            .as_ref()
            .map_or_else(|| "a surface".to_owned(), |id| format!("the surface {id}"));
        let material = self.text(&named, table, "material", true);
        if let Some(material) = &material {
            self.check_material(table, material, &named);
        }
        let contour = self.contour(table, &named);
        let outline = self.outline(table, &named);
        let azimuth = self.measured(&named, table, "normal_azimuth", Quantity::Angle, true);
        let elevation = self.measured(&named, table, "normal_elevation", Quantity::Angle, true);

        match Surface::new(&id?, &material?, azimuth?, elevation?, outline?, contour?) {
            Ok(surface) => Some(surface),
            Err(refusal) => {
                self.refuse(table.line, Fault::Scene { refusal });
                None
            }
        }
    }

    /// One obstacle, or nothing where something in it was refused.
    fn obstacle(&mut self, table: &Table) -> Option<Obstacle> {
        self.check_keys("a [[scene.obstacle]]", table, &OBSTACLE_KEYS);
        let id = self.text("a [[scene.obstacle]]", table, "id", true);
        let named = id.as_ref().map_or_else(
            || "an obstacle".to_owned(),
            |id| format!("the obstacle {id}"),
        );
        let faces = self.faces(table, &named);
        match Obstacle::new(&id?, faces?) {
            Ok(obstacle) => Some(obstacle),
            Err(refusal) => {
                self.refuse(table.line, Fault::Scene { refusal });
                None
            }
        }
    }

    /// One hole, or nothing where something in it was refused.
    fn hole(&mut self, table: &Table) -> Option<Hole> {
        self.check_keys("a [[hole]]", table, &HOLE_KEYS);
        let id = self.text("a [[hole]]", table, "id", true);
        let named = id
            .as_ref()
            .map_or_else(|| "a hole".to_owned(), |id| format!("the hole {id}"));
        let surface = self.text(&named, table, "surface", true);
        let material = self.text(&named, table, "material", true);
        if let Some(material) = &material {
            self.check_material(table, material, &named);
        }
        let centre = self.centre(table, &named);
        let major = self.measured(&named, table, "major_axis", Quantity::Length, true);
        let minor = self.measured(&named, table, "minor_axis", Quantity::Length, true);
        let bearing = self.measured(&named, table, "major_axis_bearing", Quantity::Angle, true);
        let deformation = self.deformation(table, &named);

        let (major, minor) = (major?, minor?);
        let perforation = match Perforation::new(major, minor, bearing?) {
            Ok(perforation) => perforation,
            Err(refusal) => {
                let line = axis_line(table, &refusal, major);
                self.refuse(
                    line,
                    Fault::Record {
                        about: named,
                        refusal,
                    },
                );
                return None;
            }
        };
        match Hole::new(
            &id?,
            &surface?,
            &material?,
            centre?,
            perforation,
            deformation?,
        ) {
            Ok(hole) => Some(hole),
            Err(refusal) => {
                let line = record_line(table, &refusal);
                self.refuse(
                    line,
                    Fault::Record {
                        about: named,
                        refusal,
                    },
                );
                None
            }
        }
    }

    /// Refuse a material the table has no row for, naming what named it.
    fn check_material(&mut self, table: &Table, material: &str, named_by: &str) {
        if self
            .materials
            .rows()
            .iter()
            .any(|row| row.name() == material)
        {
            return;
        }
        let line = table
            .find("material")
            .map_or(table.line, |entry| entry.line);
        self.refuse(
            line,
            Fault::MaterialHasNoRow {
                material: material.to_owned(),
                named_by: named_by.to_owned(),
            },
        );
    }

    /// The identifier of an element, without refusing where it is absent, so
    /// that a refusal about the element can still name it where it has one.
    fn identifier(&self, table: &Table, key: &str) -> Option<String> {
        match &table.find(key)?.node.value {
            Node::Text(text) => Some(text.clone()),
            _ => None,
        }
    }

    /// A string-valued key.
    fn text(
        &mut self,
        table_name: &str,
        table: &Table,
        key: &'static str,
        required: bool,
    ) -> Option<String> {
        let Some(entry) = table.find(key) else {
            if required {
                self.refuse(
                    table.line,
                    Fault::KeyAbsent {
                        table: table_name.to_owned(),
                        key,
                    },
                );
            }
            return None;
        };
        match &entry.node.value {
            Node::Text(text) => Some(text.clone()),
            other => {
                self.refuse(
                    entry.node.line,
                    Fault::ValueNotOfKind {
                        key: format!("{table_name}, {key},"),
                        wanted: "a string",
                        found: other.kind(),
                    },
                );
                None
            }
        }
    }

    /// The extent of the described space, in metres.
    fn extent(&mut self, scene: &Table) -> Option<Extent> {
        let entry = self.table_valued("[scene]", scene, "extent")?;
        let Node::Table(table) = &entry.node.value else {
            return None;
        };
        self.check_keys("the extent", table, &["x", "y", "z", "unit"]);
        let unit = self.unit("the extent", table, Quantity::Length)?;
        let mut spans = Vec::new();
        for axis in ["x", "y", "z"] {
            let bounds = self.numbers("the extent", table, axis, 2, Quantity::Length, unit)?;
            match Span::new(bounds[0], bounds[1]) {
                Ok(span) => spans.push(span),
                Err(refusal) => {
                    self.refuse(entry.node.line, Fault::Scene { refusal });
                    return None;
                }
            }
        }
        Some(Extent {
            x: spans[0],
            y: spans[1],
            z: spans[2],
        })
    }

    /// Where the ground is.
    fn ground(&mut self, scene: &Table) -> Option<Ground> {
        let level = self.measured("[scene]", scene, "ground", Quantity::Length, true)?;
        match Ground::at(level) {
            Ok(ground) => Some(ground),
            Err(refusal) => {
                let line = scene.find("ground").map_or(scene.line, |entry| entry.line);
                self.refuse(line, Fault::Scene { refusal });
                None
            }
        }
    }

    /// The outline of a surface, as vertices in metres.
    fn outline(&mut self, table: &Table, named: &str) -> Option<Vec<Point>> {
        let entry = self.table_valued(named, table, "outline")?;
        let Node::Table(outline) = &entry.node.value else {
            return None;
        };
        self.check_keys("an outline", outline, &["unit", "vertices"]);
        let unit = self.unit("an outline", outline, Quantity::Length)?;
        let vertices = self.point_list("an outline", outline, "vertices", unit)?;
        Some(vertices)
    }

    /// The faces of an obstacle, as polygons in metres.
    fn faces(&mut self, table: &Table, named: &str) -> Option<Vec<Vec<Point>>> {
        let entry = self.table_valued(named, table, "faces")?;
        let Node::Table(faces) = &entry.node.value else {
            return None;
        };
        self.check_keys("a face list", faces, &["unit", "polygons"]);
        let unit = self.unit("a face list", faces, Quantity::Length)?;
        let Some(entry) = faces.find("polygons") else {
            self.refuse(
                faces.line,
                Fault::KeyAbsent {
                    table: "a face list".to_owned(),
                    key: "polygons",
                },
            );
            return None;
        };
        let Node::Array(polygons) = &entry.node.value else {
            self.refuse(
                entry.node.line,
                Fault::ValueNotOfKind {
                    key: "a face list, polygons,".to_owned(),
                    wanted: "an array",
                    found: entry.node.value.kind(),
                },
            );
            return None;
        };
        let mut read = Vec::new();
        for polygon in polygons {
            read.push(self.points_of(&polygon.value, polygon.line, "a face", unit)?);
        }
        Some(read)
    }

    /// A key holding an array of vertices.
    fn point_list(
        &mut self,
        table_name: &str,
        table: &Table,
        key: &'static str,
        unit: &str,
    ) -> Option<Vec<Point>> {
        let Some(entry) = table.find(key) else {
            self.refuse(
                table.line,
                Fault::KeyAbsent {
                    table: table_name.to_owned(),
                    key,
                },
            );
            return None;
        };
        self.points_of(&entry.node.value, entry.node.line, table_name, unit)
    }

    /// An array of three-number vertices, converted to metres.
    fn points_of(
        &mut self,
        node: &Node,
        line: usize,
        table_name: &str,
        unit: &str,
    ) -> Option<Vec<Point>> {
        let Node::Array(elements) = node else {
            self.refuse(
                line,
                Fault::ValueNotOfKind {
                    key: format!("{table_name},"),
                    wanted: "an array of vertices",
                    found: node.kind(),
                },
            );
            return None;
        };
        let mut points = Vec::new();
        for element in elements {
            let numbers = self.number_array(
                table_name,
                &element.value,
                element.line,
                3,
                Quantity::Length,
                unit,
            )?;
            match Point::new(numbers[0], numbers[1], numbers[2]) {
                Ok(point) => points.push(point),
                Err(refusal) => {
                    self.refuse(element.line, Fault::Scene { refusal });
                    return None;
                }
            }
        }
        Some(points)
    }

    /// The centre of a perforation: three measured coordinates in one table.
    fn centre(&mut self, table: &Table, named: &str) -> Option<Centre> {
        let entry = self.table_valued(named, table, "centre")?;
        let Node::Table(centre) = &entry.node.value else {
            return None;
        };
        let key = format!("{named}, centre,");
        self.check_keys("a centre", centre, &MEASURED_KEYS);

        let form = self.uncertainty_form(&key, centre, entry.node.line)?;
        if form == "unknown" {
            return Some(Centre {
                x: Measured::unestablished(),
                y: Measured::unestablished(),
                z: Measured::unestablished(),
            });
        }

        let unit = self.unit("a centre", centre, Quantity::Length)?;
        let values = self.numbers("a centre", centre, "value", 3, Quantity::Length, unit)?;
        let mut coordinates = Vec::new();
        for (axis, value) in values.iter().enumerate() {
            let measured = if form == "sd" {
                let deviations =
                    self.numbers("a centre", centre, "sd", 3, Quantity::Length, unit)?;
                Measured::deviation(*value, deviations[axis])
            } else {
                let entry = centre.find("interval")?;
                let Node::Array(bounds) = &entry.node.value else {
                    self.refuse(
                        entry.node.line,
                        Fault::ValueNotOfKind {
                            key: format!("{key} interval,"),
                            wanted: "an array of three intervals",
                            found: entry.node.value.kind(),
                        },
                    );
                    return None;
                };
                if bounds.len() != 3 {
                    self.refuse(
                        entry.node.line,
                        Fault::ArrayNotOfLength {
                            key: format!("{key} interval,"),
                            wanted: 3,
                            found: bounds.len(),
                        },
                    );
                    return None;
                }
                let pair = self.number_array(
                    "a centre",
                    &bounds[axis].value,
                    bounds[axis].line,
                    2,
                    Quantity::Length,
                    unit,
                )?;
                Measured::interval(*value, pair[0], pair[1])
            };
            match measured {
                Ok(measured) => coordinates.push(measured),
                Err(refusal) => {
                    self.refuse(
                        entry.node.line,
                        Fault::Record {
                            about: format!("{named}, centre"),
                            refusal,
                        },
                    );
                    return None;
                }
            }
        }
        Some(Centre {
            x: coordinates[0],
            y: coordinates[1],
            z: coordinates[2],
        })
    }

    /// One measured quantity: a value, its unit, and one of the three
    /// uncertainty forms.
    fn measured(
        &mut self,
        table_name: &str,
        table: &Table,
        key: &'static str,
        quantity: Quantity,
        required: bool,
    ) -> Option<Measured> {
        let Some(entry) = table.find(key) else {
            if required {
                self.refuse(
                    table.line,
                    Fault::KeyAbsent {
                        table: table_name.to_owned(),
                        key,
                    },
                );
            }
            return None;
        };
        let named = format!("{table_name}, {key},");
        let Node::Table(measured) = &entry.node.value else {
            self.refuse(
                entry.node.line,
                Fault::ValueNotOfKind {
                    key: named,
                    wanted: "a table in braces carrying a value, a unit and an uncertainty",
                    found: entry.node.value.kind(),
                },
            );
            return None;
        };
        self.check_keys(&named, measured, &MEASURED_KEYS);

        let form = self.uncertainty_form(&named, measured, entry.node.line)?;
        if form == "unknown" {
            return Some(Measured::unestablished());
        }

        let unit = self.unit(&named, measured, quantity)?;
        let value = self.number(&named, measured, "value", quantity, unit)?;
        let built = if form == "sd" {
            let deviation = self.number(&named, measured, "sd", quantity, unit)?;
            Measured::deviation(value, deviation)
        } else {
            let bounds = self.numbers(&named, measured, "interval", 2, quantity, unit)?;
            Measured::interval(value, bounds[0], bounds[1])
        };
        match built {
            Ok(measured) => Some(measured),
            Err(refusal) => {
                self.refuse(
                    entry.node.line,
                    Fault::Record {
                        about: named,
                        refusal,
                    },
                );
                None
            }
        }
    }

    /// Which of the three uncertainty forms a measured value states, refusing
    /// none of them, more than one of them, and a value stated beside
    /// `unknown = true`.
    fn uncertainty_form(
        &mut self,
        named: &str,
        table: &Table,
        line: usize,
    ) -> Option<&'static str> {
        let present: Vec<&'static str> = UNCERTAINTY_FORMS
            .into_iter()
            .filter(|form| table.find(form).is_some())
            .collect();
        match present.as_slice() {
            [] => {
                self.refuse(
                    line,
                    Fault::UncertaintyAbsent {
                        key: named.to_owned(),
                    },
                );
                None
            }
            [one] => {
                if *one != "unknown" {
                    return Some(one);
                }
                let entry = table.find("unknown")?;
                match entry.node.value {
                    Node::Boolean(true) => {}
                    Node::Boolean(false) => {
                        self.refuse(
                            entry.node.line,
                            Fault::UnknownStatedAsFalse {
                                key: named.to_owned(),
                            },
                        );
                        return None;
                    }
                    _ => {
                        self.refuse(
                            entry.node.line,
                            Fault::ValueNotOfKind {
                                key: format!("{named} unknown,"),
                                wanted: "true",
                                found: entry.node.value.kind(),
                            },
                        );
                        return None;
                    }
                }
                if table.find("value").is_some() {
                    self.refuse(
                        entry.node.line,
                        Fault::ValueBesideUnknown {
                            key: named.to_owned(),
                        },
                    );
                    return None;
                }
                Some("unknown")
            }
            [first, second, ..] => {
                self.refuse(
                    line,
                    Fault::UncertaintyStatedTwice {
                        key: named.to_owned(),
                        first,
                        second,
                    },
                );
                None
            }
        }
    }

    /// The unit of a measured value, refused where it is absent or outside the
    /// closed set for this quantity.
    fn unit<'t>(&mut self, named: &str, table: &'t Table, quantity: Quantity) -> Option<&'t str> {
        let Some(entry) = table.find("unit") else {
            self.refuse(
                table.line,
                Fault::UnitAbsent {
                    key: named.to_owned(),
                },
            );
            return None;
        };
        let Node::Text(unit) = &entry.node.value else {
            self.refuse(
                entry.node.line,
                Fault::ValueNotOfKind {
                    key: format!("{named} unit,"),
                    wanted: "a string",
                    found: entry.node.value.kind(),
                },
            );
            return None;
        };
        if quantity.convert(unit, 1.0).is_none() {
            self.refuse(
                entry.node.line,
                Fault::UnitNotKnown {
                    key: named.to_owned(),
                    found: unit.clone(),
                    accepted: quantity.accepted(),
                },
            );
            return None;
        }
        Some(unit)
    }

    /// One number under a key, converted to the internal unit.
    fn number(
        &mut self,
        named: &str,
        table: &Table,
        key: &'static str,
        quantity: Quantity,
        unit: &str,
    ) -> Option<f64> {
        let Some(entry) = table.find(key) else {
            self.refuse(
                table.line,
                Fault::KeyAbsent {
                    table: named.to_owned(),
                    key,
                },
            );
            return None;
        };
        self.number_of(
            &format!("{named} {key},"),
            &entry.node.value,
            entry.node.line,
            quantity,
            unit,
        )
    }

    /// An array of exactly `wanted` numbers under a key, converted.
    fn numbers(
        &mut self,
        named: &str,
        table: &Table,
        key: &'static str,
        wanted: usize,
        quantity: Quantity,
        unit: &str,
    ) -> Option<Vec<f64>> {
        let Some(entry) = table.find(key) else {
            self.refuse(
                table.line,
                Fault::KeyAbsent {
                    table: named.to_owned(),
                    key,
                },
            );
            return None;
        };
        self.number_array(
            named,
            &entry.node.value,
            entry.node.line,
            wanted,
            quantity,
            unit,
        )
    }

    /// An array of exactly `wanted` numbers, converted.
    fn number_array(
        &mut self,
        named: &str,
        node: &Node,
        line: usize,
        wanted: usize,
        quantity: Quantity,
        unit: &str,
    ) -> Option<Vec<f64>> {
        let Node::Array(elements) = node else {
            self.refuse(
                line,
                Fault::ValueNotOfKind {
                    key: named.to_owned(),
                    wanted: "an array of numbers",
                    found: node.kind(),
                },
            );
            return None;
        };
        if elements.len() != wanted {
            self.refuse(
                line,
                Fault::ArrayNotOfLength {
                    key: named.to_owned(),
                    wanted,
                    found: elements.len(),
                },
            );
            return None;
        }
        let mut numbers = Vec::new();
        for element in elements {
            numbers.push(self.number_of(named, &element.value, element.line, quantity, unit)?);
        }
        Some(numbers)
    }

    /// One number, converted to the internal unit, refusing what is not one.
    fn number_of(
        &mut self,
        named: &str,
        node: &Node,
        line: usize,
        quantity: Quantity,
        unit: &str,
    ) -> Option<f64> {
        let written = match node {
            Node::Float(number) => *number,
            // A whole number where a measurement belongs is what somebody writes
            // for a value that happens to be round, and refusing it would be
            // pedantry with no failure behind it.
            Node::Integer(number) => {
                // Widened through the one integer type every value of which has
                // an exact double, so nothing here rounds. A coordinate outside
                // that range is refused rather than rounded quietly.
                let Ok(exact) = i32::try_from(*number) else {
                    self.refuse(
                        line,
                        Fault::ValueNotOfKind {
                            key: named.to_owned(),
                            wanted: "a number",
                            found: "a whole number too large to read without rounding it",
                        },
                    );
                    return None;
                };
                f64::from(exact)
            }
            other => {
                self.refuse(
                    line,
                    Fault::ValueNotOfKind {
                        key: named.to_owned(),
                        wanted: "a number",
                        found: other.kind(),
                    },
                );
                return None;
            }
        };
        if !written.is_finite() {
            self.refuse(
                line,
                Fault::NotFinite {
                    key: named.to_owned(),
                },
            );
            return None;
        }
        let converted = quantity.convert(unit, written)?;
        if converted.is_finite() {
            Some(converted)
        } else {
            self.refuse(
                line,
                Fault::NotFinite {
                    key: named.to_owned(),
                },
            );
            None
        }
    }

    /// A key holding a table, refusing anything else.
    fn table_valued<'t>(
        &mut self,
        table_name: &str,
        table: &'t Table,
        key: &'static str,
    ) -> Option<&'t Entry> {
        let Some(entry) = table.find(key) else {
            self.refuse(
                table.line,
                Fault::KeyAbsent {
                    table: table_name.to_owned(),
                    key,
                },
            );
            return None;
        };
        if matches!(entry.node.value, Node::Table(_)) {
            Some(entry)
        } else {
            self.refuse(
                entry.node.line,
                Fault::ValueNotOfKind {
                    key: format!("{table_name}, {key},"),
                    wanted: "a table in braces",
                    found: entry.node.value.kind(),
                },
            );
            None
        }
    }

    /// The declared contour of a surface.
    ///
    /// ANSI/ASB Standard 196 clause 4.3.4 requires it to be documented, which
    /// `docs/survey/standards.md` establishes by reading the standard, so the
    /// operator already has the value and there is nothing here to default.
    fn contour(&mut self, table: &Table, named: &str) -> Option<Contour> {
        match self
            .word(table, named, "contour", &["flat", "convex", "concave"])?
            .as_str()
        {
            "flat" => Some(Contour::Flat),
            "convex" => Some(Contour::Convex),
            _ => Some(Contour::Concave),
        }
    }

    /// How deformed a perforation was graded.
    fn deformation(&mut self, table: &Table, named: &str) -> Option<Deformation> {
        match self
            .word(
                table,
                named,
                "deformation",
                &["none", "slight", "moderate", "severe", "not-assessed"],
            )?
            .as_str()
        {
            "none" => Some(Deformation::None),
            "slight" => Some(Deformation::Slight),
            "moderate" => Some(Deformation::Moderate),
            "severe" => Some(Deformation::Severe),
            _ => Some(Deformation::NotAssessed),
        }
    }

    /// A key taking one word out of a closed set, refusing anything else.
    ///
    /// The set is listed back in the message, because a word outside it is
    /// almost always a spelling and an operator meeting the refusal needs to
    /// see what was available without opening a document.
    fn word(
        &mut self,
        table: &Table,
        named: &str,
        key: &'static str,
        allowed: &'static [&'static str],
    ) -> Option<String> {
        let text = self.text(named, table, key, true)?;
        if allowed.contains(&text.as_str()) {
            return Some(text);
        }
        let line = table.find(key).map_or(table.line, |entry| entry.node.line);
        self.refuse(
            line,
            Fault::WordNotKnown {
                key: format!("{named}, {key},"),
                found: text,
                accepted: allowed,
            },
        );
        None
    }
}

/// The line a refusal about the ellipse is written on, which is the line of the
/// axis it is about rather than the header of the hole.
///
/// A refusal pointing at the header sends an operator to the top of a record
/// they then have to read through, and the line of the key is the thing they
/// changed.
fn axis_line(table: &Table, refusal: &measurement::Refusal, major: Measured) -> usize {
    let key = match refusal {
        measurement::Refusal::MinorAxisExceedsMajor => "minor_axis",
        measurement::Refusal::NegativeLength => {
            if negative_as_written(major) {
                "major_axis"
            } else {
                "minor_axis"
            }
        }
        measurement::Refusal::AxisNotMeasured => {
            if major.value().is_none() {
                "major_axis"
            } else {
                "minor_axis"
            }
        }
        _ => return table.line,
    };
    line_of(table, key)
}

/// The line a refusal about the hole record is written on.
fn record_line(table: &Table, refusal: &measurement::Refusal) -> usize {
    match refusal {
        measurement::Refusal::CentreNotMeasured => line_of(table, "centre"),
        measurement::Refusal::EmptyIdentifier(which) => line_of(table, which),
        _ => table.line,
    }
}

/// Whether what the operator wrote for a quantity was already below zero,
/// reading a spread as the interval it bounds rather than as its centre.
///
/// A standard deviation is unbounded, so a normal always reaches below zero
/// somewhere, and that is not what this asks. It is the same question
/// `crate::measurement` asks before refusing a negative length, asked here only
/// to decide which of two lines a message points at.
fn negative_as_written(measured: Measured) -> bool {
    measured.value().is_some_and(|value| value < 0.0)
        || match measured.spread() {
            crate::measurement::Spread::Interval { low, .. } => low < 0.0,
            _ => false,
        }
}

/// The line a key is written on, and the table's own line where it is absent.
fn line_of(table: &Table, key: &str) -> usize {
    table.find(key).map_or(table.line, |entry| entry.line)
}

/// The line an element with this identifier was written on, for a refusal the
/// scene made about an identifier rather than about a line.
fn line_for(
    refusal: &scene::Refusal,
    surfaces: &[(String, usize)],
    obstacles: &[(String, usize)],
    holes: &[(String, usize)],
    hole_surfaces: &[(String, usize)],
) -> Option<usize> {
    match refusal {
        scene::Refusal::DuplicateIdentifier(id) => [surfaces, obstacles, holes]
            .into_iter()
            .find_map(|list| second_line_of(list, id)),
        scene::Refusal::UnknownSurface { hole, .. } => hole_surfaces
            .iter()
            .find(|(id, _)| id == hole)
            .map(|(_, line)| *line),
        _ => None,
    }
}

/// The line of the second element carrying this identifier, which is the one
/// that made it a duplicate.
fn second_line_of(list: &[(String, usize)], id: &str) -> Option<usize> {
    list.iter()
        .filter(|(name, _)| name == id)
        .nth(1)
        .map(|(_, line)| *line)
}
