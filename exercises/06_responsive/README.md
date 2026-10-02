# Responsive views · 16–19, then quiz 1

Window size is an input to `Render`. Choose a layout from the current bounds,
then let a parent element arrange its children. Re-rendering after a resize
should choose the new layout without changing the cards themselves.

- [16 · Responsive cards](../../exercises/06_responsive/responsive1.rs):
  keep a row in a wide window and stack the same cards in a narrow one.
- [17 · Scrollable content](../../exercises/06_responsive/responsive2.rs):
  let a fixed-height list scroll under its heading, and move it with a
  ScrollHandle.
- [18 · Disabled control](../../exercises/06_responsive/responsive3.rs):
  keep a disabled control inert for the mouse and the keyboard, not just dimmed.
- [19 · Pointer gesture](../../exercises/06_responsive/responsive4.rs):
  drag a value and end the gesture even when release happens outside.
- [20 · Quiz 1: compact inspector](../../exercises/quizzes/quiz1.rs):
  combine responsive layout, scrolling, and keyboard selection in a new view.

These lessons build on [flex direction and spacing](../02_views/README.md).
Their source files contain the tasks and checks. The first check measures cards
before and after resizing; the second sends a scroll gesture, presses Jump to
last and verifies that the list moves while its heading stays put. The quiz
reports four bugs as
symptoms instead of marking them in the source. Its fixes reuse breakpoints,
scrolling, disabled states, and where key events go (lessons 12–13).

Reference: [GPUI views and elements](https://github.com/zed-industries/zed/blob/main/crates/gpui/README.md).
