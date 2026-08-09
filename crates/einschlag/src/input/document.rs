//! The layer between TOML text and the shapes this project's input format uses.
//!
//! It answers one question: what did the operator write, and on which line. It
//! answers nothing about whether what they wrote means anything, which is
//! `super`'s work.
//!
//! **The grammar is not implemented here.** `toml_parser` lexes and parses, and
//! what this file does is organise the events it emits into the four containers
//! the format has: the keys before the first header, the `[scene]` table, the
//! `[[scene.surface]]` and `[[scene.obstacle]]` lists, and the `[[hole]]` list.
//! `docs/decisions/0007-input-format.md` says in as many words that a
//! hand-written parser is not the way around a missing library, and that is the
//! line this file stays on the right side of: the bytes are read by the
//! dependency and the shape is read here.
//!
//! **Every form this format does not use is refused rather than ignored.** A
//! header this file does not know, a dotted key, a date, a value written across
//! more than one form of nesting than the format has: each one is a refusal
//! carrying the line it was written on. The failure that rule exists against is
//! a field read into the wrong slot, which produces a reconstruction that looks
//! entirely ordinary.

use toml_parser::Source;
use toml_parser::decoder::{IntegerRadix, ScalarKind};
use toml_parser::parser::{Event, EventKind, parse_document};

/// Something read out of the file, with the line it was written on.
///
/// The line is what a refusal message is worth to an operator holding a file of
/// two hundred lines, so nothing here is carried without one.
#[derive(Debug, Clone, PartialEq)]
pub(super) struct At<T> {
    /// What was read.
    pub(super) value: T,
    /// The line it starts on, counting from one.
    pub(super) line: usize,
}

/// One value, in the forms this format writes.
///
/// There is no date and no time, because no key in this format is one, and a
/// date arriving where a number belongs is refused at the point it is read
/// rather than carried as a value nothing downstream knows what to do with.
#[derive(Debug, Clone, PartialEq)]
pub(super) enum Node {
    /// A quoted string.
    Text(String),
    /// A whole number.
    Integer(i64),
    /// A number with a fractional part.
    Float(f64),
    /// `true` or `false`.
    Boolean(bool),
    /// `[...]`, whose elements may be of any form including further arrays.
    Array(Vec<At<Node>>),
    /// `{ ... }`, an inline table.
    Table(Table),
}

impl Node {
    /// What this form is called in a message, so that a refusal can say what was
    /// found as well as what was wanted.
    pub(super) fn kind(&self) -> &'static str {
        match self {
            Self::Text(_) => "a string",
            Self::Integer(_) => "a whole number",
            Self::Float(_) => "a number",
            Self::Boolean(_) => "true or false",
            Self::Array(_) => "an array",
            Self::Table(_) => "a table in braces",
        }
    }
}

/// One `key = value` pair.
#[derive(Debug, Clone, PartialEq)]
pub(super) struct Entry {
    /// The key as it was written, decoded.
    pub(super) key: String,
    /// The line the key is on.
    pub(super) line: usize,
    /// What was written after the equals sign.
    pub(super) node: At<Node>,
}

/// A set of `key = value` pairs: a header's table, an inline table, or the keys
/// standing before the first header.
///
/// The entries are held in the order they were written, and a repeated key is
/// kept rather than overwritten, so that [`Table::repeated`] can name it. A map
/// that silently kept the last value would turn a duplicated key into a value
/// nobody chose.
#[derive(Debug, Clone, PartialEq, Default)]
pub(super) struct Table {
    /// The line the header or the opening brace is on, and zero for the keys
    /// before the first header, which have no line of their own.
    pub(super) line: usize,
    /// The pairs, in the order written.
    pub(super) entries: Vec<Entry>,
}

impl Table {
    /// The first entry under `key`, or nothing.
    pub(super) fn find(&self, key: &str) -> Option<&Entry> {
        self.entries.iter().find(|entry| entry.key == key)
    }

