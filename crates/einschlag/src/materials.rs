//! The material table, and the refusal where a material has no row in it.
//!
//! The uncertainty on an angle read off a perforation depends on what the
//! perforation is in. Drywall, sheet metal, glass and wood do not behave alike,
//! and the numbers for that come from the reading in
//! `../../../docs/survey/ellipse-accuracy.md` rather than from judgement.
//!
//! **The table is tracked data and not constants here.**
//! `../../../data/materials.toml` is the file, this module reads it, and
//! `../../../docs/materials.md` states what each key means and why the table
//! holds no rows today. A reader checking a number against the survey opens the
//! data file; nothing about it requires reading this source.
//!
//! **There is no default behind the refusal.** A material with no row, or an
//! angle outside the range a row states, produces [`NoFigure`] and never a
//! number. A default here would be an invented uncertainty handed to a reader as
//! a measured one, which is the failure this project exists to prevent, and it
//! would be invisible in the output because it would look exactly like a
//! measurement.
//!
//! **Angles here are degrees and nothing converts them.** The survey prints
//! degrees, the table states degrees, and the key names say so. The internal
//! unit of the library is the radian, which
//! `../../../docs/decisions/0006-frame-and-units.md` fixes, and the conversion
//! into it belongs at the one boundary issue #34 is about rather than at a
//! second site here. No caller consumes this module yet, so there is nothing to
//! convert for.

/// The tracked table, embedded at build time.
///
/// Read from the file rather than opened at run time so that a binary carries
/// the same table it was built from, and so that nothing depends on a working
/// directory. The file is still the authority a reader checks: this constant is
/// its bytes and not a second copy of them.
pub const TRACKED_TABLE: &str = include_str!("../../../data/materials.toml");

/// Where the tracked table is, for a message that has to tell somebody which
/// file to open.
pub const TRACKED_TABLE_PATH: &str = "data/materials.toml";

/// The one format version this build reads.
const FORMAT_VERSION: u64 = 1;

/// The keys a row is made of. A row missing any of them is refused.
const ROW_KEYS: [&str; 5] = [
    "name",
    "valid_from_deg",
    "valid_to_deg",
    "angle_sd_deg",
    "citation",
];

/// An angle of incidence is measured from the surface, so it runs from zero to
/// a right angle. A range reaching outside that is a mistake in the table
/// rather than an unusual measurement.
const GREATEST_ANGLE_DEG: f64 = 90.0;

/// Why a table was refused.
///
/// One variant per reason rather than a string, so that a caller can act on the
/// reason and so that everything this module refuses is readable in one place.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Fault {
    /// No `format_version` before the first row, or none at all. It is not
    /// assumed to be the version this build reads: a file predating the key and
    /// a file whose author forgot it are indistinguishable.
    FormatVersionAbsent,
    /// A `format_version` this build does not read. The message names what was
    /// found and what is read, so that somebody meeting it knows whether they
    /// need a newer tool or an older table.
    FormatVersionNotRead { found: String },
    /// `format_version` written twice.
    FormatVersionRepeated,
    /// A key at the top level of the file that is not `format_version`, which
    /// is almost always a row key written before its `[[material]]` header.
    KeyOutsideARow { key: String },
    /// A key inside a row that no row has.
    KeyNotInARow { key: String },
    /// A key written twice in one row.
    KeyRepeated { key: String },
    /// A row missing one of the five keys. Named after the row where the name
    /// is there to name it with.
    RowIncomplete { row: String, key: &'static str },
    /// Two rows for one material. Which of them applies would be decided by
    /// whichever the reader happened to reach first.
    MaterialRepeated { name: String },
    /// A material name that is empty or is only whitespace.
    MaterialNameEmpty,
    /// A row whose citation is empty. This is the guard that keeps the table
    /// from filling up with plausible numbers.
    CitationEmpty { row: String },
    /// A value that is not a number where one is required.
    NotANumber { key: String, found: String },
    /// A number that is not finite. An infinity or a nothing arriving here
    /// would carry through every draw made from it.
    NotFinite { key: String },
    /// A range whose lower bound is not below its upper bound.
    RangeInverted { row: String },
    /// A range reaching below zero or above a right angle.
    RangeOutsideTheAngles { row: String },
    /// A standard deviation that is not above zero. Zero asserts that the
    /// relation is exact on that material over that range, which no reading in
    /// the survey supports and which would narrow an answer for free.
    UncertaintyNotAboveZero { row: String },
    /// A section header that is not `[[material]]`.
    SectionNotKnown { header: String },
    /// A line that is not blank, a comment, a section header or `key = value`.
    LineNotUnderstood { text: String },
    /// A string value that is not one pair of quotes around text with no
    /// escapes in it. The reader handles the forms the table uses and refuses
    /// the rest rather than guessing at them.
    StringNotUnderstood { key: String, found: String },
}

