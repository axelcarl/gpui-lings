# Foundations

GPUI draws your app's interface from a tree of *elements*. A *view* is a struct
that owns some state, and its `render` method builds the element tree that shows
that state. When the state changes, the view tells GPUI, and GPUI calls `render`
again.

```rust
struct Counter {
    count: u32,
}

impl Render for Counter {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("counter")
            .child(format!("Clicked {} times", self.count))
            .on_click(cx.listener(|this, _, _, cx| {
                this.count += 1;
                cx.notify();
            }))
    }
}
```

Lesson 01 is a warm-up: the playground calls a function, and you change what
it returns. From lesson 02 on, every exercise holds its own view like the one
above, with a small piece of it wrong. You'll write more and more of each view
yourself as the course goes on, and in quiz 1 you'll write a whole one.

## Further information

- [Getting Started: add a view](https://gpui-kit.com/docs/getting-started/#add-a-view)
- [Render](https://gpui-kit.com/docs/render/)
- [GPUI's own introduction](https://github.com/zed-industries/zed/blob/main/crates/gpui/README.md)
