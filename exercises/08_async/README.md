# Async data & failure paths

These lessons build on lesson 19. Work that takes a while, or a lot of CPU,
shouldn't run on the UI thread. GPUI has two executors for that:

- `cx.spawn(...)` runs on the UI thread. It can update views, but it must never
  block.
- `cx.background_executor().spawn(...)` runs on other threads. It can do heavy
  work, but it can't touch views.

So the usual shape is a background task that computes a result, awaited by a
foreground task that stores it in the view:

```rust
let work = cx.background_executor().spawn(async move { expensive_sum() });
self.task = Some(cx.spawn(async move |this, cx| {
    let sum = work.await;
    let _ = this.update(cx, |this, cx| {
        this.sum = Some(sum);
        cx.notify();
    });
}));
```

Requests can fail, and they can finish out of order. Keep the request's state
explicit (idle, loading, loaded, failed), and set it to loading as soon as a new
request starts, including on retry. When a result comes back, check that the
view still wants it before showing it.

These exercises use timers and canned answers instead of a network, so the
checks can move a simulated clock and get the same result every time. After
this chapter comes [quiz 5](../quizzes/README.md), a search box built on GPUI
Base's text input.

## Further information

- [Task: how execution moves between threads](https://gpui-kit.com/docs/task/#how-execution-moves-between-threads)
- [Task: move heavy work off the UI thread](https://gpui-kit.com/docs/task/#move-heavy-work-off-the-ui-thread)
- [Task: handle completion, failure and cancellation](https://gpui-kit.com/docs/task/#handle-completion-failure-and-cancellation)
- [GPUI Kit's input component](https://gpui-kit.com/component/input), for quiz 5