/// A refusal, with the line it was refused at.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Refusal {
    /// The line in the table, counting from one. `None` where the fault is the
    /// file as a whole rather than a line in it.
    pub line: Option<usize>,
    /// What was wrong.
    pub fault: Fault,
}

impl Refusal {
    /// One sentence saying which line was refused, where there is one, and what
    /// was wrong.
    ///
    /// It names no file, because this type is produced from text and the text
    /// is not always the tracked table: a message naming `data/materials.toml`
    /// while a fixture was being read would send somebody to the wrong file.
    /// [`Refusal::message_from`] is for a caller that knows where the text came
    /// from.
    ///
    /// A plain method rather than the standard formatting trait, and a choice
    /// rather than a workaround since #111: the check in
    /// `crates/einschlag/tests/headless_and_unprivileged.rs` reads the form
    /// now rather than the bare name. It stays a method because the trait is
    /// what `to_string` and every formatting macro reach, and this text is a
    /// message about one file rather than the rendering of the type.
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

impl Fault {
    /// What was wrong, in the words somebody meeting it needs.
    #[must_use]
    pub fn message(&self) -> String {
        match self {
            Self::FormatVersionAbsent => {
                "no format_version before the first row. It is required, and a table without \
                 it is not read as version 1, because a table predating the key and a table \
                 whose author forgot it look the same."
                    .to_owned()
            }
            Self::FormatVersionNotRead { found } => format!(
                "format_version {found}, and this build reads {FORMAT_VERSION}. Nothing is \
                 read out of a table at a version this build does not know."
            ),
            Self::FormatVersionRepeated => "format_version is stated twice".to_owned(),
            Self::KeyOutsideARow { key } => format!(
                "the key {key} is at the top level of the file, where only format_version \
                 belongs. A row key needs a [[material]] header above it."
            ),
            Self::KeyNotInARow { key } => format!(
                "the key {key} is not one a row has. A row has {}.",
                ROW_KEYS.join(", ")
            ),
            Self::KeyRepeated { key } => {
                format!("the key {key} is written twice in one row")
            }
            Self::RowIncomplete { row, key } => format!(
                "the row for {row} has no {key}, and every row needs all five of {}.",
                ROW_KEYS.join(", ")
            ),
            Self::MaterialRepeated { name } => format!(
                "two rows for {name}. Which one applied would be decided by which the \
                 reader reached first."
            ),
            Self::MaterialNameEmpty => {
                "a row whose name is empty, which no hole can name".to_owned()
            }
            Self::CitationEmpty { row } => format!(
                "the row for {row} has an empty citation. A row states what the survey \
                 measured, so it names where in the survey that was read; a row nobody can \
                 follow back is a plausible number."
            ),
            Self::NotANumber { key, found } => {
                format!("the value of {key} is {found}, which is not a number")
            }
            Self::NotFinite { key } => {
                format!("the value of {key} is not a finite number")
            }
            Self::RangeInverted { row } => format!(
                "the row for {row} states valid_from_deg at or above valid_to_deg, so it is \
                 valid over nothing"
            ),
            Self::RangeOutsideTheAngles { row } => format!(
                "the row for {row} states a range reaching outside 0 to {GREATEST_ANGLE_DEG} \
                 degrees, and an angle of incidence is measured from the surface"
            ),
            Self::UncertaintyNotAboveZero { row } => format!(
                "the row for {row} states an angle_sd_deg that is not above zero, which \
                 asserts the relation is exact on that material"
            ),
            Self::SectionNotKnown { header } => format!(
                "the section {header} is not one this table has. A row opens with \
                 [[material]]."
            ),
            Self::LineNotUnderstood { text } => format!(
                "the line {text:?} is not blank, a comment, a [[material]] header or \
                 key = value. The reader refuses what it does not recognise rather than \
                 reading past it, because reading past a line is how a value goes missing \
                 without anybody being told."
            ),
            Self::StringNotUnderstood { key, found } => format!(
                "the value of {key} is {found}, and a text value here is one pair of \
                 quotes around text with no escape in it"
            ),
        }
    }
}

/// One material, over one range of angles, with the figure the survey supports
/// and where it was read.
#[derive(Debug, Clone, PartialEq)]
pub struct Row {
    name: String,
    valid_from_deg: f64,
    valid_to_deg: f64,
    angle_sd_deg: f64,
    citation: String,
}

impl Row {
    /// The material this row is about.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The lowest angle of incidence, in degrees, the row is valid at.
    #[must_use]
    pub fn valid_from_deg(&self) -> f64 {
        self.valid_from_deg
    }