    /// Every entry whose key was already written earlier in this table.
    pub(super) fn repeated(&self) -> Vec<&Entry> {
        let mut seen: Vec<&str> = Vec::new();
        let mut again = Vec::new();
        for entry in &self.entries {
            if seen.contains(&entry.key.as_str()) {
                again.push(entry);
            } else {
                seen.push(&entry.key);
            }
        }
        again
    }

    /// Every entry whose key is not in `known`.
    pub(super) fn unknown(&self, known: &[&str]) -> Vec<&Entry> {
        self.entries
            .iter()
            .filter(|entry| !known.contains(&entry.key.as_str()))
            .collect()
    }
}

/// Why the text could not be organised into the containers this format has.
///
/// These are refusals about the shape of the file rather than about what it
/// says, and `super::Fault` carries each one on to the operator with the line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum Fault {
    /// The dependency reported the text as not being TOML, or a value in it as
    /// not being the form its own syntax announced.
    Syntax {
        /// What was reported, as one sentence.
        detail: String,
    },
    /// A header this format does not have.
    SectionNotKnown {
        /// The header as it was written, without its brackets.
        header: String,
    },
    /// A header this format writes once, written twice.
    SectionRepeated {
        /// The header as it was written, without its brackets.
        header: String,
    },
    /// A list header written with one pair of brackets, or a single table
    /// written with two. The two forms mean different things in TOML and the
    /// wrong one puts every key that follows it somewhere nobody intended.
    SectionBracketedWrongly {
        /// The header as it was written, without its brackets.
        header: String,
        /// How this format writes it.
        wanted: &'static str,
    },
    /// A key written with a dot in it. This format has no dotted key, and one
    /// arriving here would be read as a key nothing knows, further from the
    /// mistake than this.
    KeyIsDotted,
    /// A date or a time, which no key in this format is.
    ValueIsADate,
    /// A number the reader could not turn into one, including a whole number
    /// written in a base this format does not use.
    NotANumber {
        /// The text that was written.
        found: String,
    },
    /// An event order the reader does not know. It refuses rather than
    /// returning nothing, because a form nobody taught it would otherwise be
    /// read as an empty file.
    NotUnderstood,
}

/// The file, organised into the containers this format has.
#[derive(Debug, Clone, PartialEq, Default)]
pub(super) struct Document {
    /// The keys standing before the first header.
    pub(super) top: Table,
    /// The `[scene]` table, where there is one.
    pub(super) scene: Option<Table>,
    /// One table per `[[scene.surface]]`.
    pub(super) surfaces: Vec<Table>,
    /// One table per `[[scene.obstacle]]`.
    pub(super) obstacles: Vec<Table>,
    /// One table per `[[hole]]`.
    pub(super) holes: Vec<Table>,
}

/// Where the keys being read are going.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Target {
    Top,
    Scene,
    Surface,
    Obstacle,
    Hole,
}

/// Read `text` into the containers this format has, or say what stopped it.
///
/// Every refusal it can find is returned rather than the first, because an
/// operator fixing a file one refusal per run is an operator who stops reading
/// the messages.
pub(super) fn read(text: &str) -> Result<Document, Vec<At<Fault>>> {
    let lines = Lines::of(text);
    let source = Source::new(text);
    let tokens = source.lex().into_vec();

    let mut events = Vec::new();
    let mut syntax = Vec::new();
    parse_document(
        &tokens,
        &mut |event: Event| events.push(event),
        &mut |error: toml_parser::ParseError| syntax.push(error),
    );

    let mut reader = Reader {
        source: &source,
        lines: &lines,
        events: &events,
        at: 0,
        faults: Vec::new(),
        document: Document::default(),
    };
    for error in &syntax {
        let line = lines.of_offset(error.unexpected().map_or(0, |span| span.start()));
        reader.faults.push(At {
            value: Fault::Syntax {
                detail: error.description().to_owned(),
            },
            line,
        });
    }

    reader.read_document();

    if reader.faults.is_empty() {
        Ok(reader.document)
    } else {
        Err(reader.faults)
    }
}

