# Ship-quality GPUI

The last chapter is about what separates a working view from one you'd ship:
it's usable without a mouse or a screen, it's tested, it stays fast with lots
of data, and its pieces can be reused.

**Accessibility.** GPUI describes the window to screen readers as an
accessibility tree: one node per control, with a role, a name and a state. GPUI
Base's unstyled controls, such as `Switch`, supply the role, state and keyboard
behavior. The name has to come from you, with `accessibility_label`. After
lesson 38, try the switch with your platform's screen reader: the automated
check can't hear what it announces.

**Testing.** A `#[gpui::test]` drives a view like a user would, on a simulated
clock that only moves when the test says so:

```rust
cx.simulate_click(button.center(), Modifiers::default());
cx.executor().advance_clock(Duration::from_secs(1));
cx.run_until_parked();
```

You've been reading checks like this since chapter 01. In lesson 39 you write
one.

**Large collections.** A virtual list only builds the rows that are on screen.
It knows every row's size up front and asks you for just the visible range.

**Reusable components.** A reusable control takes its value as input and
reports changes through a callback. The view that uses it owns the value, so two
copies of the control never share state by accident.

Last comes [quiz 6](../quizzes/README.md), the capstone, which draws on the
whole course.

## Further information

- [Accessibility](https://gpui-kit.com/docs/accessibility/)
- [Testing](https://gpui-kit.com/docs/test/)
- [Virtual list](https://gpui-kit.com/component/virtual-list)
- [RenderOnce: state belongs outside the component value](https://gpui-kit.com/docs/render-once/#state-belongs-outside-the-component-value)
