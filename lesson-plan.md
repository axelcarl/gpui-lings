# GPUI Lings: next lessons

This tracks lessons to build after the [original 15 lessons](README.md#curriculum). A checked box means the lesson is implemented; an empty box means it is planned. Numbers after 15 are tentative until implemented.

The progression borrows Rustlings' rhythm: several small exercises on one topic, a short chapter reference, then a checkpoint that combines earlier skills. Most exercises change something visible in the native preview; lesson 35 asks the learner to write the focused check itself. Checkpoints use a new scenario rather than repeat an earlier fix, and report symptoms instead of marking the fix; see [Checkpoints and retrieval](#checkpoints-and-retrieval).

## Bridges within the current 01–15 sequence

These belong at the indicated points in the learning sequence. Assign IDs and migrate saved progress when inserting them; do not silently shift a learner's completed lesson.

- [ ] **After 03 · Write a `Render` view.** Move a derived counter display from a prewired helper into a small state-owning view. Check that the root renders its current state after an update. This makes the `Render` implementation used in 05 explicit before introducing child entities.
- [ ] **After 11 · Choose observation or an event.** Have one child expose changing state and emit a one-time action; mirror the state with an observer and record the action with a subscription. Check that each channel carries the right information. This consolidates 07–11 before keyboard input.
- [ ] **After 13 · Enter and edit text.** Accept typed characters, deletion, and focus changes in a simple input, including the platform text-input path where supported. Check the resulting text and focus behavior. This prepares the searchable-results checkpoint and a real app form.

## Checkpoints and retrieval

A curriculum review on October 1, 2026 found a use for all 38 lessons. The weak point is what finishing a checkpoint proves. [`scripts/audit_lessons.py`](scripts/audit_lessons.py) measures each reference fix. Before the capstone pilot, every lesson was solved by one or two edit regions of at most seven changed lines, with a TODO within five lines of each fix. The checkpoints (20, 24, 28, 38) are 170–300-line apps that asked for one or two marked repairs. The hints for 29, 35, and the old 38 state the code. The capstone check asserted its two repairs and the narrow layout, but not the shortcut, which the Save click already covered. Passing showed that a learner could follow a marker, not find a fault.

The course already revisits its core patterns at increasing distances. That is spacing; only the cues need to go:

| Pattern | Lessons |
| --- | --- |
| Keep a returned `Subscription` or `Task` | 11, 15, 30, 32 |
| Notify after a mutation | 02, 05, 07, 26 |
| Choose flex direction at a breakpoint | 04, 16, 20 |
| Return focus to its owner | 13, 21, 24 |
| Ignore a stale completion | 27, 28 |
| Read the shared entity, not a copy | 09, 38 |

The research supports spaced revisits, retrieval without cues, and withdrawing guidance as skill grows. It does not support a fixed "two steps forward, one step back" difficulty curve. These rules change existing lessons in place, so IDs and saved progress stay valid:

1. **Fade markers by chapter.** Chapters 01–05 keep a TODO at the line to change. From chapter 06, a new concept's TODO names the behavior or function, not the line. Checkpoints have no inline markers.
2. **Report symptoms.** A checkpoint header lists what a tester would observe. Its check asserts each symptom, and the assertion message describes the symptom, not the fix.
3. **Mix distances.** A checkpoint has three to five faults from different earlier lessons, at least one from two or more chapters back.
4. **Vary the surface.** A revisited fault must not repeat its first lesson's token. Lesson 15's `drop(task)` becomes `let _task = cx.spawn(…)` beside a `_task` field in 38, so recognition alone does not solve it.
5. **Assert every claim.** Each feature a checkpoint names has an assertion, so a learner cannot break a prewired part and still pass.
6. **Hint at the strategy first.** A checkpoint hint names the method, then the lessons to revisit. Exact code waits for the last hint level.
7. **Verify each fault alone.** Leave one fault unfixed at a time and confirm that the check fails with that fault's message. Each fault must still compile and launch, and must not add a compiler warning that points to it.

- [x] **38 · Capstone pilot.** Five unmarked faults from lessons 15, 26, 16, 09, and 13. Each is caught alone by its own message, and the reference fix passes the verifier.
- [x] **20 · Inspector.** Four faults: the breakpoint reads height (16), the key handler sits off the focus path (12–13), only the mouse path checks disabled (18), and the list has no bounded height (17). The old scroll assertion passed even when nothing overflowed; the check now compares the list with its frame and requires the last row to come into view.
- [x] **24 · Command menu.** Three faults: arrow keys change state without notifying (07), the key context sits on the menu instead of the root (12), and Escape focuses the closed menu (21).
- [x] **28 · Searchable results.** Three faults: the view subscribes to one input but renders another (09), the stale-generation guard is reversed (27), and a repeated-query shortcut swallows Retry (26). The separate retry test moved into the exercise check so the starter can break retry.
- [x] **Hint levels.** `h` reveals one level per request and starts over on a new lesson; `./gpui-lings hint ID N` shows level N. Checkpoints go from strategy to lessons to exact changes. Lessons 23, 29, and 35, whose single hint gave the code, now start with a strategy.
- [x] **Fade markers in chapters 06–10.** Each TODO now sits on the function or element that owns the behavior. Lesson 22 lost its empty `else`, and 23 lost its unused weak handle, so neither placeholder gives the answer away. Lessons 30 and 32 have no marker and revisit lesson 11 in new forms: an underscore-named local and `let _ = subscription;`. The audit reports `line` for 01–15, `function` for the later lessons, and `none` for the checkpoints, 30, and 32.
- [ ] **Playtest.** Time learners on the checkpoints. If they stall, check whether they used the hint levels before adding markers back.
- [ ] **Native visual pass.** Inspect the rewritten checkpoints in the native preview, including each reported symptom.

The planned bridge *Choose observation or an event* is a discrimination task: two similar mechanisms, and the learner must pick one. Interleaving helps most in exactly that situation.

Evidence behind the rules:

- **Finding is the hard part.** Students found 70% of bugs and fixed 97% of those they found ([Fitzgerald et al., 2008](https://doi.org/10.1080/08993400802114508); 21 students). Markers remove the finding step (rules 1–2).
- **Retrieval beats re-exposure.** Practice tests outperform restudy ([Rowland, 2014](https://doi.org/10.1037/a0037559); [Adesope et al., 2017](https://doi.org/10.3102/0034654316689306), g = 0.61 overall). A marked revisit is re-exposure (rules 1, 3, 4).
- **Spacing.** The best gap between sessions grows with the gap before the final test ([Cepeda et al., 2006](https://doi.org/10.1037/0033-2909.132.3.354)). Those studies cover verbal recall, so take the direction, not a schedule (rule 3).
- **Fade support as skill grows.** Removing worked steps gradually improves transfer ([Renkl et al., 2002](https://doi.org/10.1080/00220970209599510); [Atkinson et al., 2003](https://doi.org/10.1037/0022-0663.95.4.774)). Guidance that helps novices can hinder more knowledgeable learners ([Kalyuga et al., 2003](https://doi.org/10.1207/S15326985EP3801_4)). Early chapters keep markers for that reason (rules 1, 6).
- **Hints get abused.** In Cognitive Tutor classrooms, gaming the system, including help abuse, was the behavior most strongly linked to lower learning ([Baker et al., 2004](https://doi.org/10.1145/985692.985741)) (rule 6).
- **Interleave similar things.** Interleaving helps overall (g = 0.42), more when categories look alike, and blocking wins for word lists ([Brunmair & Richter, 2019](https://doi.org/10.1037/bul0000209)) (the bridge above).
- **No sawtooth.** No study found validates deliberately alternating harder and easier exercises. The nearest support is desirable difficulties (Bjork, 1994, in *Metacognition: Knowing about Knowing*). Fading and expertise reversal both argue for steady withdrawal.

## 06 · Responsive views and interaction states

- [x] **16 · Adapt a layout to window width.** Change a two-column view into a single column below a width threshold. Check child positions at narrow and wide window sizes, including after resize. Builds on 04 and 06.
- [x] **17 · Keep content usable in a small window.** Make a long panel scroll inside its allotted area while its header stays visible. Check that the last item can be reached and the header remains in place.
- [x] **18 · Show the state of a control.** Give a toggle distinct hover, keyboard-focus, selected, and disabled states. Check the state changes and that a disabled control does not activate.
- [x] **19 · Route a pointer gesture.** Make a small draggable value control respond to press, move, and release without leaving it stuck in a pressed state. Check the value and release behavior.
- [x] **20 · Layout checkpoint: compact inspector.** Combine responsive layout, scrolling, and control states in one small inspector. Check behavior at two window sizes and with keyboard navigation.

## 07 · Deeper contexts and keyboard routing

- [x] **21 · Move between focus regions.** Give a dialog or menu two focusable regions, move focus deliberately, and restore it when the overlay closes. Check where keys are delivered before and after closing. Builds on 12–13.
- [x] **22 · Route a nested action.** Let a child handle an action while its parent handles the same action only when the child does not. Check both routes and avoid duplicate handling.
- [x] **23 · Defer a follow-up update.** Trigger a state change that needs a second update after the current mutation completes. Use GPUI's deferred work mechanism, keep entity access inside the appropriate context, and check the final state after effects run.
- [x] **24 · Input checkpoint: command menu.** Use focus, scoped actions, and event routing to open a small command menu, choose an item, and return focus to the caller. Check keyboard-only use and escape/cancel behavior.

## 08 · Async data and failure paths

- [x] **25 · Run expensive work off the UI thread.** Compute a result on the background executor and bring it back to an entity for rendering. Check that the UI remains responsive while the work runs. Builds on 15.
- [x] **26 · Show errors and retry.** Render loading, success, and recoverable error states from an injected, deterministic data source. Retry should clear the error and show the eventual result; check each transition.
- [x] **27 · Ignore stale results.** Start two requests, finish them out of order, and keep the newer selection on screen. Check that an old completion cannot overwrite it, including after the view is released.
- [x] **28 · Async checkpoint: searchable results.** Combine background work, loading/error UI, retry, and stale-result protection in a searchable list. Check rapid query changes and keyboard interaction.

## 09 · Build an application, not just a view

- [x] **29 · Start a standalone GPUI app.** Wire `Application`, a root view, and the first window in a small, isolated example. Check startup and a visible root element. The current playground supplies this wiring for the learner.
- [x] **30 · Share application state.** Put a setting used by two views in app-level state and update both from one control. Check that each view sees the same value without copying it into unrelated local state.
- [x] **31 · Save and restore a setting.** Persist a simple preference, load it on startup, and recover from missing or invalid data. Check round-trip and fallback behavior with a temporary location.
- [x] **32 · Open a second window.** Open a detail window from the main view, update shared data, and close it without leaving stale handles or subscriptions. Check both windows' state and cleanup.
- [x] **33 · Follow appearance changes.** Centralize colors and spacing, respond to light/dark appearance, and keep foreground text legible in each. Check semantic state and inspect rendered output in both modes.

## 10 · Ship-quality GPUI

- [x] **34 · Expose an accessible control.** Add an appropriate role, name, state, and keyboard operation to a control using the accessibility support available in the pinned GPUI version. Check the exposed semantics where test support permits. A manual screen-reader pass remains open in `QUALITY.md`.
- [x] **35 · Test behavior through GPUI.** Write a headless interaction test that sends an event, checks rendered state, and advances an async task without sleeping. This is a learner-authored test rather than another test supplied entirely by the course.
- [x] **36 · Render a large collection efficiently.** Show a long changing list without rebuilding or painting every offscreen row. Check visible rows, stable item identity, and scroll behavior. GPUI Base's virtual list supplies the visible range.
- [x] **37 · Compose a reusable component.** Extract a control with explicit input, event output, disabled behavior, and theme usage. Use it in two views and check that each instance keeps independent state.
- [x] **38 · Capstone: small native workspace.** Build a two-pane note workspace that combines entities, responsive layout, actions, focus, async loading with retry, persistence, and an accessible save control. Five reported bugs are found without source markers.

## Optional extension after the core path

- [ ] **GPUI Kit components.** Rebuild one earlier control with GPUI Kit components and theme tokens. Keep this separate from core GPUI so a change to Kit does not block the main course.
- [ ] **Custom element.** Implement a small measured/drawn element when ordinary view composition cannot meet the need; compare it with the simpler view version.

## Lesson authoring checklist

- Add a focused, compilable starter exercise and source-level instructions. The broken behavior must still let the native app launch.
- Add its metadata and optional hint to `shared/lessons.rs`, plus a short chapter `README.md` with prerequisites and references.
- Make the preview independent of prior solutions and resettable; use deterministic input, time, and data for checks.
- Verify the starter fails only its intended check and a reference fix passes in an isolated copy, following the [exercise contract](PLAN.md#exercise-contract).
- For a checkpoint, follow the [checkpoint rules](#checkpoints-and-retrieval) and run `scripts/audit_lessons.py`.
- Recheck API names and platform support against the repository's pinned GPUI snapshot before implementing a planned lesson. Upstream GPUI is still changing.

## References and progression model

- [Rustlings exercise topics](https://github.com/rust-lang/rustlings/tree/main/exercises) and [usage guide](https://github.com/rust-lang/rustlings/blob/main/website/content/usage/index.md): topic directories, short exercises, hints, and a predefined order. Its [exercise catalog](https://github.com/rust-lang/rustlings/blob/main/rustlings-macros/info.toml) places quizzes between groups.
- [GPUI overview](https://github.com/zed-industries/zed/blob/main/crates/gpui/README.md): application startup, views, entities, elements, actions, executor, and tests.
- [GPUI contexts](https://github.com/zed-industries/zed/blob/main/crates/gpui/docs/contexts.md) and [key dispatch](https://github.com/zed-industries/zed/blob/main/crates/gpui/docs/key_dispatch.md): reference material for the context and input chapters.
