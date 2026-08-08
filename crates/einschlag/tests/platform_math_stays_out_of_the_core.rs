//! Refuses a call from the numeric core to a floating-point function whose
//! precision the standard library does not specify.
//!
//! `docs/decisions/0009-determinism.md` promises byte-identical output from the
//! same input, the same seed and the same build. The documentation of `f64`
//! attaches an "Unspecified precision" note to its transcendental methods,
//! saying the precision varies by platform, by compiler release, and may differ
//! within one execution from one invocation to the next. The last of those is
//! inside the promise rather than inside its stated bound.
//!
//! `docs/decisions/0013-platform-math-out-of-the-numeric-core.md` takes `libm`
//! at a pinned version instead, and `crates/einschlag/src/math.rs` is the one
//! route. This is the mechanism that keeps the other route shut. A convention
//! would not: the platform's version of every name below is a method on `f64`,
//! reachable by typing a dot, and it compiles, runs and returns a number that is
//! right to within a last place. Nothing about the wrong call looks wrong.
//!
//! What is refused is the name, not the arithmetic. Square root, fused multiply
//! add and the operators are correctly rounded under IEEE 754 and are not on the
//! list, because the platform has no latitude in them.

use std::fs;
use std::path::{Path, PathBuf};

/// The methods carrying the "Unspecified precision" note, read from
/// `https://doc.rust-lang.org/std/primitive.f64.html` on 2026-08-08.
///
/// The two angle conversions are on the list for the same reason as the rest:
/// the note is attached to them as well. `crates/einschlag/src/math.rs` supplies
/// both as one multiplication each, which is exact.
const UNSPECIFIED_PRECISION: [&str; 28] = [
    "powi",
    "powf",
    "exp",
    "exp2",
    "ln",
    "log",
    "log2",
    "log10",
    "abs_sub",
    "cbrt",
    "hypot",
    "sin",
    "cos",
    "tan",
    "asin",
    "acos",
    "atan",
    "atan2",
    "sin_cos",
    "exp_m1",
    "ln_1p",
    "sinh",
    "cosh",
    "tanh",
    "asinh",
    "acosh",
    "atanh",
    "to_degrees",
];

/// Named on its own because the list above is one entry short of the note's
/// coverage and a reader counting them should find the missing one here rather
/// than assume it was forgotten. `to_radians` carries the note too, and it is
/// held apart only so that the constant above stays a copy of the order the
/// documentation prints.
const ALSO_UNSPECIFIED: &str = "to_radians";

/// The types whose inherent methods those are. A qualified call names one.
const FLOAT_TYPES: [&str; 2] = ["f64", "f32"];

#[test]
fn no_source_in_the_numeric_core_calls_a_function_of_unspecified_precision() {
    let core = numeric_core();
    let mut offences = Vec::new();

    for path in rust_sources(&core) {
        let text = read(&path);
        for (number, raw) in text.lines().enumerate() {
            let line = strip_line_comment(raw);
            for name in refused_names() {
                for form in [format!(".{name}("), format!("::{name}(")] {
                    if !line.contains(&form) {
                        continue;
                    }
                    // A qualified call is only this project's business where the
                    // path names a float. `libm::sin(` and `math::sin(` are the
                    // two routes that are supposed to exist.
                    if form.starts_with("::") && !names_a_float_type(line, &form) {
                        continue;
                    }
                    offences.push(format!(
                        "{}:{}: {}",
                        relative(&core, &path),
                        number + 1,
                        raw.trim()
                    ));
                }
            }
        }
    }

    assert!(
        offences.is_empty(),
        "the numeric core calls a floating-point function whose precision the \
         standard library does not specify, at {offences:#?}. The documentation \
         permits that result to differ from one invocation to the next inside one \
         execution, which is what docs/decisions/0009-determinism.md promises will \
         not happen. Route the call through crates/einschlag/src/math.rs, or, if \
         the function is not there yet, add it there and nowhere else."
    );
}

