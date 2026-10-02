# Actions & focus · 12–13

Keyboard input follows focus. A `FocusHandle` identifies a place in that tree;
`track_focus` attaches it to an element. `Window::focus` moves focus there.
Simply creating a handle does not focus anything.

- [12 · Actions](../../exercises/04_interaction/interaction1.rs): route Ctrl-K through a scoped key context to a named action.
- [13 · Focus](../../exercises/04_interaction/interaction2.rs): move focus to a pad that receives X.

An action describes an operation. A key binding maps input to that operation,
and its context predicate controls where it applies. Direct key handlers are
useful for an element's local input; named actions let multiple input sources
invoke the same command. Keep global shortcuts deliberate.

Both exercises check that input stops affecting the view after focus leaves it.

Reference: [GPUI keyboard input](https://github.com/zed-industries/zed/blob/main/crates/gpui/README.md).
