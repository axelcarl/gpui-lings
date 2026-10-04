# Quality check — September 29, 2026

## Verified

- Guide unit tests cover saved-progress migration, missing/ignored tests, catalog
  paths, file edits/deletions, Cargo target directories, and narrow text wrapping.
- Headless GPUI checks cover lesson isolation, the milestone preview's independent
  counter, preview reset, hint visibility, and minimum-window layout containment.
- All six starter exercise checks fail intentionally. All six pass with reference
  fixes in an isolated temporary copy; learner source files stay unchanged.
- Both packages pass strict Clippy. The native playground builds on macOS.
- Native visual inspection verified sidebar wrapping, scrolling, hint show/hide,
  copied-path feedback, Tab/Shift-Tab, Enter activation, counter changes through
  3 and 4, and reset to zero. The starter milestone deliberately celebrates at 4.
- A terminal session exercised hint, diagnostics, refusal to advance a failing
  lesson, and quit. One-shot help/list and lesson-specific checks were inspected.

## Terminal follow-up

The default screen now identifies the current exercise, its source file, goal,
and a single status. Progress artwork, banners, estimates, repeated explanatory
copy, and the separate guide path were removed. `?` exposes secondary commands;
`h` shows just the hint. Build/check messages replace stale results while work
is in progress. Existing guide tests and strict Clippy remain the validation
baseline; this change does not alter exercise source or native rendering.

## Issues corrected

The previous verifier treated a successful test process with no matching tests
as a pass. It now requires one executed, passing, non-ignored check. Old
`complete` progress records previously skipped newly added lessons; they now
resume at lesson 06. Source watching uses content and path changes instead of
the maximum modification time, so file deletion and timestamp ties are visible.
Standard-input closure ends the session and cleans up the child app.

The app's first visual pass exposed long titles overflowing the lesson rail.
Those now wrap within the rail, and a headless minimum-window regression check
covers the problem. Previewing lesson 03 no longer depends on solving lesson 02.

## Visual refresh

The app now follows shadcn/ui's neutral theme in light and dark appearance. Real
Metal frames of lessons 01, 03–07, and 11–13, the completion screen, an open hint,
and the 820 × 620 minimum size were rendered offscreen and inspected in both
appearances. Locked lessons and chapters render dimmed, and status badges cover
passing, failing, and build-error states.

This pass found that the sidebar only revealed the active lesson when an early
frame already knew the rail's size; after a resize it could stay hidden. The app
now reveals the active lesson whenever the rail changes size. The existing
minimum-window test covers lessons 07–15. Strict Clippy, the app checks, and the
starter/reference verifier pass.

## Focused window

The app now shows only a header with progress, the lesson goal, and the preview.
The status strip above the preview is red for failing checks and build errors and
green for passing ones. Real frames of passing, failing, error, and completion
states were captured in light and dark appearance. Headless checks cover the new
640 × 560 minimum size, the status for every check result, and all nine advanced
previews. Strict Clippy, the app checks, and the starter/reference verifier pass.

## Known limits

The saved progress format records a position in the course, not a per-lesson
completion history. Going back moves that position; independent history belongs
to the next practice/navigation pass. Native content can scroll on smaller
windows. Source reset and the later chapters are still on the roadmap.

Cargo reports a future-compatibility warning in the upstream `block 0.1.6`
dependency. It does not fail the current build or strict Clippy checks. The pinned
GPUI dependency graph was not upgraded during this UI pass. Native execution was
verified on macOS; other platforms and a full screen-reader audit remain open.

## Curriculum extension — September 30, 2026

Lessons 16–17 add responsive card layout and a bounded scrollable list. Headless
checks resize the first preview in both directions and send a scroll gesture to
the second. The starter verifier confirms that exactly 17 exercise checks fail;
all 17 reference fixes and the app checks pass in an isolated copy. Guide tests,
format checks, and strict Clippy pass for both packages. These two new previews
have not yet received a manual native visual pass.

