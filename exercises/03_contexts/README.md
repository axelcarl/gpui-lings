# Contexts & handlers

Almost every GPUI function takes a `cx`. It's a *context*: your way into the
app's state. `cx` and `ctx` are just variable names, so look at the type to see
what you can do with it:

| Type | What it gives you |
| --- | --- |
| `App` | The whole application: every entity, window and global |
| `Context<T>` | Everything `App` has, plus the identity of the entity `T` being updated |
| `Window` | One window: its size, focus and input |
| `Entity<T>` | A handle to state that GPUI owns |
| `Subscription` | A connection that stays active for as long as you keep it |

GPUI owns the state of every entity. You reach it through a handle and a
context, and you tell GPUI when it has changed:

```rust
let count = counter.read(cx).count;   // borrow the state
counter.update(cx, |counter, cx| {    // change it
    counter.count = count + 1;
    cx.notify();                      // and say it changed
});
```

An element's callbacks only get the event, the window and the app, not your
view. `cx.listener(...)` wraps a closure or method that takes `&mut Self` and
`Context<Self>`, so your handler can reach the view's fields.

There are two ways for one entity to hear about another. `cx.observe` runs a
callback whenever the other entity calls `notify`: it tells you *that* something
changed. `cx.subscribe` runs a callback for every typed event the other entity
`emit`s: it tells you *what* happened. Both return a `Subscription`, and the
callback only runs while you keep it. Callbacks run after the current update
has finished, never in the middle of it.

After this chapter comes [quiz 1](../quizzes/README.md). There's nothing to fix
in it: you write a small view from a description.

## Further information

- [Ownership and data flow in GPUI](https://zed.dev/blog/gpui-ownership): Zed's
  own walkthrough of entities, contexts, `observe` and events. If you read one
  thing alongside this course, read this.
- [Context](https://gpui-kit.com/docs/context/)
- [Entity](https://gpui-kit.com/docs/entity/)
- [Event](https://gpui-kit.com/docs/event/)
- [GPUI's guide to contexts](https://github.com/zed-industries/zed/blob/main/crates/gpui/docs/contexts.md)
