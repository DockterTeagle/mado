use comrak::nodes::{NodeHeading, NodeValue};
use miette::Result;

use crate::violation::Violation;
use crate::Document;

use super::{Metadata, RuleLike};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct MD020;

impl MD020 {
    const METADATA: Metadata = Metadata {
        name: "MD020",
        description: "No space inside hashes on closed atx style header",
        tags: &["headers", "atx_closed", "spaces"],
        aliases: &["no-missing-space-closed-atx"],
    };

    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl RuleLike for MD020 {
    #[inline]
    fn metadata(&self) -> &'static Metadata {
        &Self::METADATA
    }

    #[inline]
    fn check(&self, doc: &Document) -> Result<Vec<Violation>> {
        let mut violations = vec![];

        for node in doc.ast.children() {
            if let NodeValue::Paragraph = node.data.borrow().value {
                for child_node in node.children() {
                    if let NodeValue::Text(text) = &child_node.data.borrow().value {
                        let position = node.data.borrow().sourcepos;
                        if position.start.column == 1
                            && text.starts_with('#')
                            && text.ends_with('#')
                        {
                            let violation = self.to_violation(doc.path.clone(), position);
                            violations.push(violation);
                        }
                    }
                }
            }

            if let NodeValue::Heading(NodeHeading { setext: false, .. }) = node.data.borrow().value
            {
                if let Some(child_node) = node.last_child() {
                    if let NodeValue::Text(text) = &child_node.data.borrow().value {
                        if text.ends_with('#') {
                            let position = node.data.borrow().sourcepos;
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
        let text = "#Header 1#

## Header 2##

##Header 3 ##"
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let doc = Document::new(&arena, path.clone(), text).unwrap();
        let rule = MD020::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![
            rule.to_violation(path.clone(), Sourcepos::from((1, 1, 1, 10))),
            rule.to_violation(path.clone(), Sourcepos::from((3, 1, 3, 13))),
            rule.to_violation(path, Sourcepos::from((5, 1, 5, 13))),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors() {
        let text = "# Header 1 #

## Header 2 ##"
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let doc = Document::new(&arena, path, text).unwrap();
        let rule = MD020::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    // TODO: Support escaped hash
    //     #[test]
    //     fn check_no_errors_with_escaped_hash() {
    //         let text = "#Header 1\\#
    //
    // \\##Header 2##
    //
    // ## Header 3\\##
    //
    // \\## Header 4##
    //
    // ##Header 5 \\##
    //
    // \\##Header 6 ##"
    //             .to_owned();
    //         let path = Path::new("test.md").to_path_buf();
    //         let arena = Arena::new();
    //         let doc = Document::new(&arena, path, text).unwrap();
    //         let rule = MD020::default();
    //         let actual = rule.check(&doc).unwrap();
    //         let expected = vec![];
    //         assert_eq!(actual, expected);
    //     }

    #[test]
    fn check_no_errors_with_atx() {
        let text = "#Header 1

## Header 2

##Header 3"
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let doc = Document::new(&arena, path, text).unwrap();
        let rule = MD020::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_with_issue_number() {
        let text = "# Header 1 #

See [#4649](https://example.com) and [#4979](https://example.com) for details."
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let doc = Document::new(&arena, path, text).unwrap();
        let rule = MD020::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_with_list() {
        let text = "* #Header1#".to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let doc = Document::new(&arena, path, text).unwrap();
        let rule = MD020::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_with_code_block_comment() {
        let text = "```
#Header#
```"
        .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let doc = Document::new(&arena, path, text).unwrap();
        let rule = MD020::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }
}