Lessons 18–20 extend the chapter with control states, outside-release drag
cleanup, and an inspector checkpoint. The starter verifier confirms exactly 20
failing exercise checks; all 20 reference fixes and app checks pass. Guide tests,
format checks, and strict Clippy pass for both packages. Native visual inspection
of lessons 16–20 remains open; the previously installed six-lesson app is not a
valid preview of this checkout.

Lessons 21–24 add focus restoration, explicit action fallback, deferred entity
updates, and a command-menu checkpoint. The 24-lesson starter/reference verifier
and app checks pass; native visual inspection of these previews is still open.

Lessons 25–27 introduce background computation, an injected retry flow, and
out-of-order completion. Each exercise has a deterministic clock-driven check;
the 27-lesson starter/reference verifier, app checks, guide tests, and strict
Clippy pass. The native visual pass for these previews remains open.

Lesson 28 adds typed search through GPUI Base's input, simulated background
results, retry, and keyboard choice. Its check changes the query before the
earlier request finishes; the reference guard keeps the newer results visible.
All 28 starter checks fail as intended, all reference fixes and app checks
pass, and both packages pass tests, formatting, and strict Clippy. Native
visual inspection of this preview remains open.

Lesson 29 adds a separate GPUI Kit application entrypoint and a preview button
that opens its root in a new window once solved. The 29-lesson starter/reference
verifier, app checks, guide tests, and strict Clippy pass. Its native window
has not received a manual visual pass in this checkout.

Lesson 30 shares a GPUI Global across two child entities and tests that a
retained global observer repaints the summary on both setting changes. The
30-lesson starter/reference verifier, app checks, guide tests, and strict
Clippy pass. Native visual inspection of this preview remains open.

Lesson 31 saves a setting under a unique temporary path and restores it both
on startup and through a Load button. Its check covers missing and invalid
files. The 31-lesson starter/reference verifier, app checks, guide tests, and
strict Clippy pass; native visual inspection remains open.

Lesson 32 opens a detail window over a shared entity, updates it from either
window, and clears its handle on close so it can reopen. The 32-lesson
starter/reference verifier, app checks, guide tests, and strict Clippy pass.
Native visual inspection of this multiwindow flow remains open.

Lesson 33 uses window appearance and shared semantic tokens, with a dark-mode
preview that exposes the starter's illegible foreground. Light, dark, and
vibrant-dark state checks pass with the reference fix. The 33-lesson verifier,
app checks, guide tests, and strict Clippy pass. A manual visual pass of the
new appearance preview remains open.

Lesson 34 adds an accessible switch. Its check inspects the AccessKit role,
name, toggled state, and Click action, and activates the preview by pointer and
Space. The 34-lesson starter/reference verifier and app checks pass. A native
screen-reader pass is still needed to confirm spoken output and focus order.

Lesson 35 supplies a working asynchronous view with an incomplete learner test.
The reference test clicks through the rendered preview, checks Loading, advances
the GPUI test clock, and checks Ready without sleeping. Native visual inspection
of the new preview remains open.

Lesson 36 exercises GPUI Base's virtual list with 1,000 stable row IDs. The
reference check limits constructed rows, scrolls to the last row, and preserves
selection. Native visual inspection of this large-list preview remains open.

Lesson 37 adds a controlled, themed wrapper around GPUI Base's switch. Its
reference check covers independent child entities and disabled interaction.
Native visual inspection of the component preview remains open.

Lesson 38 combines observed list/detail entities, responsive panes, scoped
Ctrl-S, simulated retry, temporary persistence, and an accessible Save name.
The reference check covers the complete interaction and layout flow. Native
visual and screen-reader inspection of the capstone remains open.

## Capstone diagnosis pilot — October 1, 2026

