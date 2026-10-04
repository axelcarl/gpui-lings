# Views & layout

A parent element arranges its children. Turn on flexbox with `.flex()`, choose a
direction with `.flex_row()` or `.flex_col()`, and space the children out with
`.gap(...)`. If you know Tailwind CSS, GPUI's style methods will look familiar.

```rust
div()
    .flex()
    .flex_row()
    .gap(px(12.0))
    .child(div().p(px(8.0)).child("One"))
    .child(div().p(px(8.0)).child("Two"))
```

A view can contain other views, too. The parent creates the child once with
`cx.new(...)`, keeps the `Entity<Child>` handle it gets back, and passes that
handle to `.child(...)` in its own `render`. GPUI keeps the child's state from
one frame to the next, and the child handles its own clicks.

## Reading the checks

We're going a little out of order here: testing gets its own lesson near the
end (39), but every check renders real GPUI elements, so it helps to
know how to read one. A check opens a window without showing it, draws a frame
with `window.draw(cx)`, finds elements by their `debug_selector` name with
`debug_bounds`, and measures them or clicks them with `simulate_click`. It does
what you'd do in the preview, and then compares what it sees with what the
lesson asks for.

## Further information

- [Style: build a first layout](https://gpui-kit.com/docs/style/#build-a-first-layout)
- [Style: flexbox and grid](https://gpui-kit.com/docs/style/#flexbox-and-grid)
- [Style: space and size](https://gpui-kit.com/docs/style/#space-and-size)
- [Entity: data model or persistent view](https://gpui-kit.com/docs/entity/#data-model-or-persistent-view)