    /// The highest angle of incidence, in degrees, the row is valid at.
    #[must_use]
    pub fn valid_to_deg(&self) -> f64 {
        self.valid_to_deg
    }

    /// The standard deviation this material contributes to an estimated angle,
    /// in degrees.
    #[must_use]
    pub fn angle_sd_deg(&self) -> f64 {
        self.angle_sd_deg
    }

    /// Where in the survey this row was read.
    #[must_use]
    pub fn citation(&self) -> &str {
        &self.citation
    }
}

/// What the table has to say about one hole.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Figure<'a> {
    /// The standard deviation on the estimated angle, in degrees.
    pub angle_sd_deg: f64,
    /// Where in the survey it was read, so that the number in an output can be
    /// followed back to a publication.
    pub citation: &'a str,
}

/// Why the table has nothing to say about a hole.
///
/// It is a value rather than a message so that a caller can tell the two apart
/// without reading text, and so that neither of them can be turned into a
/// number further down.
#[derive(Debug, Clone, PartialEq)]
pub enum NoFigure {
    /// No row names this material. This project has read no error for it.
    MaterialHasNoRow { material: String },
    /// A row names it and does not reach this angle.
    AngleOutsideTheRowsRange {
        /// The material asked about.
        material: String,
        /// The angle asked about, in degrees.
        angle_deg: f64,
        /// The lowest angle the row is valid at, in degrees.
        valid_from_deg: f64,
        /// The highest angle the row is valid at, in degrees.
        valid_to_deg: f64,
    },
    /// An angle that is not a finite number was asked about.
    AngleNotFinite {
        /// The material asked about.
        material: String,
    },
}

impl NoFigure {
    /// What is missing, naming the material, so that a message reaching an
    /// operator says which hole to look at.
    ///
    /// A plain method rather than the standard formatting trait, for the reason
    /// given at [`Refusal::message`].
    #[must_use]
    pub fn message(&self) -> String {
        match self {
            Self::MaterialHasNoRow { material } => format!(
                "the material {material} has no row in the material table, so this project \
                 has read no error for it and has nothing to offer about a hole in it. \
                 docs/materials.md says what would add a row."
            ),
            Self::AngleOutsideTheRowsRange {
                material,
                angle_deg,
                valid_from_deg,
                valid_to_deg,
            } => format!(
                "the row for {material} in the material table was read over {valid_from_deg} \
                 to {valid_to_deg} degrees and this hole is at {angle_deg} degrees, which it \
                 says nothing about"
            ),
            Self::AngleNotFinite { material } => format!(
                "the angle asked about for {material} is not a finite number, so no row \
                 could be valid over it"
            ),
        }
    }
}

/// The table.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MaterialTable {
    rows: Vec<Row>,
}

impl MaterialTable {
    /// The tracked table, read from the bytes embedded at build time.
    ///
    /// # Errors
    ///
    /// Every [`Fault`], where the tracked file has one. A unit test in this
    /// module reads it, so a table that stopped parsing reddens the suite rather
    /// than reaching a caller.
    pub fn tracked() -> Result<Self, Refusal> {
        Self::parse(TRACKED_TABLE)
    }

