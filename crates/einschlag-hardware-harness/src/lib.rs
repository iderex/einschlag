//! Runs that cannot be made without equipment, and the record they produce.
//!
//! Firing at material and measuring the perforation, or surveying a scene with
//! a total station or a scanner, are worth doing and are not tests in the sense
//! the rest of this project uses the word. They need a person, a bench and an
//! instrument, they cannot be repeated on demand, and a suite containing them
//! is a suite that cannot run on a borrowed machine.
//!
//! So they live here, in a crate whose name says what it needs, and a run is a
//! binary under `src/bin` rather than a test. `cargo test` compiles a binary and
//! has no way to start one, which is the whole mechanism: the separation is
//! Cargo's rather than a convention somebody has to remember. The unit tests at
//! the bottom of this file refuse the shapes that would undo it.
//!
//! What the harness produces is figures, and a figure from a run nobody can
//! repeat is still evidence. It is weaker evidence, and the way it stays
//! readable as the weaker kind is that it carries the date it was measured and
//! the equipment it was measured with, wherever it goes. That is what [`Figure`]
//! is: the record refuses to exist without both.
//!
//! `docs/TESTING.md` holds the operator-facing half, including what each run
//! needs and what the guard below does not cover. Issue #53 is where this was
//! argued.

/// The separator between fields of a recorded figure.
///
/// A tab rather than a comma, because an equipment name is written by a person
/// and will contain a comma long before it contains a tab. A field carrying the
/// separator is refused rather than escaped, so the reader on the other side
/// needs no escaping rules.
pub const FIELD_SEPARATOR: char = '\t';

/// The order of the fields in a recorded line, as a header a reader can compare
/// against.
///
/// A recorded figure is read by the calibration report in #52, which does not
/// exist yet. Naming the order here rather than only in the formatting code
/// means the report can be written against a stated shape rather than against
/// whatever this file happened to print on the day.
pub const FIELD_ORDER: [&str; 5] = ["quantity", "value", "unit", "measured-on", "equipment"];

/// A figure produced by a run that needed equipment.
///
/// Constructed only through [`Figure::record`], which refuses anything missing.
/// The fields are private for that reason: a figure that lost its date on the
/// way into a report is a figure a reader will take for a repeatable one.
#[derive(Debug, Clone, PartialEq)]
pub struct Figure {
    quantity: String,
    value: f64,
    unit: String,
    measured_on: MeasuredOn,
    equipment: String,
}

impl Figure {
    /// Records a figure, or says what is missing.
    ///
    /// Every refusal below is a way a figure could reach a report while looking
    /// complete. An empty equipment field is the worst of them, because the
    /// figure still carries a number and a date and reads as though somebody
    /// simply did not bother to say which caliper.
    pub fn record(
        quantity: &str,
        value: f64,
        unit: &str,
        measured_on: &str,
        equipment: &str,
    ) -> Result<Self, Refusal> {
        let quantity = field("quantity", quantity)?;
        let unit = field("unit", unit)?;
        let equipment = field("equipment", equipment)?;
        if !value.is_finite() {
            return Err(Refusal::ValueIsNotFinite(value));
        }
        let measured_on = MeasuredOn::parse(measured_on)?;
        Ok(Self {
            quantity,
            value,
            unit,
            measured_on,
            equipment,
        })
    }

    /// What was measured.
    pub fn quantity(&self) -> &str {
        &self.quantity
    }

    /// The number, in the unit below.
    pub fn value(&self) -> f64 {
        self.value
    }

    /// The unit the number is in. Stated rather than assumed, because a figure
    /// whose unit is understood from context is a figure that changes meaning
    /// when the context is dropped.
    pub fn unit(&self) -> &str {
        &self.unit
    }

    /// The day the measurement was made, which is not the day this ran.
    pub fn measured_on(&self) -> &MeasuredOn {
        &self.measured_on
    }

    /// The instrument, named by whoever used it.
    pub fn equipment(&self) -> &str {
        &self.equipment
    }