Lesson 38 now lists five reported symptoms instead of marking two lines. The
faults reuse lessons 15, 26, 16, 09, and 13: a dropped load task, a retry that
leaves the old error, a reversed breakpoint, a Save that reads a stale copy
instead of the model, and a workspace that never attaches its focus handle. The
check now also asserts the wide layout and a Ctrl-S save separate from the Save
button. Each assertion message names a symptom. Leaving any single fault unfixed
fails the check with that fault's message; the full reference fix passes.

The 38-lesson starter/reference verifier, guide tests, app checks, and format
check pass. Strict Clippy fails only on `let_unit_value` in
`playground/src/preview.rs`, from commit 6c6f1a8, which this change does not
touch. With that lint allowed, the rest is clean. The new capstone has had no
native visual pass and no learner playtest yet.

## Checkpoints, hint levels, and fading — October 1, 2026

Checkpoints 20, 24, and 28 now follow the capstone's rules. Each lists reported
symptoms, has three or four unmarked faults from different earlier lessons, and
names each symptom in its check. Leaving any single fault unfixed fails the
check with that fault's message, and every intermediate state passes Clippy.
Two weak checks were found along the way. Lesson 20's scroll assertion passed
even when the list had no bounded height, because the scroll offset moved anyway;
it now compares the list with its frame and requires the last row to come into
view. Lesson 28's retry test was separate from the exercise check, so the
starter could not break retry; it is now part of `exercise_28`.

Hints reveal one level per `h` and start over on a new lesson. A guide test
covers labels, the "more" prompt, and clamping at the last level. Markers in
chapters 06–10 moved from the changed line to the function or element that owns
the behavior. `scripts/audit_lessons.py` reports line markers for 01–15,
function markers for the remaining non-checkpoint lessons except 30 and 32, and
none for those two and the checkpoints. Removing the comment in lesson 16 left
identical `if` branches, which Clippy rejects, so the narrow branch now
tightens its gap as well.

The 38-lesson verifier, guide tests, app checks, formatting, and Clippy on a
fully solved copy pass. Strict Clippy on the starter reports only the earlier
`let_unit_value` finding in `playground/src/preview.rs`. None of the rewritten
checkpoints has had a native visual pass or a learner playtest.

## Playtest feedback, Rustlings controls, and editor support — October 2, 2026

A playtest of lessons 01–19 led to these changes:

- **Controls.** The guide now uses Rustlings' watch-mode keys, acting on each
  key press without Enter: `n` next, `h` hint, `l` list, `c` check all, `x`
  reset, `q` quit. The list view supports navigation, continue at, reset,
  search, and done/pending filters. Reset runs `git stash push -- <file>`, as
  Rustlings does for exercises it doesn't embed. The screen is aligned to the
  bottom of the terminal. As in Rustlings, saving checks only the current
  exercise, and `c` rechecks everything. `./gpui-lings <lesson>` starts a session
  at any lesson.
- **Diagnostics.** A failed check leads with the assertion's message and shows
  the whole (possibly multi-line) assertion and its compared values.
- **Lessons.** Lessons 01–37 explain their code in Rustlings-style comments,
  and TODOs sit at the line to change. This reverses the marker fading of
  chapters 06–10, because the playtest found context missing. Starters no longer
  need a line deleted as part of the fix (`drop(subscription)` in 11,
  `drop(task)` in 15, similar placeholders in 25 and 29–32), and parameters the
  learner must use lost their `_` prefix. Lesson 14 starts from `self.target`
  instead of a hard-coded `None`. Lesson 17 also has the learner attach the
  ScrollHandle (Jump to last), so it is more than an overflow swap. Lesson 18's
  explanation and example are now about keeping a disabled control inert.
  Lessons 12 and 13 show whether focus is where keys will arrive, because every
  save opens a fresh window with nothing focused.
- **Quizzes.** Checkpoints 20, 24, 28 and 38 moved to `exercises/quizzes/quiz1–4.rs`.
  Saved progress that uses their old names migrates.