    /// Read a table out of text.
    ///
    /// Public so that the guards can be shown biting against a fixture rather
    /// than against the tracked file. A row that judges against the real table
    /// proves the state of the tree on the day it ran and not the guard.
    ///
    /// # Errors
    ///
    /// Every [`Fault`].
    pub fn parse(text: &str) -> Result<Self, Refusal> {
        Reader::new().read(text)
    }

    /// The rows, in the order the file states them.
    #[must_use]
    pub fn rows(&self) -> &[Row] {
        &self.rows
    }

    /// What the table says about a hole in `material` at `angle_deg`, an angle
    /// of incidence in degrees.
    ///
    /// # Errors
    ///
    /// [`NoFigure`] where no row names the material, where the row that does
    /// was not read over this angle, or where the angle is not a finite number.
    /// There is no fourth outcome and no default: a caller that gets a refusal
    /// has nothing here to fall back on.
    pub fn figure_for(&self, material: &str, angle_deg: f64) -> Result<Figure<'_>, NoFigure> {
        let Some(row) = self.rows.iter().find(|row| row.name == material) else {
            return Err(NoFigure::MaterialHasNoRow {
                material: material.to_owned(),
            });
        };
        if !angle_deg.is_finite() {
            return Err(NoFigure::AngleNotFinite {
                material: material.to_owned(),
            });
        }
        if angle_deg < row.valid_from_deg || angle_deg > row.valid_to_deg {
            return Err(NoFigure::AngleOutsideTheRowsRange {
                material: material.to_owned(),
                angle_deg,
                valid_from_deg: row.valid_from_deg,
                valid_to_deg: row.valid_to_deg,
            });
        }
        Ok(Figure {
            angle_sd_deg: row.angle_sd_deg,
            citation: &row.citation,
        })
    }
}

/// A row being assembled, one key at a time.
#[derive(Default)]
struct PartialRow {
    line: usize,
    name: Option<String>,
    valid_from_deg: Option<f64>,
    valid_to_deg: Option<f64>,
    angle_sd_deg: Option<f64>,
    citation: Option<String>,
}

impl PartialRow {
    /// The name so far, for a message about a row that has not got one yet.
    fn label(&self) -> String {
        match &self.name {
            Some(name) => name.clone(),
            None => format!("the row opened at line {}", self.line),
        }
    }
}

/// The reader.
///
/// It handles the forms the table uses and refuses everything else. The cost of
/// reading text with a narrow reader is that an unknown form could be read as
/// nothing at all, so every unknown form here is a refusal instead. The same
/// argument and the same shape are in
/// `crates/einschlag/tests/dependency_budget.rs`, which reads the manifests
/// without a library for the same reason: a library to read this file would be
/// the first thing entered in the document that counts dependencies.
struct Reader {
    version: Option<u64>,
    rows: Vec<Row>,
    current: Option<PartialRow>,
}

impl Reader {
    fn new() -> Self {
        Self {
            version: None,
            rows: Vec::new(),
            current: None,
        }
    }

    fn read(mut self, text: &str) -> Result<MaterialTable, Refusal> {
        for (index, raw) in text.lines().enumerate() {
            let line = index + 1;
            let content = strip_comment(raw).trim();
            if content.is_empty() {
                continue;
            }
            if let Some(header) = content.strip_prefix('[') {
                self.open_section(line, content, header)?;
                continue;
            }
            let Some((key, value)) = content.split_once('=') else {
                return Err(Refusal {
                    line: Some(line),
                    fault: Fault::LineNotUnderstood {
                        text: content.to_owned(),
                    },
                });
            };
            self.take_key(line, key.trim(), value.trim())?;
        }
        self.close_row()?;
        if self.version.is_none() {
            return Err(Refusal {
                line: None,
                fault: Fault::FormatVersionAbsent,
            });
        }
        Ok(MaterialTable { rows: self.rows })
    }

