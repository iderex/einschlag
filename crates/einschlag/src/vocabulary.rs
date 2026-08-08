//! Phrases this tool may not put in an operator's hands with its name on it.
//!
//! `docs/decisions/0012-certainty-vocabulary.md` is where the list is argued and
//! where its provenance, its scope and its incompleteness are written down. This
//! file is the list itself, and it is the authority: the record points here
//! rather than restating the phrases, because a list in two places drifts.
//!
//! **This file is exempt from the scan it feeds**, together with the record and
//! the survey the phrases were read out of. It has to hold the literals to
//! refuse them. The exemption register is in
//! `crates/einschlag/tests/certainty_vocabulary.rs` and it fails in both
//! directions: an exempt file that no longer holds a phrase is stale, and an
//! exempt file that is not there is dangling.

/// One refused phrase and where it was read from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Phrase {
    /// Lower case, single-spaced. Matched against text normalised the same way.
    pub text: &'static str,
    /// The document the phrase was read out of, so that adding to this list is
    /// an argued change rather than somebody's preference.
    pub source: &'static str,
}

const PCAST: &str = "PCAST 2016, recommendations to the courts, quoted in docs/survey/challenges.md";
const NCFS: &str = "National Commission on Forensic Science, quoted in the same PCAST report";

/// Every phrase refused, with its source.
///
/// These are phrases and not single words, and that is the whole design rather
/// than a shortcut. PCAST names `minimal` and `negligible` as words that appear
/// in a claim about an error rate. Refusing the bare word would refuse
/// `profile = "minimal"` in a toolchain file, which is a true statement about a
/// compiler and has nothing to do with certainty. A check that fires on true
/// statements is a check somebody switches off.
pub const REFUSED: &[Phrase] = &[
    Phrase { text: "zero error rate", source: PCAST },
    Phrase { text: "negligible error rate", source: PCAST },
    Phrase { text: "minimal error rate", source: PCAST },
    Phrase { text: "microscopic error rate", source: PCAST },
    Phrase { text: "vanishingly small", source: PCAST },
    Phrase { text: "essentially zero", source: PCAST },
    Phrase { text: "100 percent certainty", source: PCAST },
    Phrase { text: "100% certainty", source: PCAST },
    Phrase { text: "reasonable degree of scientific certainty", source: PCAST },
    Phrase { text: "to the exclusion of all other sources", source: PCAST },
    Phrase { text: "practical impossibility", source: PCAST },
    Phrase { text: "reasonable degree of certainty", source: NCFS },
];

/// The first refused phrase in `text`, or `None`.
///
/// Case is ignored and runs of whitespace are collapsed, so a phrase broken
/// across two lines of a wrapped document is found. That is the ordinary way a
/// phrase hides from a naive search in this repository's own prose.
pub fn first_refused(text: &str) -> Option<&'static Phrase> {
    let normalised = normalise(text);
    REFUSED.iter().find(|phrase| normalised.contains(phrase.text))
}

/// Lower case with every run of whitespace collapsed to one space.
fn normalise(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut pending_space = false;
    for character in text.chars() {
        if character.is_whitespace() {
            pending_space = !out.is_empty();
            continue;
        }
        if pending_space {
            out.push(' ');
            pending_space = false;
        }
        out.extend(character.to_lowercase());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{first_refused, normalise, REFUSED};

    #[test]
    fn every_phrase_is_stored_in_the_form_the_matcher_compares_against() {
        for phrase in REFUSED {
            assert_eq!(
                phrase.text,
                normalise(phrase.text),
                "the phrase is not lower case and single-spaced, so it can never match"
            );
            assert!(!phrase.text.is_empty(), "an empty phrase matches everything");
            assert!(
                !phrase.source.is_empty(),
                "the phrase {:?} names no source, so adding to this list stops being an argued change",
                phrase.text
            );
        }
    }

    #[test]
    fn no_phrase_makes_another_unreachable() {
        for outer in REFUSED {
            for inner in REFUSED {
                if outer.text == inner.text {
                    continue;
                }
                assert!(
                    !outer.text.contains(inner.text),
                    "{:?} contains {:?}, so the second can never be the phrase reported",
                    outer.text,
                    inner.text
                );
            }
        }
    }

    #[test]
    fn a_phrase_is_found_whatever_case_it_is_written_in() {
        let found = first_refused("Reported To A Reasonable Degree Of Scientific Certainty.");
        assert_eq!(
            found.map(|phrase| phrase.text),
            Some("reasonable degree of scientific certainty")
        );
    }

    #[test]
    fn a_phrase_is_found_when_a_line_break_falls_inside_it() {
        let wrapped = "the result is stated to a reasonable degree of scientific\ncertainty, which is the sentence this refuses";
        assert!(
            first_refused(wrapped).is_some(),
            "a phrase wrapped across two lines was not found, which is how it hides in a document"
        );
    }

    #[test]
    fn a_bare_word_from_the_source_is_not_refused_on_its_own() {
        // PCAST names "minimal" and "negligible" inside a claim about an error
        // rate. The words on their own are ordinary English and one of them is
        // in this repository's toolchain file.
        assert!(first_refused("profile = \"minimal\"").is_none());
        assert!(first_refused("a negligible difference in build time").is_none());
    }

    #[test]
    fn ordinary_text_is_not_refused() {
        assert!(first_refused("the region contains the origin at the stated level").is_none());
        assert!(first_refused("").is_none());
    }
}
