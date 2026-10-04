# GPUI Lings: next lessons

This tracks the course's lessons and the ones still planned. A checked box means implemented; an empty box means planned. Lesson numbers follow the [42-lesson order](README.md#curriculum) of October 4, 2026; [QUALITY.md](QUALITY.md) keeps the numbers each entry had at the time.

The progression borrows Rustlings' rhythm: several small exercises on one topic, a short chapter reference, then a checkpoint that combines earlier skills. Most exercises change something visible in the native preview; lesson 39 asks the learner to write the focused check itself, and quiz 1 asks for a whole view. Checkpoints use a new scenario rather than repeat an earlier fix, and report symptoms instead of marking the fix; see [Checkpoints and retrieval](#checkpoints-and-retrieval).

## Bridges

These belong at the indicated points in the learning sequence. Assign IDs and migrate saved progress when inserting them; do not silently shift a learner's completed lesson.

- [x] **After 03 · Write a `Render` view.** Lessons 02 and 03 are now views of their own, and quiz 1 (12) has the learner write a whole `Render` implementation after chapter 03.
- [ ] **After 11 · Choose observation or an event.** Have one child expose changing state and emit a one-time action; mirror the state with an observer and record the action with a subscription. Check that each channel carries the right information. Quiz 2 (20) now uses both in one view, but as debugging; choosing between them is still untested.
- [ ] **After 16 · Enter and edit text.** Accept typed characters, deletion, and focus changes in a simple input, including the platform text-input path where supported. Check the resulting text and focus behavior. This prepares the searchable-results checkpoint and a real app form.

## Checkpoints and retrieval

A curriculum review on October 1, 2026 found a use for all 38 lessons. The weak point is what finishing a checkpoint proves. [`scripts/audit_lessons.py`](scripts/audit_lessons.py) measures each reference fix. Before the capstone pilot, every lesson was solved by one or two edit regions of at most seven changed lines, with a TODO within five lines of each fix. The checkpoints (20, 24, 28, 38) are 170–300-line apps that asked for one or two marked repairs. The hints for 29, 35, and the old 38 state the code. The capstone check asserted its two repairs and the narrow layout, but not the shortcut, which the Save click already covered. Passing showed that a learner could follow a marker, not find a fault.

The course already revisits its core patterns at increasing distances. That is spacing; only the cues need to go:

| Pattern | Lessons |
| --- | --- |
| Keep a returned `Subscription` or `Task` | 11, 19, 20, 34, 36 |
| Notify after a mutation | 02, 05, 07, 28, 30 |
| Choose flex direction at a breakpoint | 04, 21, 25, 42 |
| Return focus to its owner | 13, 26, 28 |
| Ignore a stale completion | 31, 32 |
| Read the shared entity, not a copy | 09, 10, 20, 42 |
| Hold a strong handle only where you own | 17, 20 |

The research supports spaced revisits, retrieval without cues, and withdrawing guidance as skill grows. It does not support a fixed "two steps forward, one step back" difficulty curve. These rules change existing lessons in place, so IDs and saved progress stay valid:

1. **Fade markers by chapter.** Chapters 01–05 keep a TODO at the line to change. From chapter 06, a new concept's TODO names the behavior or function, not the line. Checkpoints have no inline markers. *(October 2, 2026: a playtest found context missing, so every lesson now marks the line to change and explains its code, as Rustlings does. Quizzes keep no markers. See [QUALITY.md](QUALITY.md).)*
2. **Report symptoms.** A checkpoint header lists what a tester would observe. Its check asserts each symptom, and the assertion message describes the symptom, not the fix.
3. **Mix distances.** A checkpoint has three to five faults from different earlier lessons, at least one from two or more chapters back.
4. **Vary the surface.** A revisited fault must not repeat its first lesson's token. Lesson 19's `drop(task)` becomes `let _task = cx.spawn(…)` beside a `_task` field in 42, so recognition alone does not solve it.
5. **Assert every claim.** Each feature a checkpoint names has an assertion, so a learner cannot break a prewired part and still pass.
6. **Hint at the strategy first.** A checkpoint hint names the method, then the lessons to revisit. Exact code waits for the last hint level.
7. **Verify each fault alone.** Leave one fault unfixed at a time and confirm that the check fails with that fault's message. Each fault must still compile and launch, and must not add a compiler warning that points to it.