- **Editor support.** The playground's manifest moved to the repository root,
  with the guide in `guide/` and the shared catalog in `shared/` as a crate.
  rust-analyzer now resolves every exercise module: its diagnostics run reports
  no errors, compared with dozens of unresolved-module errors before.

The playtest also reported that Ctrl-K (lesson 12) and X (lesson 13) didn't
reach the preview. With the reference fixes applied, a real click and key press
reach the handlers in a native window, including one that starts inactive and
one launched with the guide's environment. A window launched by the guide often
doesn't become the active app, and each rebuild replaces it with nothing
focused. Until the learner clicks it, keys go to the previously active app. The
previews now show the focus state, and the lessons explain this.

Verified: guide and shared tests (34), app tests (13), the 38-lesson
starter/reference verifier, formatting, and Clippy with `-D warnings` on all
three packages. Lesson 30's preview looks correct even in its starter state,
because the control's notify redraws the whole window; only the hidden `updates`
counter shows the bug. Showing that counter would make the fault visible.

## Rustlings style pass — October 4, 2026

Lessons now read like Rustlings 6.5's exercises, compared side by side with a
local copy:

- **Exercise files.** The `//!` header (title, concept, Goal paragraph and an
  "Example —" block with an "illustrative names" disclaimer) became a short
  `//` introduction in the second person, with backticked identifiers. The
  terminal and preview already show the title and goal. TODOs stay at the line
  to change. They say what's wrong and name a new API, but no longer spell out
  the finished call in lessons 08, 10, 13, 14, 15, 21, 22, 23, 29 and 32; the
  hints still do. Where the file already does something similar, the TODO
  points at it, as Rustlings' `vecs2` does. Lesson 09 shows the shape of
  `update` with `???` holes. Lesson 01's introduction now explains the tool, as
  Rustlings' `intro1` does.
- **Background.** Rustlings leans on the Rust book. GPUI has none, so chapter
  READMEs now explain their ideas with one small example and end in "Further
  information" links to sections of the GPUI Kit guides, which cover GPUI one
  topic at a time. `exercises/README.md` maps chapters to guides, as Rustlings'
  does to book chapters, and recommends Zed's *Ownership and data flow in GPUI*
  as one longer read. The chapter 02 README explains how to read a check, since
  every lesson relies on one and testing is taught in lesson 35.
- **Hints.** Every hint explains why the fix works, refers back to earlier
  lessons by number, and ends with a link to the matching guide section. The
  guide now keeps paragraph breaks when wrapping, never splits a link, and
  wraps the one-shot `hint` command to the terminal width.
- **Plan.** `lesson-plan.md` gained a lesson style section in the authoring
  checklist and seven progression changes taken from Rustlings, from an early
  quiz on the entity model to chapter names. None of those change lesson IDs
  yet.

The unused `lessons::instructions` parser for `//!` headers was removed; the
catalog test now checks that every exercise opens with an introduction. All 29
hint links and 44 README links return 200, and every `#anchor` exists on its
page. Verified: guide and shared tests (33 + 1), the 38-lesson
starter/reference verifier, formatting, and Clippy with `-D warnings` on all
three packages. No preview code changed, so no native visual pass was needed.
Nobody has playtested the new wording yet.

## Progression from Rustlings — October 4, 2026

The five progression changes planned earlier the same day are in. The course
has 42 lessons now, renumbered in order; see the curriculum in the README.

- **Quiz 1 (12), built from a description.** After chapter 03, the learner
  writes a tally view: a label and a row with Tap and Clear. The starter's
  `render` returns a placeholder line, so the playground still launches.
- **Quiz 2 (20), an inbox after chapter 05.** Four unmarked faults from lessons
  that no quiz had used: a weak handle where the owner needs a strong one (17),
  an observer that copies a stale value (10), a discarded subscription (11),
  and a detached task that Cancel can't stop (19). Leaving any one fault in
  fails the check with that fault's symptom, verified one at a time in a
  temporary copy.