/// Byte offsets into line numbers.
///
/// Built once for the whole file rather than counting newlines per refusal,
/// which is quadratic on a file that has many.
struct Lines {
    /// The byte offset each line starts at, in order, starting with zero.
    starts: Vec<usize>,
}

impl Lines {
    fn of(text: &str) -> Self {
        let mut starts = vec![0];
        for (offset, byte) in text.bytes().enumerate() {
            if byte == b'\n' {
                starts.push(offset + 1);
            }
        }
        Self { starts }
    }

    /// The line an offset falls on, counting from one.
    fn of_offset(&self, offset: usize) -> usize {
        match self.starts.binary_search(&offset) {
            Ok(index) => index + 1,
            Err(index) => index,
        }
    }
}

/// The walk over the event stream.
struct Reader<'i> {
    source: &'i Source<'i>,
    lines: &'i Lines,
    events: &'i [Event],
    at: usize,
    faults: Vec<At<Fault>>,
    document: Document,
}

impl Reader<'_> {
    /// The whole file: headers and the pairs under each one.
    fn read_document(&mut self) {
        let mut target = Target::Top;
        while let Some(event) = self.peek() {
            match event.kind() {
                EventKind::Whitespace | EventKind::Comment | EventKind::Newline => {
                    self.at += 1;
                }
                EventKind::Error => {
                    let line = self.line_of(event);
                    self.refuse(line, Fault::NotUnderstood);
                    self.at += 1;
                }
                EventKind::StdTableOpen | EventKind::ArrayTableOpen => {
                    let doubled = event.kind() == EventKind::ArrayTableOpen;
                    if let Some(next) = self.read_header(doubled) {
                        target = next;
                    }
                }
                EventKind::SimpleKey => {
                    if let Some(entry) = self.read_pair() {
                        self.place(target, entry);
                    }
                }
                _ => {
                    let line = self.line_of(event);
                    self.refuse(line, Fault::NotUnderstood);
                    self.at += 1;
                }
            }
        }
    }

    /// A header, and where the keys after it are going.
    ///
    /// Returns nothing where the header was refused, and the walk then carries
    /// on filling the container it was already filling. That keeps the pairs
    /// under a bad header from being read into the previous one, because the
    /// header itself is already a refusal and the file will not be accepted.
    fn read_header(&mut self, doubled: bool) -> Option<Target> {
        let opening = *self.peek()?;
        let line = self.line_of(&opening);
        self.at += 1;

        let mut parts: Vec<String> = Vec::new();
        let mut dotted = false;
        while let Some(event) = self.peek() {
            match event.kind() {
                EventKind::SimpleKey => {
                    let event = *event;
                    parts.push(self.decode_key(&event));
                    self.at += 1;
                }
                EventKind::KeySep => {
                    dotted = true;
                    self.at += 1;
                }
                EventKind::Whitespace => self.at += 1,
                EventKind::StdTableClose | EventKind::ArrayTableClose => {
                    self.at += 1;
                    break;
                }
                _ => {
                    self.refuse(line, Fault::NotUnderstood);
                    self.at += 1;
                    return None;
                }
            }
        }
        let _ = dotted;

        let header = parts.join(".");
        let known: &[(&str, bool, &str)] = &[
            ("scene", false, "[scene]"),
            ("scene.surface", true, "[[scene.surface]]"),
            ("scene.obstacle", true, "[[scene.obstacle]]"),
            ("hole", true, "[[hole]]"),
        ];
        let Some(&(_, wants_doubled, written)) =
            known.iter().find(|(name, _, _)| *name == header.as_str())
        else {
            self.refuse(line, Fault::SectionNotKnown { header });
            return None;
        };
        if wants_doubled != doubled {
            self.refuse(
                line,
                Fault::SectionBracketedWrongly {
                    header,
                    wanted: written,
                },
            );
            return None;
        }

        let table = Table {
            line,
            entries: Vec::new(),
        };
        match header.as_str() {
            "scene" => {
                if self.document.scene.is_some() {
                    self.refuse(line, Fault::SectionRepeated { header });
                    return None;
                }
                self.document.scene = Some(table);
                Some(Target::Scene)
            }
            "scene.surface" => {
                self.document.surfaces.push(table);
                Some(Target::Surface)
            }
            "scene.obstacle" => {
                self.document.obstacles.push(table);
                Some(Target::Obstacle)
            }
            _ => {
                self.document.holes.push(table);
                Some(Target::Hole)
            }
        }
    }

    /// One `key = value` pair standing on its own line.
    fn read_pair(&mut self) -> Option<Entry> {
        let key_event = *self.peek()?;
        let line = self.line_of(&key_event);
        let key = self.decode_key(&key_event);
        self.at += 1;

        while let Some(event) = self.peek() {
            match event.kind() {
                EventKind::Whitespace => self.at += 1,
                EventKind::KeySep => {
                    self.refuse(line, Fault::KeyIsDotted);
                    self.skip_to_end_of_line();
                    return None;
                }
                EventKind::KeyValSep => {
                    self.at += 1;
                    break;
                }
                _ => {
                    self.refuse(line, Fault::NotUnderstood);
                    self.skip_to_end_of_line();
                    return None;
                }
            }
        }

        let node = self.read_value()?;
        Some(Entry { key, line, node })
    }

    /// One value, in any of the forms this format writes.
    fn read_value(&mut self) -> Option<At<Node>> {
        self.skip_trivia();
        let event = *self.peek()?;
        let line = self.line_of(&event);
        match event.kind() {
            EventKind::Scalar => {
                self.at += 1;
                let node = self.decode_scalar(&event)?;
                Some(At { value: node, line })
            }
            EventKind::ArrayOpen => {
                self.at += 1;
                let mut elements = Vec::new();
                loop {
                    self.skip_trivia();
                    let next = *self.peek()?;
                    match next.kind() {
                        EventKind::ArrayClose => {
                            self.at += 1;
                            break;
                        }
                        EventKind::ValueSep => {
                            self.at += 1;
                        }
                        _ => {
                            let element = self.read_value()?;
                            elements.push(element);
                        }
                    }
                }
                Some(At {
                    value: Node::Array(elements),
                    line,
                })
            }
            EventKind::InlineTableOpen => {
                self.at += 1;
                let mut table = Table {
                    line,
                    entries: Vec::new(),
                };
                loop {
                    self.skip_trivia();
                    let next = *self.peek()?;
                    match next.kind() {
                        EventKind::InlineTableClose => {
                            self.at += 1;
                            break;
                        }
                        EventKind::ValueSep => {
                            self.at += 1;
                        }
                        EventKind::SimpleKey => {
                            let entry = self.read_pair()?;
                            table.entries.push(entry);
                        }
                        _ => {
                            let at = self.line_of(&next);
                            self.refuse(at, Fault::NotUnderstood);
                            self.at += 1;
                            return None;
                        }
                    }
                }
                Some(At {
                    value: Node::Table(table),
                    line,
                })
            }
            _ => {
                self.refuse(line, Fault::NotUnderstood);
                self.at += 1;
                None
            }
        }
    }

    /// One scalar, refusing the forms this format has no key for.
    fn decode_scalar(&mut self, event: &Event) -> Option<Node> {
        let line = self.line_of(event);
        let Some(raw) = self.source.get(event) else {
            self.refuse(line, Fault::NotUnderstood);
            return None;
        };
        let written = raw.as_str().to_owned();
        let mut decoded = String::new();
        let mut errors: Vec<toml_parser::ParseError> = Vec::new();
        let kind = raw.decode_scalar(&mut decoded, &mut errors);
        for error in &errors {
            self.refuse(
                line,
                Fault::Syntax {
                    detail: error.description().to_owned(),
                },
            );
        }
        if !errors.is_empty() {
            return None;
        }

        match kind {
            ScalarKind::String => Some(Node::Text(decoded)),
            ScalarKind::Boolean(state) => Some(Node::Boolean(state)),
            ScalarKind::DateTime => {
                self.refuse(line, Fault::ValueIsADate);
                None
            }
            ScalarKind::Float => match decoded.parse::<f64>() {
                Ok(number) => Some(Node::Float(number)),
                Err(_) => {
                    self.refuse(line, Fault::NotANumber { found: written });
                    None
                }
            },
            ScalarKind::Integer(IntegerRadix::Dec) => match decoded.parse::<i64>() {
                Ok(number) => Some(Node::Integer(number)),
                Err(_) => {
                    self.refuse(line, Fault::NotANumber { found: written });
                    None
                }
            },
            ScalarKind::Integer(_) => {
                self.refuse(line, Fault::NotANumber { found: written });
                None
            }
        }
    }

    /// One key, decoded out of whatever quoting it was written with.
    fn decode_key(&mut self, event: &Event) -> String {
        let line = self.line_of(event);
        let Some(raw) = self.source.get(event) else {
            self.refuse(line, Fault::NotUnderstood);
            return String::new();
        };
        let mut decoded = String::new();
        let mut errors: Vec<toml_parser::ParseError> = Vec::new();
        raw.decode_key(&mut decoded, &mut errors);
        for error in &errors {
            self.refuse(
                line,
                Fault::Syntax {
                    detail: error.description().to_owned(),
                },
            );
        }
        decoded
    }

    /// Put a pair in the container the last header opened.
    fn place(&mut self, target: Target, entry: Entry) {
        let table = match target {
            Target::Top => &mut self.document.top,
            Target::Scene => match self.document.scene.as_mut() {
                Some(table) => table,
                None => &mut self.document.top,
            },
            Target::Surface => match self.document.surfaces.last_mut() {
                Some(table) => table,
                None => &mut self.document.top,
            },
            Target::Obstacle => match self.document.obstacles.last_mut() {
                Some(table) => table,
                None => &mut self.document.top,
            },
            Target::Hole => match self.document.holes.last_mut() {
                Some(table) => table,
                None => &mut self.document.top,
            },
        };
        table.entries.push(entry);
    }

    fn peek(&self) -> Option<&Event> {
        self.events.get(self.at)
    }

    fn skip_trivia(&mut self) {
        while let Some(event) = self.peek() {
            match event.kind() {
                EventKind::Whitespace | EventKind::Comment | EventKind::Newline => self.at += 1,
                _ => break,
            }
        }
    }

    /// Step past everything up to and including the next newline, so that one
    /// refused line does not turn into a refusal for every line after it.
    fn skip_to_end_of_line(&mut self) {
        while let Some(event) = self.peek().copied() {
            self.at += 1;
            if event.kind() == EventKind::Newline {
                break;
            }
        }
    }

    fn line_of(&self, event: &Event) -> usize {
        self.lines.of_offset(event.span().start())
    }

    fn refuse(&mut self, line: usize, fault: Fault) {
        self.faults.push(At { value: fault, line });
    }
}

