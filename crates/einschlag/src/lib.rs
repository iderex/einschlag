//! The geometry and uncertainty core.
//!
//! Everything this project computes belongs here, and the front end reaches it
//! only through the items this file marks `pub`. That boundary is the compiler's
//! rather than a convention: a front end that reads the core's internals is a
//! front end the core cannot be replaced underneath, and milestones 4 to 7
//! assume it can.
//!
//! The core is empty today. What fills it is decided in `docs/decisions/` and
//! carried by milestones 4 and 5.

/// The name the tool reports itself as.
///
/// It lives here rather than in the front end so that a second front end, or a
/// script driving the tool, reports the same name as the first one.
pub const TOOL_NAME: &str = "einschlag";
