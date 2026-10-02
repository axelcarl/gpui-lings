# Application structure · 29–33

[29 · Start a standalone GPUI app](../../exercises/09_application/application1.rs)
connects the platform application, initialization, a first window, and its root
view. The playground preview lets you open the second window without leaving
the course. You can also run the isolated example:

```sh
cargo run --example lesson29
```

The starter example opens no window until you complete `open_workspace`. The
playground itself still launches. Use GPUI Kit's `application`, `init`, and
`open_window` helpers; the root still implements GPUI's `Render` trait.

Reference: [GPUI Kit application setup](https://gpui-kit.com/docs/installation/).

[30 · Share application state](../../exercises/09_application/application2.rs)
puts one spacing setting in a GPUI Global. One child changes it and another
reads it in `Render`. Keep the summary's global observer subscription alive so
both views repaint after each change.

[31 · Save and restore a setting](../../exercises/09_application/application3.rs)
writes a simple preference to an injected temporary path. Restore the saved
value and use a safe default for a missing or invalid file. The preview has
buttons to save, load, and write invalid data without touching real app
preferences.

[32 · Open a second window](../../exercises/09_application/application4.rs)
opens a detail view that shares an entity with the main view. Update from both
windows, then keep the close observer alive so the main view can clear its
handle and reopen a fresh detail window.

[33 · Follow appearance changes](../../exercises/09_application/application5.rs)
uses semantic tokens for light and dark surfaces. Its controls let you preview
both modes; the view also observes real window appearance changes.
