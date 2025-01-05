use std::path::Path;

use comrak::nodes::{NodeValue, Sourcepos};
use miette::Result;
use scraper::Html;

use crate::{violation::Violation, Document};

use super::RuleLike;

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct MD033 {
    allowed_elements: Vec<String>,
}

impl MD033 {
    pub const DEFAULT_ALLOWED_ELEMENTS: Vec<String> = vec![];

    #[inline]
    #[must_use]
    pub fn new(allowed_elements: &[String]) -> Self {
        Self {
            allowed_elements: allowed_elements.to_vec(),
        }
    }

    fn check_html(
        &self,
        html: &str,
        path: &Path,
        position: &Sourcepos,
        violations: &mut Vec<Violation>,
    ) {
        // NOTE: `Html::parse_fragment` automatically adds `<html>` as the root element,
        //        so we need to check if `<html>` is allowed as a string
        if html.starts_with("<html") && !self.allowed_elements.contains(&"html".to_owned()) {
            let violation = self.to_violation(path.to_path_buf(), *position);
            violations.push(violation);
            return;
        }

        let fragment = Html::parse_fragment(html);
        for element in fragment.root_element().descendent_elements() {
            let name = element.value().name();
            let is_allowed = self.allowed_elements.contains(&name.to_owned());

            // NOTE: Skip <html> root
            if !is_allowed && name != "html" {
                let violation = self.to_violation(path.to_path_buf(), *position);
                violations.push(violation);
                break;
            }
        }
    }
}

impl Default for MD033 {
    #[inline]
    fn default() -> Self {
        Self {
            allowed_elements: Self::DEFAULT_ALLOWED_ELEMENTS,
        }
    }
}

impl RuleLike for MD033 {
    #[inline]
    fn name(&self) -> &'static str {
        "MD033"
    }

    #[inline]
    fn description(&self) -> &'static str {
        "Inline HTML"
    }

    #[inline]
    fn tags(&self) -> Vec<&'static str> {
        vec!["html"]
    }

    #[inline]
    fn aliases(&self) -> Vec<&'static str> {
        vec!["no-inline-html"]
    }

    #[inline]
    fn check(&self, doc: &Document) -> Result<Vec<Violation>> {
        let mut violations = vec![];

        for node in doc.ast.descendants() {
            let position = node.data.borrow().sourcepos;

            match &node.data.borrow().value {
                NodeValue::HtmlInline(html) => {
                    self.check_html(html, &doc.path, &position, &mut violations);
                }
                NodeValue::HtmlBlock(html) => {
                    // NOTE: Skip non-html elements (e.g. comments)
                    //       See https://spec.commonmark.org/0.31.2/#html-blocks
                    if (2..=5).contains(&html.block_type) {
                        continue;
                    }

                    self.check_html(&html.literal, &doc.path, &position, &mut violations);
                }
                _ => {}
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
        let text = "<h1>Inline HTML header</h1>".to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let doc = Document::new(&arena, path.clone(), text).unwrap();
        let rule = MD033::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![rule.to_violation(path, Sourcepos::from((1, 1, 1, 27)))];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_errors_nested() {
        let text = r##"<h1><a href="#">Inline HTML header</a></h1>"##.to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let doc = Document::new(&arena, path.clone(), text).unwrap();
        let rule = MD033::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![rule.to_violation(path, Sourcepos::from((1, 1, 1, 43)))];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_errors_with_allowed_tag() {
        let text = "<p>h1</p>".to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let doc = Document::new(&arena, path.clone(), text).unwrap();
        let rule = MD033::new(&["h1".to_owned()]);
        let actual = rule.check(&doc).unwrap();
        let expected = vec![rule.to_violation(path, Sourcepos::from((1, 1, 1, 9)))];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_errors_with_allowed_tag2() {
        let text = "<pre>text</pre>".to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let doc = Document::new(&arena, path.clone(), text).unwrap();
        let rule = MD033::new(&["p".to_owned()]);
        let actual = rule.check(&doc).unwrap();
        let expected = vec![rule.to_violation(path, Sourcepos::from((1, 1, 0, 0)))];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_errors_with_html() {
        let text = "<html>text</html>".to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let doc = Document::new(&arena, path.clone(), text).unwrap();
        let rule = MD033::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![rule.to_violation(path, Sourcepos::from((1, 1, 1, 17)))];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_errors_with_br() {
        let text = "Some text<br>
Some more text"
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let doc = Document::new(&arena, path.clone(), text).unwrap();
        let rule = MD033::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![rule.to_violation(path, Sourcepos::from((1, 10, 1, 13)))];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors() {
        let text = "# Markdown header".to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let doc = Document::new(&arena, path, text).unwrap();
        let rule = MD033::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_with_allowed_tag() {
        let text = "<h1>Inline HTML header</h1>".to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let doc = Document::new(&arena, path, text).unwrap();
        let rule = MD033::new(&["h1".to_owned()]);
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_with_comment() {
        let text = "<!-- html comment -->".to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let doc = Document::new(&arena, path, text).unwrap();
        let rule = MD033::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }
}
