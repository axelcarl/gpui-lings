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