    /// The one line a recorded figure is written as, in [`FIELD_ORDER`].
    pub fn line(&self) -> String {
        let separator = FIELD_SEPARATOR;
        format!(
            "{}{separator}{}{separator}{}{separator}{}{separator}{}",
            self.quantity,
            self.value,
            self.unit,
            self.measured_on.text(),
            self.equipment
        )
    }

    /// The header line matching [`Figure::line`], for a file holding several.
    pub fn header() -> String {
        FIELD_ORDER.join(&FIELD_SEPARATOR.to_string())
    }
}

/// A date written the way the operator writes it, `YYYY-MM-DD`.
///
/// It is stated rather than read from the clock. The clock says when the
/// harness ran, and a figure measured at a bench in the morning and recorded in
/// the evening would then carry the wrong day. It is also one fewer thing a run
/// takes from the machine it is on, which
/// `docs/decisions/0009-determinism.md` cares about for a different reason.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MeasuredOn {
    text: String,
}

impl MeasuredOn {
    /// Parses `YYYY-MM-DD`, refusing anything else.
    ///
    /// The check is on the shape and on the ranges, not on the calendar. It
    /// does not know which years are leap years and does not claim to: what it
    /// prevents is a date field holding a note, an empty string or a day and
    /// month written the other way round, which is what actually arrives.
    pub fn parse(text: &str) -> Result<Self, Refusal> {
        let text = text.trim();
        let refuse = || Refusal::DateIsNotADay(text.to_owned());
        let mut parts = text.split('-');
        let year = parts.next().ok_or_else(refuse)?;
        let month = parts.next().ok_or_else(refuse)?;
        let day = parts.next().ok_or_else(refuse)?;
        if parts.next().is_some() {
            return Err(refuse());
        }
        if year.len() != 4 || month.len() != 2 || day.len() != 2 {
            return Err(refuse());
        }
        if !text.chars().all(|c| c.is_ascii_digit() || c == '-') {
            return Err(refuse());
        }
        let month: u32 = month.parse().map_err(|_| refuse())?;
        let day: u32 = day.parse().map_err(|_| refuse())?;
        if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
            return Err(refuse());
        }
        Ok(Self {
            text: text.to_owned(),
        })
    }

    /// The day as it was written, `YYYY-MM-DD`.
    ///
    /// A method rather than the standard formatting trait. Implementing that
    /// trait puts its name in the source, and
    /// `crates/einschlag/tests/headless_and_unprivileged.rs` refuses that name
    /// as a route to a window system. Nothing in this workspace implements it,
    /// so this file follows the tree rather than being the first to argue with
    /// the check.
    pub fn text(&self) -> &str {
        &self.text
    }
}

