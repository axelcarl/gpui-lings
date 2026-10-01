# Deeper contexts & input · 21–24

The earlier lessons introduce one focus handle, one action binding, and one
entity update at a time. This chapter combines them into small input flows:

- [21 · Focus regions](../../playground/src/exercises/interaction/regions.rs):
  focus an overlay's two regions and return focus to its launcher on close.
- [22 · Nested action](../../playground/src/exercises/interaction/propagation.rs):
  let a child consume an action or explicitly pass it to the parent.
- [23 · Deferred update](../../playground/src/exercises/contexts/deferred.rs):
  schedule a second entity update after the current mutable borrow ends.
- [24 · Command menu](../../playground/src/exercises/interaction/menu.rs):
  combine a scoped action, focus, keyboard selection, and Escape cancellation.

Each source file contains its task and check. Lesson 24 is a checkpoint in a
new scenario: it reports three bugs as symptoms instead of marking them in the
source. Revisit [actions and focus](../04_interaction/README.md) and
[contexts](../03_contexts/README.md) when needed. The checks send real input
and verify state and focus after each transition.

References: [GPUI contexts](https://github.com/zed-industries/zed/blob/main/crates/gpui/docs/contexts.md),
[key dispatch](https://github.com/zed-industries/zed/blob/main/crates/gpui/docs/key_dispatch.md).