- [x] **42 · Capstone pilot.** Five unmarked faults from lessons 19, 30, 21, 09, and 13. Each is caught alone by its own message, and the reference fix passes the verifier.
- [x] **25 · Inspector.** Four faults: the breakpoint reads height (21), the key handler sits off the focus path (13–16), only the mouse path checks disabled (23), and the list has no bounded height (22). The old scroll assertion passed even when nothing overflowed; the check now compares the list with its frame and requires the last row to come into view.
- [x] **28 · Command menu.** Three faults: arrow keys change state without notifying (07), the key context sits on the menu instead of the root (16), and Escape focuses the closed menu (26).
- [x] **32 · Searchable results.** Three faults: the view subscribes to one input but renders another (09), the stale-generation guard is reversed (31), and a repeated-query shortcut swallows Retry (30). The separate retry test moved into the exercise check so the starter can break retry.
- [x] **Hint levels.** `h` reveals one level per request and starts over on a new lesson; `./gpui-lings hint ID N` shows level N. Checkpoints go from strategy to lessons to exact changes. Lessons 18, 33, and 39, whose single hint gave the code, now start with a strategy.
- [x] **Fade markers in chapters 06–10.** Each TODO now sits on the function or element that owns the behavior. Lesson 27 lost its empty `else`, and 18 lost its unused weak handle, so neither placeholder gives the answer away. Lessons 34 and 36 had no marker and revisited lesson 11 in new forms. A playtest reversed this on October 2, 2026: every lesson marks its line again, and only quizzes have no markers.
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

## Progression ideas from Rustlings

