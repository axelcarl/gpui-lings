# Contexts & handlers · 07–11

`cx` and `ctx` are variable names, not different types. Read the type to see
what an operation can access:

| Type | What it gives you |
| --- | --- |
| `App` | Application state and access to entities |
| `Context<T>` | App access plus the identity of the entity being updated |
| `Window` | Window-local state, input routing, layout, and focus |
| `Entity<T>` | A strong handle to persistent state owned by GPUI |
| `Subscription` | A connection that stays active while its handle lives |

An element callback receives the event, window, and app. `cx.listener` adapts
that callback so your method also receives `&mut Self` and `Context<Self>`.
Keep mutable borrows inside synchronous update closures; do not capture them
in callbacks or keep them across `await`.

1. [07 · Notify](../../exercises/03_contexts/contexts1.rs): state changes need notification.
2. [08 · Listener](../../exercises/03_contexts/contexts2.rs): connect a handler to its view.
3. [09 · Update](../../exercises/03_contexts/contexts3.rs): mutate the entity behind the handle.
4. [10 · Observe](../../exercises/03_contexts/contexts4.rs): react to another entity's notification and read its current value.
5. [11 · Subscribe](../../exercises/03_contexts/contexts5.rs): receive a typed event payload and retain the connection.

`notify` announces a changed entity. `emit` sends a specific event. Neither
replaces the other; choose according to what the receiver needs to know.
Observers run as effects after the current update, not inline during mutation.

Reference: [GPUI entities, contexts, and events](https://github.com/zed-industries/zed/blob/main/crates/gpui/README.md).
The exercises target the GPUI snapshot pinned by this repository's Cargo.lock.
