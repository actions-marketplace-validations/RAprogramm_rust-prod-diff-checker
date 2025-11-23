pub mod formatter;
pub mod github;
pub mod json;

pub use formatter::{Formatter, format_output};
pub use github::GithubFormatter;
pub use json::JsonFormatter;
