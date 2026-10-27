#import "@local/mindtape:0.2.0": *

#show: task.with(
  title: "expose each heading's section span",
  status: proposed(2026, 10, 28),
)

= Summary

A heading's `range` covers the heading alone, so every caller that wants the
text a heading opens recomputes the section from the flat list: from the end
of the heading to the next heading of the same or smaller depth, or the end
of the source. MindTape's content gate and speconaut's section splitter each
carry their own copy. Compute it once here.

= Scope

- `Heading` gains `pub section: Range<usize>`, the byte range of the section's
  content after the heading itself, up to the next heading of the same or
  smaller depth or the end of the source. Nested subsections fall inside it.
- `parse_headings` fills it; `tree` keeps it as is.
- Tests: a sibling ends the section; a deeper heading stays inside it; a
  shallower heading ends it; the last section runs to the end of the source;
  a depth jump (`=` then `===`) nests.
- Ships as 0.3.0: a new public field breaks struct literals.