#[cfg(test)]
mod tests {
    use super::{Fault, Node, read};

    #[test]
    fn a_pair_carries_the_line_it_was_written_on() {
        let document = read("format_version = 1\n\n[scene]\nname = \"a\"\n").expect("reads");
        let version = document
            .top
            .find("format_version")
            .expect("the key is there");
        assert_eq!(version.line, 1);
        assert_eq!(version.node.value, Node::Integer(1));
        let scene = document.scene.expect("the scene table is there");
        assert_eq!(scene.find("name").expect("the key is there").line, 4);
    }

    #[test]
    fn an_inline_table_and_an_array_come_back_in_the_shape_they_were_written() {
        let document = read("[scene]\nextent = { x = [0.0, 1.5], unit = \"m\" }\n").expect("reads");
        let scene = document.scene.expect("the scene table is there");
        let Node::Table(extent) = &scene.find("extent").expect("the key is there").node.value
        else {
            panic!("the extent did not come back as a table");
        };
        let Node::Array(x) = &extent.find("x").expect("x is there").node.value else {
            panic!("x did not come back as an array");
        };
        assert_eq!(x.len(), 2);
        assert_eq!(x[1].value, Node::Float(1.5));
    }

    #[test]
    fn each_list_header_opens_a_new_table_and_the_scene_opens_one_only_once() {
        let text = "[scene]\n[[hole]]\nid = \"A1\"\n[[hole]]\nid = \"A2\"\n";
        let document = read(text).expect("reads");
        assert_eq!(document.holes.len(), 2);
        assert_eq!(
            document.holes[1]
                .find("id")
                .expect("id is there")
                .node
                .value,
            Node::Text("A2".to_owned())
        );

        let faults = read("[scene]\n[scene]\n").expect_err("a second scene table is refused");
        assert!(
            matches!(faults[0].value, Fault::SectionRepeated { .. }),
            "{faults:?}"
        );
    }

