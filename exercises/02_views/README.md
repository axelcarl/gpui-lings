# Views & layout · 04–06

A view owns state; its `Render` implementation produces elements. A parent
container controls how its children are arranged, while a child entity can own
an independent interaction.

- [04 · Flex](../../exercises/02_views/views1.rs): direction determines the main axis.
- [05 · Child entity](../../exercises/02_views/views2.rs): a persistent view owns the toggle state.
- [06 · Spacing](../../exercises/02_views/views3.rs): gap belongs to the parent; padding belongs inside an element.

These checks render real GPUI elements and measure bounds or send clicks.
An exercise's source file contains its instructions and expected result.

Reference: [GPUI elements and views](https://github.com/zed-industries/zed/blob/main/crates/gpui/README.md).
