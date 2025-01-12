use comrak::nodes::NodeValue;
use miette::Result;

use crate::{violation::Violation, Document};

use super::RuleLike;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct MD039;

impl MD039 {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl RuleLike for MD039 {
    #[inline]
    fn name(&self) -> &'static str {
        "MD039"
    }

    #[inline]
    fn description(&self) -> &'static str {
        "Spaces inside link text"
    }

    #[inline]
    fn tags(&self) -> &'static [&'static str] {
        &["whitespace", "links"]
    }

    #[inline]
    fn aliases(&self) -> &'static [&'static str] {
        &["no-space-in-links"]
    }

    #[inline]
    fn check(&self, doc: &Document) -> Result<Vec<Violation>> {
        let mut violations = vec![];

        for node in doc.ast.descendants() {
            if let NodeValue::Link(_) = node.data.borrow().value {
                if let Some(text_node) = node.first_child() {
                    if let NodeValue::Text(text) = &text_node.data.borrow().value {
                        if text.trim_start() != text {
                            let position = text_node.data.borrow().sourcepos;
                            let violation = self.to_violation(doc.path.clone(), position);
                            violations.push(violation);
                            continue;
                        }
                    }
                }

                if let Some(text_node) = node.last_child() {
                    if let NodeValue::Text(text) = &text_node.data.borrow().value {
                        if text.trim_end() != text {
                            let position = text_node.data.borrow().sourcepos;
                            let violation = self.to_violation(doc.path.clone(), position);
                            violations.push(violation);
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
        let text = "[ a link ](http://www.example.com/)
[a link ](http://www.example.com/)
[ a link](http://www.example.com/)"
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let doc = Document::new(&arena, path.clone(), text).unwrap();
        let rule = MD039::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![
            rule.to_violation(path.clone(), Sourcepos::from((1, 2, 1, 9))),
            rule.to_violation(path.clone(), Sourcepos::from((2, 2, 2, 8))),
            rule.to_violation(path, Sourcepos::from((3, 2, 3, 8))),
        ];
        assert_eq!(actual, expected);
    }

    // NOTE: These results may differ from mdl
    #[test]
    fn check_errors_code() {
        let text = "[ a `link` ](http://www.example.com/)
[ `link` ](http://www.example.com)
[`link` ](http://www.example.com)
[ `link`](http://www.example.com)"
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let doc = Document::new(&arena, path.clone(), text).unwrap();
        let rule = MD039::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![
            rule.to_violation(path.clone(), Sourcepos::from((1, 2, 1, 4))),
            rule.to_violation(path.clone(), Sourcepos::from((2, 2, 2, 2))),
            rule.to_violation(path.clone(), Sourcepos::from((3, 8, 3, 8))),
            rule.to_violation(path, Sourcepos::from((4, 2, 4, 2))),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors() {
        let text = "[a link](http://www.example.com/)".to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let doc = Document::new(&arena, path, text).unwrap();
        let rule = MD039::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_url() {
        let text = "[a link]( http://www.example.com/ )".to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let doc = Document::new(&arena, path, text).unwrap();
        let rule = MD039::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_bracket() {
        let text = "< http://www.example.com/ >".to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let doc = Document::new(&arena, path, text).unwrap();
        let rule = MD039::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_code() {
        let text = "[a `link`](http://www.example.com)
[`link`](http://www.example.com)"
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let doc = Document::new(&arena, path, text).unwrap();
        let rule = MD039::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }
}