    fn open_section(&mut self, line: usize, content: &str, header: &str) -> Result<(), Refusal> {
        if header.trim_end_matches(']').trim_start_matches('[') != "material"
            || !content.ends_with(']')
        {
            return Err(Refusal {
                line: Some(line),
                fault: Fault::SectionNotKnown {
                    header: content.to_owned(),
                },
            });
        }
        if self.version.is_none() {
            return Err(Refusal {
                line: Some(line),
                fault: Fault::FormatVersionAbsent,
            });
        }
        self.close_row()?;
        self.current = Some(PartialRow {
            line,
            ..PartialRow::default()
        });
        Ok(())
    }

    fn take_key(&mut self, line: usize, key: &str, value: &str) -> Result<(), Refusal> {
        let Some(row) = self.current.as_mut() else {
            if key != "format_version" {
                return Err(Refusal {
                    line: Some(line),
                    fault: Fault::KeyOutsideARow {
                        key: key.to_owned(),
                    },
                });
            }
            if self.version.is_some() {
                return Err(Refusal {
                    line: Some(line),
                    fault: Fault::FormatVersionRepeated,
                });
            }
            let found = value.parse::<u64>().map_err(|_| Refusal {
                line: Some(line),
                fault: Fault::NotANumber {
                    key: key.to_owned(),
                    found: value.to_owned(),
                },
            })?;
            if found != FORMAT_VERSION {
                return Err(Refusal {
                    line: Some(line),
                    fault: Fault::FormatVersionNotRead {
                        found: found.to_string(),
                    },
                });
            }
            self.version = Some(found);
            return Ok(());
        };

        match key {
            "name" => {
                let name = read_string(line, key, value)?;
                if name.trim().is_empty() {
                    return Err(Refusal {
                        line: Some(line),
                        fault: Fault::MaterialNameEmpty,
                    });
                }
                set_once(&mut row.name, name, line, key)
            }
            "citation" => {
                let citation = read_string(line, key, value)?;
                if citation.trim().is_empty() {
                    return Err(Refusal {
                        line: Some(line),
                        fault: Fault::CitationEmpty { row: row.label() },
                    });
                }
                set_once(&mut row.citation, citation, line, key)
            }
            "valid_from_deg" => set_once(
                &mut row.valid_from_deg,
                read_number(line, key, value)?,
                line,
                key,
            ),
            "valid_to_deg" => set_once(
                &mut row.valid_to_deg,
                read_number(line, key, value)?,
                line,
                key,
            ),
            "angle_sd_deg" => set_once(
                &mut row.angle_sd_deg,
                read_number(line, key, value)?,
                line,
                key,
            ),
            _ => Err(Refusal {
                line: Some(line),
                fault: Fault::KeyNotInARow {
                    key: key.to_owned(),
                },
            }),
        }
    }

    fn close_row(&mut self) -> Result<(), Refusal> {
        let Some(row) = self.current.take() else {
            return Ok(());
        };
        let line = Some(row.line);
        let label = row.label();
        let missing = |key: &'static str| Refusal {
            line,
            fault: Fault::RowIncomplete {
                row: label.clone(),
                key,
            },
        };
        let name = row.name.clone().ok_or_else(|| missing("name"))?;
        let valid_from_deg = row
            .valid_from_deg
            .ok_or_else(|| missing("valid_from_deg"))?;
        let valid_to_deg = row.valid_to_deg.ok_or_else(|| missing("valid_to_deg"))?;
        let angle_sd_deg = row.angle_sd_deg.ok_or_else(|| missing("angle_sd_deg"))?;
        let citation = row.citation.clone().ok_or_else(|| missing("citation"))?;

        if self.rows.iter().any(|existing| existing.name == name) {
            return Err(Refusal {
                line,
                fault: Fault::MaterialRepeated { name },
            });
        }
        if valid_from_deg >= valid_to_deg {
            return Err(Refusal {
                line,
                fault: Fault::RangeInverted { row: name },
            });
        }
        if valid_from_deg < 0.0 || valid_to_deg > GREATEST_ANGLE_DEG {
            return Err(Refusal {
                line,
                fault: Fault::RangeOutsideTheAngles { row: name },
            });
        }
        // A value that is not a number was already refused where it was read,
        // so this comparison is not the one that lets one through.
        if angle_sd_deg <= 0.0 {
            return Err(Refusal {
                line,
                fault: Fault::UncertaintyNotAboveZero { row: name },
            });
        }

