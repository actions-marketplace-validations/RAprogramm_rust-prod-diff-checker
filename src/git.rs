pub mod diff_parser;
pub mod hunk;

pub use diff_parser::{FileDiff, parse_diff};
pub use hunk::{Hunk, HunkLine, LineType};
