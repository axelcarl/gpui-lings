# Lifetimes & async

An entity lives as long as something holds a strong `Entity<T>` handle to it. A
`WeakEntity<T>`, from `entity.downgrade()`, points at the same entity without
keeping it alive. Call `upgrade()` to get a strong handle back. It returns
`None` once the entity is gone, so treat that as a normal case, not an error.

Weak handles come up whenever code runs *later*, when the view it belongs to
may already be gone. This chapter has two kinds of later.

**Later in this update.** While GPUI runs your handler, your view is mutably
borrowed, so you can't update it again through its handle until the handler
returns. `cx.defer(...)` runs a closure right after the current update ends.
Take `cx.weak_entity()` to reach the view from there.

**Later in time.** `cx.spawn` returns a `Task`, and the work only runs for as
long as you keep that task. Dropping it cancels the work. Store it in your view
to tie the work to the view, or call `.detach()` when it should finish no
matter what.

```rust
self.refresh = Some(cx.spawn(async move |this, cx| {
    cx.background_executor().timer(Duration::from_secs(1)).await;
    // `this` is a WeakEntity: the view may have closed while we waited.
    let _ = this.update(cx, |this, cx| {
        this.refreshed = true;
        cx.notify();
    });
}));
```

Inside the async block, `cx` is an `AsyncApp`, which you can hold across
`.await`. To change your view, go back in through `this.update(cx, ...)`. The
block runs on the UI thread, so it must never block: wait with the executor's
`timer`, not `std::thread::sleep`.

The checks in this chapter don't wait for real time. They move GPUI's simulated
clock forward instead, so they finish instantly. After this chapter comes
[quiz 2](../quizzes/README.md), which mixes handles and tasks with `observe`
and `subscribe` from chapter 03.

## Further information

- [Entity: use a WeakEntity for back references](https://gpui-kit.com/docs/entity/#use-a-weakentity-for-back-references-and-callbacks)
- [Entity: lifecycle](https://gpui-kit.com/docs/entity/#lifecycle)
- [Context: defer until the current update ends](https://gpui-kit.com/docs/context/#defer-until-the-current-update-ends)
- [Task](https://gpui-kit.com/docs/task/)
