# typst-cst

[![CI](https://github.com/mlavrinenko/typst-cst/actions/workflows/ci.yml/badge.svg)](https://github.com/mlavrinenko/typst-cst/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/typst-cst.svg)](https://crates.io/crates/typst-cst)
[![License: MIT](https://img.shields.io/crates/l/typst-cst.svg)](LICENSE-MIT)

Parse-plane structure extraction for Typst: heading tree with source spans, no eval, no World, no IO

## Install

```bash
cargo add typst-cst
```

## Usage

```rust
use typst_cst::{parse_headings, tree};

let src = "= Top\n== Sub\n";
let flat = parse_headings(src);
assert_eq!(flat.len(), 2);
assert_eq!(flat[0].depth, 1);
assert_eq!(flat[1].text, "Sub");

let roots = tree(flat);
assert_eq!(roots[0].children.len(), 1);
```

Enable the `serde` feature to derive `Serialize` on `Heading`/`HeadingNode`.

## Development

Prerequisites: [Nix](https://nixos.org/) with flakes enabled.

```bash
direnv allow         # or: nix develop

just check           # fmt + clippy + tests + file-size + drift check
just build
just test
just cover           # code coverage (70% minimum)
just fmt             # format code
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for coding conventions.

## License

MIT
