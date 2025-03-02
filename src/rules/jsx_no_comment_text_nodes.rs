// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.

use super::{Context, LintRule};
use crate::handler::{Handler, Traverse};
use crate::tags::Tags;
use crate::{tags, Program};
use deno_ast::view::JSXText;
use deno_ast::SourceRanged;

#[derive(Debug)]
pub struct JSXNoCommentTextNodes;

const CODE: &str = "jsx-no-comment-text-nodes";

impl LintRule for JSXNoCommentTextNodes {
  fn tags(&self) -> Tags {
    &[tags::RECOMMENDED, tags::REACT, tags::JSX, tags::FRESH]
  }

  fn code(&self) -> &'static str {
    CODE
  }

  fn lint_program_with_ast_view(
    &self,
    context: &mut Context,
    program: Program,
  ) {
    JSXNoCommentTextNodesHandler.traverse(program, context);
  }
}

const MESSAGE: &str =
  "Comments inside children should be placed inside curly braces";

struct JSXNoCommentTextNodesHandler;

impl Handler for JSXNoCommentTextNodesHandler {
  fn jsx_text(&mut self, node: &JSXText, ctx: &mut Context) {
    let value = &node.inner.value;
    if value.starts_with("//") || value.starts_with("/*") {
      ctx.add_diagnostic(node.range(), CODE, MESSAGE);
    }
  }
}

// most tests are taken from ESlint, commenting those
// requiring code path support
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn jsx_no_comment_text_nodes_valid() {
    assert_lint_ok! {
      JSXNoCommentTextNodes,
      filename: "file:///foo.jsx",
      // non derived classes.
      r#"<div>{/* comment */}</div>"#,
    };
  }

  #[test]
  fn jsx_no_comment_text_nodes_invalid() {
    assert_lint_err! {
      JSXNoCommentTextNodes,
      filename: "file:///foo.jsx",
      "<div>// comment</div>": [
        {
          col: 5,
          message: MESSAGE,
        }
      ],
      r#"<div>/* comment */</div>"#: [
        {
          col: 5,
          message: MESSAGE,
        }
      ],
    };
  }
}
