# Async data & failure paths · 25–28

These lessons build on [15 · Keep async work alive](../../exercises/05_lifetimes/lifetimes2.rs).
Each preview has its own tasks and deterministic input; tests advance GPUI's
clock without waiting for real time.

- [25 · Background work](../../exercises/08_async/async1.rs):
  await a background calculation and render its result in the owning entity.
- [26 · Error and retry](../../exercises/08_async/async2.rs):
  replace an error with Loading immediately, then show the next injected result.
- [27 · Stale results](../../exercises/08_async/async3.rs):
  accept the newest selection even when an older request completes later.
- [28 · Searchable results](../../exercises/08_async/async4.rs):
  type a query, retry a failed search, and choose a result with the keyboard.

The checkpoint reports three bugs as symptoms instead of marking them in the
source; one reaches back to entity identity in lesson 09. It uses GPUI Base's
unstyled input so platform text editing works; the dedicated text-input lesson
in the [plan](../../lesson-plan.md) remains open.
References: [GPUI contexts](https://github.com/zed-industries/zed/blob/main/crates/gpui/docs/contexts.md)
and [GPUI overview](https://github.com/zed-industries/zed/blob/main/crates/gpui/README.md).
