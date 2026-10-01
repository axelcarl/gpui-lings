# GPUI Lings plan

The learner edits one deliberately flawed native app. The terminal guide checks
small exercises in order; the app provides immediate visual feedback.

## First playable slice — complete

- [x] Create a dependency-free guide with `list`, `check`, `hint`, `watch`, and `app` commands.
- [x] Start the guided session, watch files, and launch the playground with one command.
- [x] Show only the active exercise and advance on `n` after a passing check.
- [x] Split exercise code and guides into individual files grouped by chapter.
- [x] Keep starter behavior bugs compilable so the app always launches.
- [x] Add three beginner exercises with focused checks that run without GPUI.
- [x] Explain each exercise and provide an optional hint.
- [x] Compile and start the native playground on macOS.
- [x] Visually inspect the native window; check hints, file-path copying, counter clicks, keyboard activation, and preview reset.
- [x] Add GPUI-rendered layout and entity exercises with headless interaction checks.
- [x] Verify all fifteen starter checks fail and all fifteen reference fixes pass, without modifying learner files.

## Experience and quality pass — September 29, 2026

- [x] Add a consistent slate/mint theme, lesson rail, progress indicators, preview frame, and readable concept/source sections.
- [x] Make sidebar titles wrap and main content scroll; check layout at the 820 × 620 minimum window size.
- [x] Add accessible button labels, keyboard activation, focus outlines, inline hints, file-path copying, and preview reset.
- [x] Replace compiler-log streaming with a terminal dashboard, progress, readable status, and on-demand diagnostics.
- [x] Add guide/previous commands and explicit lesson IDs for `check` and `app`.
- [x] Support plain output and NO_COLOR; wrap long paths and Unicode text.
- [x] Use one lesson catalog for the app and terminal.
- [x] Detect edited/deleted files and atomic saves; debounce checks and retain changes made during a build.
- [x] Reject empty/ignored test matches, close children on quit/EOF, and write progress atomically.
- [x] Migrate old completion records so new lessons are not silently skipped.
- [x] Make the milestone preview independent of the counter solution.
- [x] Add guide/app regression checks and clear strict Clippy findings in both packages.

## Terminal simplification and lesson structure

- [x] Put the current exercise number/title first, followed by one source path,
  one goal, and one status line. Remove the banner, progress art, time estimate,
  chapter/concept labels, second file path, and repeated coaching text.
- [x] Keep `h`, `n`, `?`, and `q` visible. Put the full command list behind `?`;
  hints and diagnostics remain available on demand.

**Implemented: combine each exercise's explanation with its source.**
The Rustlings pattern fits these focused tasks: instructions beside code,
chapter READMEs for context, and a shared catalog for order and optional hints.
References: [exercise instructions](https://github.com/rust-lang/rustlings/blob/main/exercises/05_vecs/vecs2.rs),
[chapter context](https://github.com/rust-lang/rustlings/blob/main/exercises/01_variables/README.md),
and [catalog](https://github.com/rust-lang/rustlings/blob/main/rustlings-macros/info.toml).

- [x] Migrate all six original guides into source comments and chapter references.
- [x] Have `g` read current source instructions and the app use their concept
  paragraph; remove duplicated detailed explanations from the catalog.
- [x] Add nine source-based exercises covering contexts, handlers, entity updates,
  observers, event subscriptions, actions, focus, weak handles, and tasks.
- [x] Keep every preview independent and resettable; scroll the rail to the
  current exercise while keeping the brand and progress visible.
- [x] Add real input, notification, payload, ownership, and deterministic timer
  checks, plus all-nine-preview shell integration coverage at minimum size.

## Visual refresh

- [x] Restyle the app after stock shadcn/ui: neutral light and dark palettes that
  follow the system appearance, a sidebar with chapter groups, a breadcrumb
  header with one status badge, and a preview card.
- [x] Remove decorative labels, concept/duration tags, and coaching footnotes.
- [x] Dim and lock lessons past the saved position; dim chapters with no open lessons.
- [x] Use embedded Lucide icons and shadcn-style buttons, badges, and focus rings.
- [x] Restyle previews that used their own colors (flex, entity, spacing, focus)
  with the shared palette.
- [x] Reveal the active lesson whenever the sidebar changes size.
- [x] Focus the window on the code's output: drop the sidebar, concept text, file
  card, and hint; keep progress in the header and let the preview fill the window.
- [x] Color the check status red while failing and green once the learner can move on.

## Remaining work, in order

The numbered core path through lesson 38 is implemented. The three earlier
bridges and optional extensions remain in [lesson-plan.md](lesson-plan.md);
inserting bridges into saved progress needs an explicit migration.

1. **Finish the layout chapter.**
   - [x] Lesson 06: consistent pixel spacing with rendered-bounds checks.
   - [x] Lesson 16: responsive structure at multiple window sizes.
   - [x] Lesson 17: scrollable content under a fixed heading.
   - [x] Lessons 18–20: control states, pointer gesture, and a compact inspector checkpoint.
   - [ ] Empty visual states as a learner exercise.
2. **Extend entities and contexts.**
   - [x] Lessons 07–11: contexts, handlers, parent/child updates, observers, and event subscriptions.
   - [x] Check that state updates reach the original child and model changes reach observers.
   - [x] Lesson 14: strong versus weak entity ownership.
   - [x] Lesson 23: `cx.defer` across a borrow boundary.
3. **Teach actions, focus, and keyboard navigation.**
   - [x] Lessons 12–13: bind a named action, move focus deliberately, test key-driven behavior.
   - [x] Lessons 21–22: multiple focus regions and action propagation.
   - [x] Lesson 24: command menu checkpoint with keyboard selection and cancellation.
   - [ ] Natural Tab traversal through multiple regions.
4. **Async work.**
   - [x] Lesson 15: loading, task retention, and cancellation without blocking the UI.
   - [x] Background computation and recoverable error/retry flows.
   - [x] Out-of-order requests and stale-result protection.
   - [x] Searchable-results checkpoint with typed input and keyboard selection.
5. **Persistence and accessibility.**
   - [x] Save application state and recover from invalid data.
   - [x] Teach semantics and focus with integration checks where GPUI exposes them.
   - [ ] Complete a screen-reader audit of lesson content beyond the labeled controls.
6. **GPUI Kit.**
   - [ ] Introduce styled components, theming, and application structure after the core GPUI chapters.
7. **Checkpoints that require diagnosis.**
   - [x] Capstone 38: five unmarked faults reported as symptoms, each caught alone.
   - [x] Checkpoints 20, 24, and 28 follow the same rules; hints reveal one level at a time;
     markers after chapter 05 name the behavior instead of the line.
   - [ ] Playtest the checkpoints and inspect them natively. See
     [Checkpoints and retrieval](lesson-plan.md#checkpoints-and-retrieval).
8. **Practice and distribution.**
   - [ ] Add source reset with an explicit backup/restore contract and versioned starter files.
   - [ ] Track completed lessons separately from the selected lesson, for free navigation and review.
   - [ ] Check Linux/Windows native setup, packaging, and terminal behavior; this pass was verified on macOS.

## Exercise contract

Every behavior exercise must change visible app behavior, have a narrow automated
check, and explain its GPUI concept. Lesson 35 instead asks the learner to write
the interaction test for a working view. A broken exercise must not prevent
launch. Starter tests intentionally fail. Test reference fixes in an isolated
copy, preserving the user's exercise code. Add new lesson metadata to
`shared/lessons.rs`, source instructions, a chapter reference, and a reference
patch in `scripts/verify_lessons.py`.
