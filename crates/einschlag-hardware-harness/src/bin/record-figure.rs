//! Writes one figure measured with equipment into the form a report can read.
//!
//! What needs the equipment is the measuring, not this program. A person fires
//! at material, or sets up a total station, reads a number off an instrument,
//! and this is the route by which that number enters the record carrying the
//! day it was read and the instrument it was read with. Without those two it is
//! refused, because a figure that lost them reads in a report exactly like one
//! from a run anybody could repeat.
//!
//! It is a binary rather than a test on purpose. `docs/TESTING.md` says why, and
//! the guards in this crate's library refuse the shapes that would undo it.
//!
//! ```text
//! cargo run -p einschlag-hardware-harness --bin record-figure -- \
//!     --quantity "perforation major axis" \
//!     --value 12.4 --unit mm \
//!     --measured-on 2026-08-08 \
//!     --equipment "digital caliper, bench 1"
//! ```

use std::process::ExitCode;

use einschlag_hardware_harness::Figure;

/// What this program understands. Anything else is refused rather than ignored,
/// because a misspelled option that is skipped silently produces a record with
/// a field missing and no sign that anything was dropped.
const OPTIONS: [&str; 5] = [
    "--quantity",
    "--value",
    "--unit",
    "--measured-on",
    "--equipment",
];

fn main() -> ExitCode {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    if arguments.is_empty() {
        print!("{}", usage());
        return ExitCode::from(2);
    }

    match record(&arguments) {
        Ok(line) => {
            println!("{line}");
            ExitCode::SUCCESS
        }
        Err(why) => {
            eprintln!("{why}");
            eprint!("{}", usage());
            ExitCode::from(2)
        }
    }
}

/// Reads the options and records the figure, or says what went wrong.
fn record(arguments: &[String]) -> Result<String, String> {
    let mut quantity = None;
    let mut value = None;
    let mut unit = None;
    let mut measured_on = None;
    let mut equipment = None;

    let mut rest = arguments.iter();
    while let Some(option) = rest.next() {
        if !OPTIONS.contains(&option.as_str()) {
            return Err(format!("{option:?} is not an option this run understands"));
        }
        let given = rest
            .next()
            .ok_or_else(|| format!("{option} was given with nothing after it"))?;
        let slot = match option.as_str() {
            "--quantity" => &mut quantity,
            "--value" => &mut value,
            "--unit" => &mut unit,
            "--measured-on" => &mut measured_on,
            _ => &mut equipment,
        };
        if slot.is_some() {
            return Err(format!("{option} was given twice"));
        }
        *slot = Some(given.clone());
    }

    let quantity = required("--quantity", quantity)?;
    let value = required("--value", value)?;
    let unit = required("--unit", unit)?;
    let measured_on = required("--measured-on", measured_on)?;
    let equipment = required("--equipment", equipment)?;

    let value: f64 = value
        .parse()
        .map_err(|_| format!("the value {value:?} is not a number"))?;

    Figure::record(&quantity, value, &unit, &measured_on, &equipment)
        .map(|figure| figure.line())
        .map_err(|why| why.message())
}

fn required(option: &str, given: Option<String>) -> Result<String, String> {
    given.ok_or_else(|| format!("{option} was not given"))
}

/// What the operator sees when the run is started with nothing, or wrongly.
///
/// It says what the run needs before it says how to type it, because somebody
/// reading this on a machine with no bench and no instrument should find that
/// out here rather than after filling in five options.
fn usage() -> String {
    let tool = einschlag::TOOL_NAME;
    let header = Figure::header();
    format!(
        "{tool} hardware harness: record one figure measured with equipment.

What this run needs: a measurement already made. An instrument, a bench, and a
person who read a number off it. Nothing here drives an instrument and nothing
here invents a figure; it writes down one that was taken, together with the day
it was taken and the equipment used, and refuses a figure missing either.

usage:
  cargo run -p einschlag-hardware-harness --bin record-figure -- \\
      --quantity <what was measured> \\
      --value <number> --unit <unit> \\
      --measured-on <YYYY-MM-DD> \\
      --equipment <the instrument, named by whoever used it>

It writes one line to standard output, in this field order:

  {header}

The date is the day of the measurement and not the day of this run. Recording a
figure a week after the bench work is ordinary; recording it under today's date
is a false statement about when it was taken.

docs/TESTING.md says why this is a binary rather than a test, and what the
guards around that do not cover.
"
    )
}
