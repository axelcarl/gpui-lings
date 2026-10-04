# Focus & dispatch

Chapter 04 moved focus to one element and sent it one key or action at a time.
Real interfaces have several places that take focus, and several handlers that
could answer the same key. This chapter is about where input ends up.

**Focus in overlays.** A dialog or menu can have several focusable regions. Keep
a handle for each one, and for whatever opened the overlay. When the overlay
closes, its elements leave the tree, so give focus back to the opener on every
way out: a Close button, the Escape key, or choosing an item. Otherwise the
keyboard is left pointing at nothing.

**Action propagation.** An action starts at the focused element and travels up
through its parents. The first handler it reaches *consumes* it. A handler that
decides not to deal with the action calls `cx.propagate()`, and the action
continues upward:

```rust
fn save(&mut self, _: &Save, _: &mut Window, cx: &mut Context<Self>) {
    if self.read_only {
        cx.propagate(); // Let a parent handle it instead.
        return;
    }
    self.write_to_disk(cx);
}
```

After this chapter comes [quiz 4](../quizzes/README.md), a keyboard-driven
command menu.

## Further information

- [Focus: trap and restore focus in an overlay](https://gpui-kit.com/docs/focus/#trap-and-restore-focus-in-an-overlay)
- [Action: handler order and propagation](https://gpui-kit.com/docs/action/#handler-order-and-propagation)
- [GPUI's guide to key dispatch](https://github.com/zed-industries/zed/blob/main/crates/gpui/docs/key_dispatch.md)
