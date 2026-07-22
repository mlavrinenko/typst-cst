# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2026-07-22

### Added

- `parse_comments` and `Comment`/`CommentKind`: extract line (`//`) and block
  (`/* */`) comments from Typst source on the parse plane, alongside
  `parse_headings`. Same discipline — no eval, no `World`, no IO, no regex.

## [0.1.0] - 2026-07-05

### Added

- Initial release, extracted from the mindtape workspace: `parse_headings` and
  `tree` over `typst-syntax` — heading tree with source spans, no eval, no
  World, no IO. Optional `serde` feature.
