# Responsive views · 16–20

Window size is an input to `Render`. Choose a layout from the current bounds,
then let a parent element arrange its children. Re-rendering after a resize
should choose the new layout without changing the cards themselves.

- [16 · Responsive cards](../../playground/src/exercises/views/responsive.rs):
  keep a row in a wide window and stack the same cards in a narrow one.
- [17 · Scrollable content](../../playground/src/exercises/views/scrolling.rs):
  keep a heading visible while a fixed-height list scrolls to its last row.
- [18 · Control states](../../playground/src/exercises/views/states.rs):
  show hover, focus, selected, and disabled states; block disabled activation.
- [19 · Pointer gesture](../../playground/src/exercises/views/drag.rs):
  drag a value and end the gesture even when release happens outside.
- [20 · Compact inspector](../../playground/src/exercises/views/inspector.rs):
  combine responsive layout, scrolling, and keyboard selection in a new view.

These lessons build on [flex direction and spacing](../02_views/README.md).
Their source files contain the tasks and checks. The first check measures cards
before and after resizing; the second sends a scroll gesture and verifies that
the list moves while its heading stays put. The checkpoint reports four bugs as
symptoms instead of marking them in the source. Its fixes reuse breakpoints,
scrolling, disabled states, and where key events go (lessons 12–13).

Reference: [GPUI views and elements](https://github.com/zed-industries/zed/blob/main/crates/gpui/README.md).
