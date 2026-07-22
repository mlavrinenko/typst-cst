#import "@local/mindtape:0.2.0": *

#show: task.with(
  title: "parse comments on the parse plane",
  status: done(2026, 7, 22),
  tags: ("api",),
)

= Summary

Add `parse_comments`, a sibling of the existing `parse_headings`, extracting
every line and block comment from Typst source on the parse plane
(`typst_syntax`, `SyntaxKind::LineComment` / `BlockComment`) — no eval, no
`World`, no IO, no regex. MindTape will consume this to surface comments
alongside headings.

= Scope

- Public API:
  ```rust
  pub enum CommentKind { Line, Block }
  pub struct Comment { pub kind: CommentKind, pub text: String, pub line: usize, pub range: Range<usize> }
  #[must_use] pub fn parse_comments(text: &str) -> Vec<Comment>
  ```
- `text` is the comment's inner text with delimiters stripped (`//`, `/*`/`*/`)
  and trimmed.
- `range` is the byte range of the whole comment token including delimiters;
  `line` is the 1-based line of the comment's start.
- Results in document order.
- `Comment`/`CommentKind` derive `Debug`, `Clone`, `PartialEq`, `Eq`, and carry
  `#[cfg_attr(feature = "serde", derive(serde::Serialize))]`, matching
  `Heading`.
- Doc example on `parse_comments` in the crate-level style; broaden the
  crate-level docs and the `Cargo.toml` `description` to cover comments too.
- Tests: line comment; block comment; multi-line block comment reports its
  start line; `//` inside a string literal is not reported; `//` inside a raw
  block (single and triple backtick) is not reported; a comment inside a code
  block / a function call's argument list is reported; empty source yields an
  empty vec; a file with comments before and after a heading keeps document
  order.
- Ships as part of the 0.2.0 release.