/// Why a figure was refused.
#[derive(Debug, Clone, PartialEq)]
pub enum Refusal {
    /// A field was empty or was whitespace only.
    FieldIsEmpty(&'static str),
    /// A field carried the separator or a line break, either of which would
    /// split one figure into two records or run two into one.
    FieldSplitsTheRecord(&'static str),
    /// The number was not a number: infinite, or not one at all.
    ValueIsNotFinite(f64),
    /// The date was not `YYYY-MM-DD`.
    DateIsNotADay(String),
}

impl Refusal {
    /// What to put in front of the person who ran this.
    ///
    /// A method rather than the standard formatting trait, for the reason given
    /// at [`MeasuredOn::text`].
    pub fn message(&self) -> String {
        match self {
            Self::FieldIsEmpty(name) => format!(
                "the {name} field is empty, and a figure without it cannot be read \
                 back as the kind of evidence it is"
            ),
            Self::FieldSplitsTheRecord(name) => format!(
                "the {name} field carries a tab or a line break, which would break \
                 the record it is written into"
            ),
            Self::ValueIsNotFinite(value) => format!("the value {value} is not a finite number"),
            Self::DateIsNotADay(text) => {
                format!("the measurement date {text:?} is not a day written as YYYY-MM-DD")
            }
        }
    }
}

/// One text field, trimmed, refused where it is empty or would break the record.
fn field(name: &'static str, value: &str) -> Result<String, Refusal> {
    let value = value.trim();
    if value.contains(FIELD_SEPARATOR) || value.contains('\n') || value.contains('\r') {
        return Err(Refusal::FieldSplitsTheRecord(name));
    }
    Ok(value.to_owned())
}

#[cfg(test)]
mod tests {
    use super::{Figure, MeasuredOn, Refusal, FIELD_ORDER, FIELD_SEPARATOR};
    use std::fs;
    use std::path::{Path, PathBuf};

    fn good() -> Figure {
        Figure::record(
            "perforation major axis",
            12.4,
            "mm",
            "2026-08-08",
            "digital caliper, bench 1",
        )
        .expect("a figure with every field is recorded")
    }

    #[test]
    fn a_figure_carries_the_day_and_the_instrument_into_its_line() {
        let line = good().line();
        assert!(
            line.contains("2026-08-08"),
            "the recorded line lost the measurement date: {line:?}"
        );
        assert!(
            line.contains("digital caliper, bench 1"),
            "the recorded line lost the equipment: {line:?}"
        );
        assert_eq!(
            line.split(FIELD_SEPARATOR).count(),
            FIELD_ORDER.len(),
            "the recorded line does not have one field per name in FIELD_ORDER: {line:?}"
        );
        assert_eq!(
            Figure::header().split(FIELD_SEPARATOR).count(),
            FIELD_ORDER.len(),
            "the header and the line disagree about how many fields there are"
        );
    }

    #[test]
    fn a_figure_with_no_equipment_is_refused() {
        let refusal = Figure::record("major axis", 12.4, "mm", "2026-08-08", "   ")
            .expect_err("a figure with no equipment is refused");
        assert_eq!(refusal, Refusal::FieldIsEmpty("equipment"));
    }

    #[test]
    fn a_figure_with_no_measurement_date_is_refused() {
        let refusal = Figure::record("major axis", 12.4, "mm", "", "caliper")
            .expect_err("a figure with no date is refused");
        assert_eq!(refusal, Refusal::DateIsNotADay(String::new()));
    }

    /// The shape that actually arrives. A European operator writing the day
    /// first produces a string that parses as a date under a lenient reader and
    /// means a different day.
    #[test]
    fn a_date_written_the_other_way_round_is_refused() {
        let refusal = MeasuredOn::parse("08-08-2026").expect_err("a day-first date is refused");
        assert_eq!(refusal, Refusal::DateIsNotADay("08-08-2026".to_owned()));
    }

    #[test]
    fn a_field_that_would_split_the_record_is_refused() {
        let refusal = Figure::record(
            "major axis",
            12.4,
            "mm",
            "2026-08-08",
            "caliper\tand a scanner",
        )
        .expect_err("an equipment field carrying the separator is refused");
        assert_eq!(refusal, Refusal::FieldSplitsTheRecord("equipment"));
    }

    #[test]
    fn a_value_that_is_not_a_number_is_refused() {
        let refusal = Figure::record("major axis", f64::NAN, "mm", "2026-08-08", "caliper")
            .expect_err("a value that is not finite is refused");
        assert!(matches!(refusal, Refusal::ValueIsNotFinite(_)));
    }

    /// The crate directory, from the manifest directory Cargo substitutes.
    fn here() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }

    fn workspace_root() -> PathBuf {
        here()
            .parent()
            .and_then(Path::parent)
            .expect("this crate sits two levels under the workspace root")
            .to_path_buf()
    }

    fn read(path: &Path) -> String {
        fs::read_to_string(path).unwrap_or_else(|why| panic!("cannot read {path:?}: {why}"))
    }

    /// Every file under `src/bin`, which is where a run lives.
    fn run_sources() -> Vec<PathBuf> {
        let bin = here().join("src").join("bin");
        let entries =
            fs::read_dir(&bin).unwrap_or_else(|why| panic!("cannot read {bin:?}: {why}"));
        entries
            .map(|entry| entry.expect("the directory entry is readable").path())
            .filter(|path| path.extension().is_some_and(|kind| kind == "rs"))
            .collect()
    }

    /// The guard the issue asks for, in the direction of keeping runs out.
    ///
    /// A run added as a test rather than as a binary would be started by
    /// `cargo test` on every machine in the project, which is the failure this
    /// crate exists to prevent. Three shapes would do it and all three are
    /// refused here: a `tests` directory, a `[[test]]` target in the manifest,
    /// and a `#[test]` inside a run.
    #[test]
    fn the_default_suite_has_no_run_it_could_start() {
        let tests_directory = here().join("tests");
        assert!(
            !tests_directory.exists(),
            "there is a tests directory at {tests_directory:?}. A run in this crate \
             is a binary under src/bin; the test runner cannot start a binary and it \
             does start a test."
        );

        // A section header on its own line rather than the string anywhere in
        // the file. The manifest explains in a comment why it declares no test
        // target, and a reader that counted the explanation as the thing would
        // refuse the sentence saying the thing is refused. Watched happening.
        let manifest = read(&here().join("Cargo.toml"));
        assert!(
            !manifest
                .lines()
                .map(str::trim)
                .any(|line| line == "[[test]]"),
            "the manifest declares a test target, so `cargo test` would run something \
             in this crate that needs equipment"
        );

        let mut offences = Vec::new();
        for path in run_sources() {
            let text = read(&path);
            for (number, line) in text.lines().enumerate() {
                if line.replace(' ', "").contains("#[test]") {
                    let name = path.file_name().unwrap_or_default().to_string_lossy();
                    offences.push(format!("src/bin/{name}:{}", number + 1));
                }
            }
        }
        assert!(
            offences.is_empty(),
            "a run carries a test attribute at {offences:?}, and `cargo test` runs the \
             unit tests of a binary target. Move the assertion into the run itself."
        );
    }

    /// The other direction the issue asks for: the runs are compiled by the
    /// default suite even though none of them is executed by it, so a change to
    /// the core that breaks one is caught.
    ///
    /// That holds because this crate is an ordinary workspace member and
    /// `cargo test` builds every target of every default member. What would
    /// quietly remove it is a `default-members` list that leaves this crate out,
    /// which is a single line nobody would read as switching off a check.
    #[test]
    fn the_default_suite_compiles_the_runs() {
        const MEMBER: &str = "crates/einschlag-hardware-harness";

        let manifest = read(&workspace_root().join("Cargo.toml"));
        let members = manifest
            .lines()
            .map(str::trim)
            .find(|line| line.starts_with("members"))
            .unwrap_or_else(|| panic!("the workspace manifest has no members line"));
        assert!(
            members.contains(MEMBER),
            "the workspace does not list {MEMBER}, so nothing compiles the runs: {members}"
        );

        if let Some(default_members) = manifest
            .lines()
            .map(str::trim)
            .find(|line| line.starts_with("default-members"))
        {
            assert!(
                default_members.contains(MEMBER),
                "the workspace has a default-members list that leaves {MEMBER} out, so \
                 `cargo test` no longer compiles the runs: {default_members}"
            );
        }
    }

    /// Both guards above walk the tree, and a tree they cannot find is a tree
    /// they report nothing about. Watched happening while writing them: with the
    /// bin directory named wrongly, the run reader returned an empty list and
    /// the guard passed.
    #[test]
    fn the_guards_are_looking_at_something() {
        let runs = run_sources();
        assert!(
            !runs.is_empty(),
            "there is no run under src/bin, so the guard above is reading an empty \
             directory and would pass on a crate with nothing in it"
        );
        assert!(
            workspace_root().join("Cargo.toml").is_file(),
            "the workspace manifest is not where this test looks for it, so the \
             membership guard is reading nothing"
        );
    }
}