        self.rows.push(Row {
            name,
            valid_from_deg,
            valid_to_deg,
            angle_sd_deg,
            citation,
        });
        Ok(())
    }
}

/// Fill a key that has not been filled, or refuse the second one.
fn set_once<T>(slot: &mut Option<T>, value: T, line: usize, key: &str) -> Result<(), Refusal> {
    if slot.is_some() {
        return Err(Refusal {
            line: Some(line),
            fault: Fault::KeyRepeated {
                key: key.to_owned(),
            },
        });
    }
    *slot = Some(value);
    Ok(())
}

/// Everything before a `#` that is not inside a pair of quotes.
fn strip_comment(line: &str) -> &str {
    let mut inside = false;
    for (at, character) in line.char_indices() {
        match character {
            '"' => inside = !inside,
            '#' if !inside => return &line[..at],
            _ => {}
        }
    }
    line
}

/// One pair of quotes around text with no escape in it.
fn read_string(line: usize, key: &str, value: &str) -> Result<String, Refusal> {
    let refused = || Refusal {
        line: Some(line),
        fault: Fault::StringNotUnderstood {
            key: key.to_owned(),
            found: value.to_owned(),
        },
    };
    let inner = value
        .strip_prefix('"')
        .and_then(|rest| rest.strip_suffix('"'))
        .ok_or_else(refused)?;
    if inner.contains('"') || inner.contains('\\') {
        return Err(refused());
    }
    Ok(inner.to_owned())
}

/// A finite number.
fn read_number(line: usize, key: &str, value: &str) -> Result<f64, Refusal> {
    let number = value.parse::<f64>().map_err(|_| Refusal {
        line: Some(line),
        fault: Fault::NotANumber {
            key: key.to_owned(),
            found: value.to_owned(),
        },
    })?;
    if !number.is_finite() {
        return Err(Refusal {
            line: Some(line),
            fault: Fault::NotFinite {
                key: key.to_owned(),
            },
        });
    }
    Ok(number)
}

#[cfg(test)]
mod tests {
    use super::{Fault, MaterialTable, NoFigure, TRACKED_TABLE, TRACKED_TABLE_PATH};

    /// A row one field away from a valid one is what every fixture below is
    /// built from, so that each proves a mistake somebody would actually make.
    const VALID: &str = r#"
format_version = 1

[[material]]
name = "gypsum-wallboard-12mm"
valid_from_deg = 40.0
valid_to_deg = 90.0
angle_sd_deg = 2.6
citation = "a fixture, and not a reading of any publication"
"#;

    fn parse_fault(text: &str) -> Fault {
        MaterialTable::parse(text)
            .expect_err("this fixture is supposed to be refused")
            .fault
    }

    #[test]
    fn the_tracked_table_is_read_by_the_reader_that_ships_with_it() {
        let table = MaterialTable::tracked().unwrap_or_else(|why| {
            panic!(
                "the tracked table is refused: {}",
                why.message_from(TRACKED_TABLE_PATH)
            )
        });
        // Not an assertion that the table is empty. The count moves the day a
        // row lands, and a test that had to be edited then would be a test
        // somebody edits without reading.
        for row in table.rows() {
            assert!(
                !row.citation().trim().is_empty(),
                "the row for {} carries no citation",
                row.name()
            );
        }
    }

    #[test]
    fn the_fixture_the_others_are_built_from_is_valid() {
        // Without this, every fixture below could be passing for the wrong
        // reason: a change breaking the reader outright would refuse them all
        // and every test would still be green.
        let table = MaterialTable::parse(VALID).expect("the valid fixture parses");
        assert_eq!(table.rows().len(), 1, "the valid fixture has one row");
    }

    #[test]
    fn a_row_without_a_citation_is_refused() {
        let without = VALID.replace(
            "citation = \"a fixture, and not a reading of any publication\"\n",
            "",
        );
        assert_eq!(
            parse_fault(&without),
            Fault::RowIncomplete {
                row: "gypsum-wallboard-12mm".to_owned(),
                key: "citation",
            }
        );
    }