- **Chapter 04 is `04_keyboard`, one idea per lesson:** focus (13), a key
  handler the learner writes (14, new), an action with an unscoped binding (15,
  new), and a key context (16, formerly 12). Focus now comes first.
- **More code later.** Lesson 24 has the learner write three drag handlers,
  lesson 29 both tasks, and lesson 26 a second way out of the overlay. The
  audit shows 17, 13 and 10 changed lines; before chapter 06, lessons stay at
  one or two short edits.
- **Repetition within a file.** Lesson 11 subscribes to a second event type,
  and lessons 24 and 26 each handle two paths that need the same cleanup.
- **Lessons 02 and 03 are views.** The fix sits in a `cx.listener` closure and
  in `render`. Lesson 01 stays a plain function checked by `rustc` alone, so a
  fresh clone still passes its first check without GPUI's test build. The
  first native check moved from lesson 04 to 02.
- **Names.** `04_keyboard`, `06_layout_states` and `07_dispatch` replace
  `04_interaction`, `06_responsive` and `07_deeper`; `cx.defer` moved to
  chapter 05 as lesson 18. The playground compiles each exercise as a module
  named after its file and looks previews up by name, so lessons 01–06 lost
  their special cases in `lib.rs`. The standalone example is
  `cargo run --example application1`.
- **Saved progress.** The state file now has a version 2 header. Version 1
  files map moved names (`interaction1` to `keyboard4`, `quiz1` to `quiz3`, and
  so on) on load, so reused names can't be confused; the ID-based legacy file
  counts positions in a frozen copy of the 38-lesson order. Guide tests cover
  both.

Verified: guide and shared tests (33 + 1), playground app checks including one
that opens and resets every lesson's preview, the 42-lesson starter/reference
verifier, quiz 2's faults one at a time, formatting, Clippy with
`-D warnings` on all three packages, and all 60 links in hints and READMEs.
Not done: a learner playtest. The native visual pass follows.

## Native visual pass of new and changed previews — October 4, 2026

Real Metal frames of the playground window, rendered offscreen with GPUI's
`VisualTestAppContext::capture_screenshot`, for lessons 02, 03, 11–16, 20, 24,
26, 28 and 29. Each lesson was captured in its starter state and with the
reference fixes applied, before and after driving it with Tab, Enter, arrow
keys, shortcuts and a mouse drag, in light and dark appearance. Every starter
shows its bug in the preview (02 stays at 0, 03 doesn't celebrate at 3, 11's
parent hears nothing, 14 ignores the arrows, 15 ignores Ctrl-L, 20 has no badge
and an empty log, 24 never moves and stays Dragging, 26 ignores Escape, 29
stays Loading), and every solved preview shows the intended result.

The pass found and fixed:

- **Focus rings filled whole surfaces grey.** The ring is a spread shadow
  drawn behind the element, so a focusable surface without a background showed
  it across its interior. Lessons 13–16 and quizzes 3–5 now give those surfaces
  the card background, and the ring is an outline.
- **Two buttons couldn't be reached with Tab.** Lesson 26's Open and quiz 4's
  launcher track focus handles that weren't Tab stops, so a keyboard user
  couldn't start either flow, although quiz 4 asks for keyboard-only use. Both
  handles are now Tab stops.
- **Quiz 1 started as a blank card, and its Reset sat under the playground's
  own Reset.** The starter now shows "Your tally view goes here", and the
  quiz's button is Clear.
- **Smaller:** lessons 15 and 16 keep one panel width when their focus text
  changes, lesson 13's pad no longer wraps to three lines, and lesson 29 says
  "Not started" instead of ": 0" before Start.

The capture tool was temporary: a hidden constructor for the playground view,
an example, and dev-dependencies on `gpui-pre-macos` (test-support) and
`image`. It was removed afterwards. Re-verified after the fixes: all checks
above, plus quiz 4's faults one at a time.
