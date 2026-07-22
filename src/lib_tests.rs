#![allow(clippy::unwrap_used, clippy::indexing_slicing)]

use super::{CommentKind, parse_comments, parse_headings, tree};

#[test]
fn test_parse_headings_depth_and_text() {
    let headings = parse_headings("= Top\n== Sub one\n=== Deep\n");
    assert_eq!(headings.len(), 3);
    assert_eq!(headings[0].depth, 1);
    assert_eq!(headings[0].text, "Top");
    assert_eq!(headings[1].depth, 2);
    assert_eq!(headings[1].text, "Sub one");
    assert_eq!(headings[2].depth, 3);
    assert_eq!(headings[2].text, "Deep");
}

#[test]
fn test_parse_headings_line_numbers() {
    let headings = parse_headings("= One\n\n== Two\n");
    assert_eq!(headings[0].line, 1);
    assert_eq!(headings[1].line, 3);
}

#[test]
fn test_parse_headings_skips_code_and_body() {
    let src = "#import \"x\": *\n#show: f\n\n== Summary\nprose here\n=== Detail\n";
    let headings = parse_headings(src);
    assert_eq!(headings.len(), 2);
    assert_eq!(headings[0].text, "Summary");
    assert_eq!(headings[0].depth, 2);
    assert_eq!(headings[1].text, "Detail");
}

#[test]
fn test_no_headings() {
    assert!(parse_headings("just prose, no headings\n").is_empty());
}

#[test]
fn test_tree_nesting() {
    let roots = tree(parse_headings("= A\n== B\n== C\n=== D\n= E\n"));
    assert_eq!(roots.len(), 2);
    assert_eq!(roots[0].heading.text, "A");
    assert_eq!(roots[0].children.len(), 2);
    assert_eq!(roots[0].children[1].heading.text, "C");
    assert_eq!(roots[0].children[1].children[0].heading.text, "D");
    assert_eq!(roots[1].heading.text, "E");
    assert!(roots[1].children.is_empty());
}

#[test]
fn test_tree_depth_jump() {
    let roots = tree(parse_headings("== Summary\n==== Deep\n"));
    assert_eq!(roots.len(), 1);
    assert_eq!(roots[0].children.len(), 1);
    assert_eq!(roots[0].children[0].heading.text, "Deep");
}

#[test]
fn test_tree_flat_siblings() {
    let roots = tree(parse_headings("= A\n= B\n= C\n"));
    assert_eq!(roots.len(), 3);
    assert!(roots.iter().all(|r| r.children.is_empty()));
}

#[test]
fn test_parse_comments_line() {
    let comments = parse_comments("// a line comment\n");
    assert_eq!(comments.len(), 1);
    assert_eq!(comments[0].kind, CommentKind::Line);
    assert_eq!(comments[0].text, "a line comment");
    assert_eq!(comments[0].range, 0..17);
}

#[test]
fn test_parse_comments_block() {
    let comments = parse_comments("/* a block comment */\n");
    assert_eq!(comments.len(), 1);
    assert_eq!(comments[0].kind, CommentKind::Block);
    assert_eq!(comments[0].text, "a block comment");
}

#[test]
fn test_parse_comments_multiline_block_reports_start_line() {
    let src = "= Heading\n/* line two\n   line three\n   line four */\n";
    let comments = parse_comments(src);
    assert_eq!(comments.len(), 1);
    assert_eq!(comments[0].line, 2);
    assert_eq!(comments[0].text, "line two\n   line three\n   line four");
}

#[test]
fn test_parse_comments_ignores_string_literal() {
    let src = "#let s = \"http://example.com // not a comment\"\n";
    assert!(parse_comments(src).is_empty());
}

#[test]
fn test_parse_comments_ignores_raw_block() {
    let src = "```\n// not a comment either\n```\n`// not a comment`\n";
    assert!(parse_comments(src).is_empty());
}

#[test]
fn test_parse_comments_inside_call_args() {
    let src = "#f(1, // arg comment\n   2)\n";
    let comments = parse_comments(src);
    assert_eq!(comments.len(), 1);
    assert_eq!(comments[0].kind, CommentKind::Line);
    assert_eq!(comments[0].text, "arg comment");
}

#[test]
fn test_parse_comments_empty_source() {
    assert!(parse_comments("").is_empty());
}

#[test]
fn test_parse_comments_keeps_document_order_around_heading() {
    let src = "// before\n= Heading\n// after\n";
    let comments = parse_comments(src);
    assert_eq!(comments.len(), 2);
    assert_eq!(comments[0].text, "before");
    assert_eq!(comments[1].text, "after");
    assert!(comments[0].range.start < comments[1].range.start);
}