A comparison with Rustlings 6.5 on October 4, 2026, implemented the same day. Rustlings has 94 exercises
in 23 topics, three of them quizzes, which follow the 16th, 45th and 61st
exercise. For 42 exercises, making
the code compile is the whole exercise: rustc's error message does the
teaching. That one doesn't transfer, since the playground compiles every
exercise into one binary and a compile error would stop every preview. Hints
lean on rust-analyzer instead ("hover `on_click` to see the callback it
expects"). The rest suggests these changes, roughly in order of value:

- [x] **An early quiz on the entity model.** Rustlings' first quiz comes after
  16 short exercises; ours came at lesson 20, and listeners, `observe`,
  `subscribe` and weak handles were never the fault in any quiz. Quiz 2 (20)
  now follows chapter 05: an inbox with four faults from lessons 17 (the panel
  keeps only a weak handle to its badge), 10 (the observer copies a stale
  value), 11 (the subscription is discarded) and 19 (Fetch detaches its task, so
  Cancel can't stop it). Each fails the check alone with its own symptom.
- [x] **One quiz you build instead of debug.** Rustlings' quizzes give a spec,
  the tests and an empty signature. Quiz 1 (12) follows chapter 03 the same
  way: a description of a tally view, a `render` that returns a placeholder
  so the playground still launches, and a check of the label, the row of
  buttons, Tap and Clear.
- [x] **Smaller steps where an idea is new.** Chapter 04 is now `04_keyboard`,
  one idea per lesson: focus (13), a key handler the learner writes (14, new),
  an action with an unscoped binding (15, new), and a key context (16, the old
  lesson 12). Focus now comes before anything that needs it.
- [x] **Ask for more code later in the course.** Rustlings raises difficulty by
  how much you write. Lesson 24 now has the learner write all three remaining
  drag handlers (17 lines), lesson 29 both the background and the foreground
  task (13 lines), lesson 26 a second way to close the overlay (10 lines), and
  quiz 1 a whole view (31 lines). Lessons before chapter 06 stay at one or two
  short edits.
- [x] **Repeat an idea within one file.** Lesson 11 subscribes to a second
  event type and keeps both subscriptions, lesson 24's two release handlers do
  the same cleanup, and lesson 26 restores focus on Escape as well as Close.
- [x] **Touch GPUI in chapter 01.** Lessons 02 and 03 are views now: one fix
  sits inside a `cx.listener` closure, the other inside `render`. Lesson 01
  stays a function that the playground calls. It is checked with `rustc` alone,
  so a fresh clone gets its first green check in seconds, before GPUI's test
  build exists. The first native check moved from lesson 04 to 02; the build it
  waits for is the same.
- [x] **Name chapters after what they teach.** `04_interaction` became
  `04_keyboard`, `06_responsive` became `06_layout_states`, and `07_deeper`
  became `07_dispatch`. `cx.defer` moved into chapter 05, between weak handles
  and tasks: it needs a weak handle, and it is the first kind of "later". The
  playground compiles each exercise as a module named after its file, such as
  `exercises::keyboard2`, instead of `lifetimes::background`. Saved state now
  carries a version, and version 1 names map to their new exercises on load.
- [x] **Teach the tool in lesson 01.** Rustlings' `intro1` explains watch mode
  before any fix. Lesson 01's introduction now covers saving, the check, `n`
  and `h`.
- [x] **Say when the course goes out of order.** Rustlings announces it
  ("Going out of order from the book to cover tests"). Every check here is a
  `#[gpui::test]`, but testing is lesson 35. The chapter 02 README now explains
  how to read a check.

## 06 · Layout & control states

- [x] **21 · Adapt a layout to window width.** Change a two-column view into a single column below a width threshold. Check child positions at narrow and wide window sizes, including after resize. Builds on 04 and 06.
- [x] **22 · Keep content usable in a small window.** Make a long panel scroll inside its allotted area while its header stays visible. Check that the last item can be reached and the header remains in place.
- [x] **23 · Show the state of a control.** Give a toggle distinct hover, keyboard-focus, selected, and disabled states. Check the state changes and that a disabled control does not activate.
- [x] **24 · Route a pointer gesture.** Make a small draggable value control respond to press, move, and release without leaving it stuck in a pressed state. The learner writes the move and both release handlers. Check the value and both release paths.
- [x] **25 · Quiz 3: compact inspector.** Combine responsive layout, scrolling, and control states in one small inspector. Check behavior at two window sizes and with keyboard navigation.

## 07 · Focus & dispatch

- [x] **26 · Move between focus regions.** Give a dialog or menu two focusable regions, move focus deliberately, and restore it when the overlay closes, by Close or Escape. Check where keys are delivered before and after closing. Builds on 13–16.
- [x] **27 · Route a nested action.** Let a child handle an action while its parent handles the same action only when the child does not. Check both routes and avoid duplicate handling.
- [x] **18 · Defer a follow-up update.** (Moved to chapter 05.) Trigger a state change that needs a second update after the current mutation completes. Use GPUI's deferred work mechanism, keep entity access inside the appropriate context, and check the final state after effects run.
- [x] **28 · Quiz 4: command menu.** Use focus, scoped actions, and event routing to open a small command menu, choose an item, and return focus to the caller. Check keyboard-only use and escape/cancel behavior.

## 08 · Async data and failure paths

- [x] **29 · Run expensive work off the UI thread.** Compute a result on the background executor and bring it back to an entity for rendering. The learner writes both tasks. Check that the UI remains responsive while the work runs. Builds on 19.
- [x] **30 · Show errors and retry.** Render loading, success, and recoverable error states from an injected, deterministic data source. Retry should clear the error and show the eventual result; check each transition.
- [x] **31 · Ignore stale results.** Start two requests, finish them out of order, and keep the newer selection on screen. Check that an old completion cannot overwrite it, including after the view is released.
- [x] **32 · Quiz 5: searchable results.** Combine background work, loading/error UI, retry, and stale-result protection in a searchable list. Check rapid query changes and keyboard interaction.

## 09 · Build an application, not just a view

- [x] **33 · Start a standalone GPUI app.** Wire `Application`, a root view, and the first window in a small, isolated example. Check startup and a visible root element. The current playground supplies this wiring for the learner.
- [x] **34 · Share application state.** Put a setting used by two views in app-level state and update both from one control. Check that each view sees the same value without copying it into unrelated local state.
- [x] **35 · Save and restore a setting.** Persist a simple preference, load it on startup, and recover from missing or invalid data. Check round-trip and fallback behavior with a temporary location.
- [x] **36 · Open a second window.** Open a detail window from the main view, update shared data, and close it without leaving stale handles or subscriptions. Check both windows' state and cleanup.
- [x] **37 · Follow appearance changes.** Centralize colors and spacing, respond to light/dark appearance, and keep foreground text legible in each. Check semantic state and inspect rendered output in both modes.

## 10 · Ship-quality GPUI

- [x] **38 · Expose an accessible control.** Add an appropriate role, name, state, and keyboard operation to a control using the accessibility support available in the pinned GPUI version. Check the exposed semantics where test support permits. A manual screen-reader pass remains open in `QUALITY.md`.
- [x] **39 · Test behavior through GPUI.** Write a headless interaction test that sends an event, checks rendered state, and advances an async task without sleeping. This is a learner-authored test rather than another test supplied entirely by the course.
- [x] **40 · Render a large collection efficiently.** Show a long changing list without rebuilding or painting every offscreen row. Check visible rows, stable item identity, and scroll behavior. GPUI Base's virtual list supplies the visible range.
- [x] **41 · Compose a reusable component.** Extract a control with explicit input, event output, disabled behavior, and theme usage. Use it in two views and check that each instance keeps independent state.
- [x] **42 · Quiz 6 (capstone): small native workspace.** Build a two-pane note workspace that combines entities, responsive layout, actions, focus, async loading with retry, persistence, and an accessible save control. Five reported bugs are found without source markers.

## Optional extension after the core path

- [ ] **GPUI Kit components.** Rebuild one earlier control with GPUI Kit components and theme tokens. Keep this separate from core GPUI so a change to Kit does not block the main course.
- [ ] **Custom element.** Implement a small measured/drawn element when ordinary view composition cannot meet the need; compare it with the simpler view version.

## Lesson authoring checklist

- Add a focused, compilable starter exercise in the [lesson style](#lesson-style). The broken behavior must still let the native app launch.
- Add its metadata and hints to `shared/lessons.rs`, and any background it needs to its chapter `README.md`.
- Make the preview independent of prior solutions and resettable; use deterministic input, time, and data for checks.
- Verify the starter fails only its intended check and a reference fix passes in an isolated copy, following the [exercise contract](PLAN.md#exercise-contract).
- For a checkpoint, follow the [checkpoint rules](#checkpoints-and-retrieval) and run `scripts/audit_lessons.py`.
- Recheck API names and platform support against the repository's pinned GPUI snapshot before implementing a planned lesson. Upstream GPUI is still changing.

### Lesson style

Lessons follow Rustlings' voice and layout. GPUI has no book to lean on, so the
chapter README does the book's job.

- **Introduction.** Open with plain `//` comments, two to ten lines: what the
  idea is and what this small app does, plus what to try in the preview. Talk
  to the learner as "you", and to the course as "we". Use contractions and short
  sentences, and backtick every identifier. No title, goal paragraph or example
  block: the terminal and the preview already show the title and goal.
- **TODO.** Put it at the line to change. Say what's wrong and what should
  happen, and name an API the learner hasn't met yet, but leave the finished
  code to the hints. When the file already does something similar, point at it
  ("the way `load` does"). When an API's shape matters, show it with `???`
  holes, as Rustlings does.
- **Hints.** Explain why the fix works, refer back to earlier lessons by number,
  and end with a link to the matching GPUI Kit guide section, on its own line.
  Separate paragraphs with a blank line; the guide keeps them apart. A second
  level can give the exact change.
- **Chapter README.** A few short paragraphs on the chapter's ideas, one small
  example, and a "Further information" list of guide links. Add the chapter to
  [`exercises/README.md`](exercises/README.md).
- **Quizzes.** Start with "This is a quiz for the following lessons:". A debug
  quiz lists the reported symptoms; a build quiz, like quiz 1, describes what
  to build and the debug selectors the check looks for. Keep the three hint
  levels.

## References and progression model

- [Rustlings exercise topics](https://github.com/rust-lang/rustlings/tree/main/exercises) and [usage guide](https://github.com/rust-lang/rustlings/blob/main/website/content/usage/index.md): topic directories, short exercises, hints, and a predefined order. Its [exercise catalog](https://github.com/rust-lang/rustlings/blob/main/rustlings-macros/info.toml) places quizzes between groups.
- [GPUI overview](https://github.com/zed-industries/zed/blob/main/crates/gpui/README.md): application startup, views, entities, elements, actions, executor, and tests.
- [GPUI contexts](https://github.com/zed-industries/zed/blob/main/crates/gpui/docs/contexts.md) and [key dispatch](https://github.com/zed-industries/zed/blob/main/crates/gpui/docs/key_dispatch.md): reference material for the context and input chapters.