/// The reader is only worth anything if it is looking at files. A change that
/// moved the core would otherwise leave this test walking an empty directory and
/// reporting no offences, which reads exactly like a clean tree.
#[test]
fn the_reader_actually_found_the_core() {
    let core = numeric_core();
    let sources = rust_sources(&core);
    assert!(
        !sources.is_empty(),
        "no Rust source under {}, so the check above would pass on nothing",
        core.display()
    );
    let names: Vec<String> = sources.iter().map(|path| relative(&core, path)).collect();
    assert!(
        names.iter().any(|name| name == "src/math.rs"),
        "the one file allowed to reach a math implementation is not among the \
         files this check reads: {names:?}"
    );
}

/// The offence the check above is written against, spelled as a person would
/// mistype it, so that the matcher is known to recognise the shape rather than
/// only the shapes that happen not to appear in the tree.
#[test]
fn the_matcher_recognises_the_mistake_it_exists_for() {
    for line in [
        "    let angle = ratio.asin();",
        "let d = y.atan2(x);",
        "        let radians = degrees.to_radians();",
        "let p = f64::powi(base, 2);",
    ] {
        assert!(
            refused_names().any(|name| {
                line.contains(&format!(".{name}("))
                    || (line.contains(&format!("::{name}("))
                        && names_a_float_type(line, &format!("::{name}(")))
            }),
            "the matcher does not recognise {line:?}"
        );
    }

    for line in [
        "    let length = squared.sqrt();",
        "    let scaled = base.mul_add(factor, offset);",
        "    libm::sin(x)",
        "    math::asin(ratio)",
        "    // the ellipse relation is an .asin( and it lives in math.rs",
    ] {
        let stripped = strip_line_comment(line);
        assert!(
            !refused_names().any(|name| {
                stripped.contains(&format!(".{name}("))
                    || (stripped.contains(&format!("::{name}("))
                        && names_a_float_type(stripped, &format!("::{name}(")))
            }),
            "the matcher refuses {line:?}, which it should not"
        );
    }
}

fn refused_names() -> impl Iterator<Item = &'static str> {
    UNSPECIFIED_PRECISION
        .into_iter()
        .chain(std::iter::once(ALSO_UNSPECIFIED))
}

/// Whether the path immediately before a qualified call names a float type.
///
/// `f64::sin(x)` is the platform's. `libm::sin(x)` and `math::sin(x)` are not,
/// and telling them apart is the whole of this function.
fn names_a_float_type(line: &str, form: &str) -> bool {
    line.match_indices(form).any(|(at, _)| {
        let before = &line[..at];
        FLOAT_TYPES.iter().any(|float| before.ends_with(float))
    })
}

/// Everything up to a line comment, where the comment is not inside a string.
///
/// A comment is prose and a reader writing `x.sin()` in one is describing the
/// mistake rather than making it. The quote test is the same one
/// `dependency_budget.rs` uses: a quotation mark before the marker means the
/// marker may be inside a string, and this reader leaves the line whole rather
/// than truncating it, which errs towards refusing.
fn strip_line_comment(line: &str) -> &str {
    match line.find("//") {
        Some(at) if !line[..at].contains('"') => &line[..at],
        _ => line,
    }
}

/// The crate the geometry and the uncertainty live in.
fn numeric_core() -> PathBuf {
    let core = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    assert!(
        core.join("Cargo.toml").is_file(),
        "no manifest at {}",
        core.display()
    );
    core
}

/// Every Rust source under a crate's `src`, which is the code that ships.
///
/// The tests beside it are not walked. A test may construct the platform call in
/// order to measure it, and `transcendental_stability.rs` does exactly that.
fn rust_sources(crate_root: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let mut pending = vec![crate_root.join("src")];

    while let Some(directory) = pending.pop() {
        let entries = fs::read_dir(&directory)
            .unwrap_or_else(|why| panic!("cannot read {}: {why}", directory.display()));
        for entry in entries {
            let path = entry.expect("the directory entry is readable").path();
            if path.is_dir() {
                pending.push(path);
            } else if path.extension().is_some_and(|suffix| suffix == "rs") {
                found.push(path);
            }
        }
    }
    found.sort();
    found
}

fn relative(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .expect("every walked path is under the crate root")
        .to_string_lossy()
        .replace('\\', "/")
}

fn read(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|why| panic!("cannot read {}: {why}", path.display()))
}
