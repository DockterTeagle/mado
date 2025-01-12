use comrak::nodes::NodeValue;
use linkify::LinkFinder;
use miette::Result;

use crate::{violation::Violation, Document};

use super::RuleLike;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct MD034;

impl MD034 {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl RuleLike for MD034 {
    #[inline]
    fn name(&self) -> &'static str {
        "MD034"
    }

    #[inline]
    fn description(&self) -> &'static str {
        "Bare URL used"
    }

    #[inline]
    fn tags(&self) -> &'static [&'static str] {
        &["links", "url"]
    }

    #[inline]
    fn aliases(&self) -> &'static [&'static str] {
        &["no-bare-urls"]
    }

    // TODO: Use safe casting
    #[inline]
    #[allow(clippy::cast_possible_wrap)]
    fn check(&self, doc: &Document) -> Result<Vec<Violation>> {
        let mut violations = vec![];
        let finder = LinkFinder::new();

        for node in doc.ast.descendants() {
            if let NodeValue::Text(text) = &node.data.borrow().value {
                for link in finder.links(text) {
                    if let Some(parent) = node.parent() {
                        if let NodeValue::Link(_) = parent.data.borrow().value {
                            continue;
                        }
                    }

                    // NOTE: link.start and link.end start from 0
                    let mut position = node.data.borrow().sourcepos;
                    position.end = position.start.column_add(link.end() as isize);
                    position.start = position.start.column_add(link.start() as isize);

                    let violation = self.to_violation(doc.path.clone(), position);
                    violations.push(violation);
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
        let text = "For more information, see http://www.example.com/.".to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let doc = Document::new(&arena, path.clone(), text).unwrap();
        let rule = MD034::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![rule.to_violation(path, Sourcepos::from((1, 27, 1, 50)))];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_with_brackets() {
        let text = "For more information, see <http://www.example.com/>.".to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let doc = Document::new(&arena, path, text).unwrap();
        let rule = MD034::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_with_link() {
        let text = "For more information, see [http://www.example.com/](http://www.example.com/)."
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let doc = Document::new(&arena, path, text).unwrap();
        let rule = MD034::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_with_code() {
        let text = "For more information, see `http://www.example.com/`.".to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let doc = Document::new(&arena, path, text).unwrap();
        let rule = MD034::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }
}
