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

/// The source text of the section `heading` opens.
fn section_text<'a>(src: &'a str, heading: &super::Heading) -> &'a str {
    &src[heading.section.clone()]
}

#[test]
fn test_section_ends_at_a_sibling() {
    let src = "= A\nalpha\n= B\nbeta\n";
    let headings = parse_headings(src);
    assert_eq!(section_text(src, &headings[0]), "\nalpha\n");
}

#[test]
fn test_section_keeps_deeper_headings_inside() {
    let src = "== A\nalpha\n=== Sub\ninner\n== B\nbeta\n";
    let headings = parse_headings(src);
    assert_eq!(section_text(src, &headings[0]), "\nalpha\n=== Sub\ninner\n");
    assert_eq!(section_text(src, &headings[1]), "\ninner\n");
}

#[test]
fn test_section_ends_at_a_shallower_heading() {
    let src = "= A\n== B\nbeta\n= C\n";
    let headings = parse_headings(src);
    assert_eq!(section_text(src, &headings[1]), "\nbeta\n");
}

#[test]
fn test_last_section_runs_to_the_end() {
    let src = "= A\n== B\nbeta";
    let headings = parse_headings(src);
    assert_eq!(section_text(src, &headings[0]), "\n== B\nbeta");
    assert_eq!(section_text(src, &headings[1]), "\nbeta");
}

#[test]
fn test_section_spans_a_depth_jump() {
    let src = "= A\n=== Deep\nx\n== B\n";
    let headings = parse_headings(src);
    assert_eq!(section_text(src, &headings[0]), "\n=== Deep\nx\n== B\n");
    assert_eq!(section_text(src, &headings[1]), "\nx\n");
}

#[test]
fn test_tree_keeps_sections() {
    let src = "= A\n== B\n";
    let roots = tree(parse_headings(src));
    assert_eq!(roots[0].heading.section, 3..src.len());
    assert_eq!(roots[0].children[0].heading.section, 8..src.len());
}
