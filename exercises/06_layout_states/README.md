# Layout & control states

`render` receives the `Window`, so a view can look at the window's size and
build a different tree for it. GPUI renders again after every resize, so the
choice is made fresh each time:

```rust
let width = window.bounds().size.width;
let label = if width < px(600.0) { "Compact" } else { "Wide" };
div().child(label)
```

When content doesn't fit, its overflow mode decides what happens.
`overflow_hidden()` clips it. `overflow_y_scroll()` clips it too, but lets the
user scroll. A scrolling element needs an `id`, so GPUI can remember its scroll
offset between frames. To scroll from code, keep a `ScrollHandle` in your view
and attach it with `track_scroll`.

This chapter also looks at the states a control goes through. Styling a
control as disabled, hovered or focused only changes how it *looks*; the view's
state decides what it *does*, for the mouse and the keyboard alike. A pointer
gesture has the same kind of trap: a drag can end over a different element than
the one it started on, so `on_mouse_up_out` has to clean up too.

After this chapter comes [quiz 3](../quizzes/README.md), which mixes these
lessons with focus and keys from chapter 04.

## Further information

- [Style: choose clipping, scrolling or positioning](https://gpui-kit.com/docs/style/#choose-clipping-scrolling-or-positioning)
- [Style: position and overflow](https://gpui-kit.com/docs/style/#position-and-overflow)
- [Window: geometry and scale](https://gpui-kit.com/docs/window/#geometry-and-scale)
- [Event: pointer and keyboard input are also events](https://gpui-kit.com/docs/event/#pointer-and-keyboard-input-are-also-events)
