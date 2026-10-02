# Lifetimes & async · 14–15

GPUI handles make ownership explicit. A strong `Entity<T>` keeps its value
alive; a `WeakEntity<T>` does not. Upgrading a weak handle can fail once the
last strong owner disappears, so handle the missing entity as normal control
flow.

- [14 · Weak handles](../../exercises/05_lifetimes/lifetimes1.rs): inspect before and after the owner is released.
- [15 · Tasks](../../exercises/05_lifetimes/lifetimes2.rs): retain a task while loading, then cancel it by dropping its handle.

`Context::spawn` gives the future a weak entity and an `AsyncApp`. After an
`await`, use the handle's update closure to regain synchronous access to state.
A foreground task must yield while waiting; blocking work belongs on the
background executor. This exercise uses an async timer, so no network is needed.

Dropping a `Task` cancels it. Retaining it gives the view control over its
lifetime; `detach` intentionally gives up that cancellation handle. Replacing
a stored task cancels the previous load, and releasing the view drops its task.

The check advances a simulated clock and verifies both completion and
cancellation. Real error/retry flows and background computation remain future
exercises.

Reference: [GPUI asynchronous work](https://github.com/zed-industries/zed/blob/main/crates/gpui/README.md).
