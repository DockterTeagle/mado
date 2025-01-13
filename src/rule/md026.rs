use comrak::nodes::NodeValue;
use miette::Result;

use crate::{violation::Violation, Document};

use super::{Metadata, RuleLike};

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct MD026 {
    punctuation: String,
}

impl MD026 {
    const METADATA: Metadata = Metadata {
        name: "MD026",
        description: "Trailing punctuation in header",
        tags: &["headers"],
        aliases: &["no-trailing-punctuation"],
    };

    pub const DEFAULT_PUNCTUATION: &str = ".,;:!?";

    #[inline]
    #[must_use]
    pub fn new(punctuation: String) -> Self {
        Self { punctuation }
    }
}

impl Default for MD026 {
    #[inline]
    fn default() -> Self {
        Self {
            punctuation: Self::DEFAULT_PUNCTUATION.to_owned(),
        }
    }
}

impl RuleLike for MD026 {
    #[inline]
    fn metadata(&self) -> &'static Metadata {
        &Self::METADATA
    }

    #[inline]
    fn check(&self, doc: &Document) -> Result<Vec<Violation>> {
        let mut violations = vec![];

        for node in doc.ast.children() {
            if let NodeValue::Heading(_) = node.data.borrow().value {
                if let Some(child) = node.last_child() {
                    if let NodeValue::Text(text) = &child.data.borrow().value {
                        if let Some(last_char) = text.chars().last() {
                            if self.punctuation.contains(last_char) {
                                let position = node.data.borrow().sourcepos;
                                let violation = self.to_violation(doc.path.clone(), position);
                                violations.push(violation);
                            }
                        }
                    }
                }
            }
        }

        Ok(violations)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use comrak::{nodes::Sourcepos, Arena};
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn check_errors() {
        let text = "# This is a header.".to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let doc = Document::new(&arena, path.clone(), text).unwrap();
        let rule = MD026::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![rule.to_violation(path.clone(), Sourcepos::from((1, 1, 1, 19)))];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_errors_with_link() {
        let text = "# [This is a header](http://example.com).".to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let doc = Document::new(&arena, path.clone(), text).unwrap();
        let rule = MD026::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![rule.to_violation(path.clone(), Sourcepos::from((1, 1, 1, 41)))];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_errors_with_code() {
        let text = "# `This is a header`.".to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let doc = Document::new(&arena, path.clone(), text).unwrap();
        let rule = MD026::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![rule.to_violation(path.clone(), Sourcepos::from((1, 1, 1, 21)))];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_errors_with_emph() {
        let text = "# *This is a header*.".to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let doc = Document::new(&arena, path.clone(), text).unwrap();
        let rule = MD026::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![rule.to_violation(path.clone(), Sourcepos::from((1, 1, 1, 21)))];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors() {
        let text = "# This is a header".to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let doc = Document::new(&arena, path.clone(), text).unwrap();
        let rule = MD026::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_with_link() {
        let text = "# [This is a header.](http://example.com)".to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let doc = Document::new(&arena, path, text).unwrap();
        let rule = MD026::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_with_code() {
        let text = "# `This is a header.`".to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let doc = Document::new(&arena, path, text).unwrap();
        let rule = MD026::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_with_emph() {
        let text = "# *This is a header.*".to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let doc = Document::new(&arena, path, text).unwrap();
        let rule = MD026::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }
}
