# Keyboard input

Keyboard input goes to the element that has *focus*. A `FocusHandle` names a
place that can have focus, `track_focus` attaches it to an element, and
`window.focus(&handle, cx)` moves focus there. Creating a handle doesn't focus
anything by itself.

The focused element hears each key through `on_key_down`. Keys it doesn't
handle carry on up through its parents:

```rust
div()
    .track_focus(&self.focus)
    .on_key_down(cx.listener(|this, event: &KeyDownEvent, _, cx| {
        if event.keystroke.key == "enter" {
            this.submit(cx);
        }
    }))
```

A key handler is fine for one element and one key. Commands that a button, a
menu and a shortcut should all trigger are better as *actions*. An action is a
named command, a `KeyBinding` turns a keystroke into one, and `on_action` says
what an element does when the action reaches it:

```rust
actions!(editor, [Save]);

cx.bind_keys([KeyBinding::new("ctrl-s", Save, Some("Editor"))]);

div()
    .key_context("Editor")      // where the binding applies
    .track_focus(&self.focus)   // where keys arrive
    .on_action(cx.listener(Self::save))
```

The last argument to `KeyBinding::new` is a key context. With `None`, the
binding matches wherever focus is. With a name, it only matches while focus is
inside an element with that `key_context`, which lets the same key do
different things in different parts of your app.

The chapter goes one step at a time: focus (13), a key handler (14), an action
(15), and a key context (16).

A practical note: every time you save, the playground opens a fresh preview
window with nothing focused, and your keystrokes keep going to whichever app was
active before. Click into the preview before you try a key.

## Further information

- [Focus](https://gpui-kit.com/docs/focus/)
- [Event: pointer and keyboard input are also events](https://gpui-kit.com/docs/event/#pointer-and-keyboard-input-are-also-events)
- [Action](https://gpui-kit.com/docs/action/)
- [KeyBinding](https://gpui-kit.com/docs/keybinding/)
- [GPUI's guide to key dispatch](https://github.com/zed-industries/zed/blob/main/crates/gpui/docs/key_dispatch.md)
