use core::cmp::Ordering;
use core::fmt::{Display, Error, Formatter, Result};

use colored::Colorize as _;

use crate::Violation;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Concise<'a> {
    violation: &'a Violation,
}

impl<'a> Concise<'a> {
    pub fn new(violation: &'a Violation) -> Self {
        Self { violation }
    }

    #[cfg(test)]
    pub fn violation(&self) -> &'a Violation {
        self.violation
    }
}

impl Display for Concise<'_> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let path = self.violation.path().to_str().ok_or(Error)?;
        write!(
            f,
            "{}{}{}{}{}{} {} {}",
            path.bold(),
            ":".blue(),
            self.violation.position().start.line,
            ":".blue(),
            self.violation.position().start.column,
            ":".blue(),
            self.violation.name().red().bold(),
            self.violation.description()
        )
    }
}

impl PartialOrd for Concise<'_> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Concise<'_> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.violation.cmp(other.violation)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use comrak::nodes::Sourcepos;
    use pretty_assertions::assert_eq;

    use crate::rule::Metadata;

    use super::*;

    const METADATA: Metadata = Metadata {
        name: "name",
        description: "description",
        aliases: &["alias"],
        tags: &["tags"],
    };

    #[test]
    fn display_fmt() {
        let path = Path::new("file.md").to_path_buf();
        let position = Sourcepos::from((0, 1, 3, 5));
        let violation = Violation::new(path, &METADATA, position);
        let actual = Concise::new(&violation).to_string();
        let expected = "\u{1b}[1mfile.md\u{1b}[0m\u{1b}[34m:\u{1b}[0m0\u{1b}[34m:\u{1b}[0m1\u{1b}[34m:\u{1b}[0m \u{1b}[1;31mname\u{1b}[0m description";
        assert_eq!(actual, expected);
    }
}