    #[test]
    fn a_row_whose_citation_is_empty_is_refused() {
        // The near miss of the test above: the key is there, so a reader
        // checking that every row has one would pass this.
        let emptied = VALID.replace("a fixture, and not a reading of any publication", "  ");
        assert_eq!(
            parse_fault(&emptied),
            Fault::CitationEmpty {
                row: "gypsum-wallboard-12mm".to_owned(),
            }
        );
    }

    #[test]
    fn a_row_missing_any_other_key_is_refused() {
        for (line, key) in [
            ("name = \"gypsum-wallboard-12mm\"\n", "name"),
            ("valid_from_deg = 40.0\n", "valid_from_deg"),
            ("valid_to_deg = 90.0\n", "valid_to_deg"),
            ("angle_sd_deg = 2.6\n", "angle_sd_deg"),
        ] {
            let without = VALID.replace(line, "");
            let fault = parse_fault(&without);
            assert!(
                matches!(fault, Fault::RowIncomplete { key: missing, .. } if missing == key),
                "removing {key} was refused as {fault:?}"
            );
        }
    }

    #[test]
    fn a_table_with_no_format_version_is_refused() {
        let without = VALID.replace("format_version = 1\n", "");
        assert_eq!(parse_fault(&without), Fault::FormatVersionAbsent);
    }

    #[test]
    fn a_format_version_this_build_does_not_read_is_refused() {
        let later = VALID.replace("format_version = 1", "format_version = 2");
        assert_eq!(
            parse_fault(&later),
            Fault::FormatVersionNotRead {
                found: "2".to_owned(),
            }
        );
    }

    #[test]
    fn a_key_that_no_row_has_is_refused() {
        let extra = VALID.replace("angle_sd_deg = 2.6", "angle_sd_degrees = 2.6");
        assert_eq!(
            parse_fault(&extra),
            Fault::KeyNotInARow {
                key: "angle_sd_degrees".to_owned(),
            }
        );
    }

    #[test]
    fn a_second_row_for_one_material_is_refused() {
        let twice = format!(
            "{VALID}\n[[material]]\nname = \"gypsum-wallboard-12mm\"\nvalid_from_deg = 10.0\n\
             valid_to_deg = 20.0\nangle_sd_deg = 9.0\ncitation = \"a second fixture row\"\n"
        );
        assert_eq!(
            parse_fault(&twice),
            Fault::MaterialRepeated {
                name: "gypsum-wallboard-12mm".to_owned(),
            }
        );
    }

    #[test]
    fn a_range_that_is_inverted_or_outside_the_angles_is_refused() {
        let inverted = VALID.replace("valid_from_deg = 40.0", "valid_from_deg = 95.0");
        assert_eq!(
            parse_fault(&inverted),
            Fault::RangeInverted {
                row: "gypsum-wallboard-12mm".to_owned(),
            }
        );
        let outside = VALID.replace("valid_to_deg = 90.0", "valid_to_deg = 91.0");
        assert_eq!(
            parse_fault(&outside),
            Fault::RangeOutsideTheAngles {
                row: "gypsum-wallboard-12mm".to_owned(),
            }
        );
    }

    #[test]
    fn an_uncertainty_that_is_not_above_zero_is_refused() {
        let flat = VALID.replace("angle_sd_deg = 2.6", "angle_sd_deg = 0.0");
        assert_eq!(
            parse_fault(&flat),
            Fault::UncertaintyNotAboveZero {
                row: "gypsum-wallboard-12mm".to_owned(),
            }
        );
    }

    #[test]
    fn a_line_the_reader_does_not_understand_is_refused_rather_than_read_past() {
        let stray = VALID.replace("angle_sd_deg = 2.6", "angle_sd_deg");
        assert_eq!(
            parse_fault(&stray),
            Fault::LineNotUnderstood {
                text: "angle_sd_deg".to_owned(),
            }
        );
    }

    #[test]
    fn a_section_that_is_not_a_material_is_refused() {
        let other = VALID.replace("[[material]]", "[[materials]]");
        assert_eq!(
            parse_fault(&other),
            Fault::SectionNotKnown {
                header: "[[materials]]".to_owned(),
            }
        );
    }