    #[test]
    fn a_header_this_format_does_not_have_is_refused_rather_than_skipped() {
        let faults = read("[scene]\n[shooter]\nx = 1\n").expect_err("refused");
        assert!(
            faults
                .iter()
                .any(|fault| matches!(&fault.value, Fault::SectionNotKnown { header } if header == "shooter")),
            "{faults:?}"
        );
    }

    #[test]
    fn a_list_written_with_one_bracket_is_refused_naming_the_form_this_format_uses() {
        let faults = read("[hole]\nid = \"A1\"\n").expect_err("refused");
        assert!(
            matches!(
                &faults[0].value,
                Fault::SectionBracketedWrongly { wanted, .. } if *wanted == "[[hole]]"
            ),
            "{faults:?}"
        );
    }

    #[test]
    fn a_dotted_key_is_refused_where_it_is_written() {
        let faults = read("[scene]\nextent.x = 1\n").expect_err("refused");
        assert!(
            faults
                .iter()
                .any(|fault| fault.value == Fault::KeyIsDotted && fault.line == 2),
            "{faults:?}"
        );
    }

    #[test]
    fn a_date_is_refused_because_no_key_in_this_format_is_one() {
        let faults = read("[scene]\nname = 2026-08-09\n").expect_err("refused");
        assert!(
            faults
                .iter()
                .any(|fault| fault.value == Fault::ValueIsADate),
            "{faults:?}"
        );
    }

    #[test]
    fn text_that_is_not_toml_is_refused_with_the_line_the_dependency_reported() {
        let faults = read("format_version = \n").expect_err("refused");
        assert!(
            faults
                .iter()
                .any(|fault| matches!(fault.value, Fault::Syntax { .. })),
            "{faults:?}"
        );
    }

    #[test]
    fn a_repeated_key_is_kept_rather_than_overwritten() {
        let document = read("[scene]\nname = \"a\"\nname = \"b\"\n").expect("reads");
        let scene = document.scene.expect("the scene table is there");
        assert_eq!(scene.entries.len(), 2);
        let repeated = scene.repeated();
        assert_eq!(repeated.len(), 1);
        assert_eq!(repeated[0].line, 3);
    }

    #[test]
    fn a_key_the_caller_does_not_know_is_reported_rather_than_dropped() {
        let document = read("[scene]\nname = \"a\"\nnickname = \"b\"\n").expect("reads");
        let scene = document.scene.expect("the scene table is there");
        let unknown = scene.unknown(&["name"]);
        assert_eq!(unknown.len(), 1);
        assert_eq!(unknown[0].key, "nickname");
    }
}
