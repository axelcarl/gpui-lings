# Application structure

So far, the playground has started the app and opened the window for every
lesson. This chapter is about the parts of an application around your views.

A GPUI program starts the application, sets up GPUI Kit, and opens its first
window with a root view:

```rust
fn main() {
    gpui_kit::application().run(|cx| {
        gpui_kit::init(cx);
        gpui_kit::open_window(WindowOptions::default(), cx, |_, cx| {
            cx.new(|_| Workspace::default())
        })
        .expect("could not open the main window");
    });
}
```

Lesson 33 has a complete program like this. Besides the button in the preview,
you can run it by itself with `cargo run --example application1`.

State that belongs to the whole app, such as a setting, can live in a `Global`.
Any context reads it with `cx.global::<T>()`, but reading doesn't subscribe: a
view that should redraw when it changes keeps a subscription from
`cx.observe_global::<T>(...)`. Several windows can also share one entity, each
observing it, so a change in one window shows up in all of them.

Settings should survive a restart, so the app saves them to a file. Reading
that file back can fail, and the app should fall back to a default instead of
crashing. Colors should follow the system's light or dark appearance: choose
them by role (background, foreground) from a palette, never by looks.

## Further information

- [Getting Started: create a project](https://gpui-kit.com/docs/getting-started/#create-a-project)
- [Window](https://gpui-kit.com/docs/window/)
- [Global](https://gpui-kit.com/docs/global/)
- [Multi Window](https://gpui-kit.com/docs/multi-window/)
- [Coding guide: theme and styling](https://gpui-kit.com/docs/coding-guides/#theme-and-styling)
- [The Rust book: recoverable errors with `Result`](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
