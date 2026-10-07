#import "@local/mindtape:0.2.0": *

#show: task.with(
  title: "finish a half-failed release on its tag",
  status: proposed(2026, 10, 7),
)

= Summary

Sibling crates had release runs go red at `cargo publish` on a version
published by hand before its tag was pushed, with no way to finish the release
on the same tag. Here, the v0.2.0 tag names a commit that was never pushed to
origin/main, because `just release` tags whatever it is given after
`just check`.

= Scope

- Adopt cratemplate's hardened lib release workflow: `cargo publish` is
  skipped for a version crates.io has, and the workflow re-runs on an existing
  tag through `workflow_dispatch`.
- Adopt its `just release X.Y.Z [--dry-run]` preflight, which refuses a main
  that differs from origin/main, keeping `just check` as the gate; document it
  in CONTRIBUTING.md.
- Push main, so the v0.2.0 commit is on origin/main.