    #[test]
    fn a_number_that_is_not_one_is_refused() {
        let text = VALID.replace("angle_sd_deg = 2.6", "angle_sd_deg = 2,6");
        assert_eq!(
            parse_fault(&text),
            Fault::NotANumber {
                key: "angle_sd_deg".to_owned(),
                found: "2,6".to_owned(),
            }
        );
    }

    #[test]
    fn a_hole_in_a_material_with_a_row_gets_the_figure_and_its_citation() {
        let table = MaterialTable::parse(VALID).expect("the valid fixture parses");
        let figure = table
            .figure_for("gypsum-wallboard-12mm", 60.0)
            .expect("the row reaches 60 degrees");
        assert!(
            (figure.angle_sd_deg - 2.6).abs() < 1e-12,
            "the figure is {}",
            figure.angle_sd_deg
        );
        assert_eq!(
            figure.citation,
            "a fixture, and not a reading of any publication"
        );
    }

    #[test]
    fn a_material_with_no_row_is_refused_and_the_message_names_it() {
        let table = MaterialTable::parse(VALID).expect("the valid fixture parses");
        let refusal = table
            .figure_for("float-glass-4mm", 60.0)
            .expect_err("no row names float glass");
        assert_eq!(
            refusal,
            NoFigure::MaterialHasNoRow {
                material: "float-glass-4mm".to_owned(),
            }
        );
        assert!(
            refusal.message().contains("float-glass-4mm"),
            "the message does not name the material: {}",
            refusal.message()
        );
    }

    #[test]
    fn an_angle_outside_the_rows_range_is_refused_and_the_message_names_the_range() {
        let table = MaterialTable::parse(VALID).expect("the valid fixture parses");
        let refusal = table
            .figure_for("gypsum-wallboard-12mm", 20.0)
            .expect_err("the row starts at 40 degrees");
        assert!(
            matches!(refusal, NoFigure::AngleOutsideTheRowsRange { .. }),
            "refused as {refusal:?}"
        );
        let message = refusal.message();
        for part in ["gypsum-wallboard-12mm", "40", "90", "20"] {
            assert!(
                message.contains(part),
                "the message does not name {part}: {message}"
            );
        }
    }

    #[test]
    fn the_tracked_table_offers_nothing_for_a_material_it_has_no_row_for() {
        // The property this issue is about, asserted against the table that
        // ships rather than against a fixture: there is no fourth outcome and
        // nothing behind the refusal.
        let table = MaterialTable::tracked().expect("the tracked table parses");
        for material in [
            "gypsum-wallboard-12mm",
            "sheet-metal-1mm",
            "float-glass-4mm",
        ] {
            for angle in [5.0, 45.0, 90.0] {
                let outcome = table.figure_for(material, angle);
                assert!(
                    outcome.is_err(),
                    "the tracked table produced a figure for {material} at {angle} degrees: \
                     {outcome:?}"
                );
            }
        }
    }

    #[test]
    fn an_angle_that_is_not_a_number_gets_a_refusal_rather_than_a_row() {
        let table = MaterialTable::parse(VALID).expect("the valid fixture parses");
        let refusal = table
            .figure_for("gypsum-wallboard-12mm", f64::NAN)
            .expect_err("an angle that is not a number reaches no row");
        assert!(matches!(refusal, NoFigure::AngleNotFinite { .. }));
    }

    #[test]
    fn a_comment_is_read_as_a_comment_and_a_quoted_hash_is_not() {
        let commented = VALID.replace(
            "citation = \"a fixture, and not a reading of any publication\"",
            "citation = \"a fixture # not a comment\" # and this is one",
        );
        let table = MaterialTable::parse(&commented).expect("the fixture parses");
        assert_eq!(table.rows()[0].citation(), "a fixture # not a comment");
    }

    #[test]
    fn the_tracked_table_constant_is_the_tracked_file() {
        assert!(
            TRACKED_TABLE.contains("format_version"),
            "the embedded table carries no format_version, so the include is not the file"
        );
    }
}
