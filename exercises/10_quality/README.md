# Ship-quality GPUI · 34–38

[34 · Expose an accessible control](../../playground/src/exercises/app/accessibility.rs)
uses GPUI Base's unstyled Switch. It supplies the Switch role, toggled state,
and keyboard activation, while application code provides the accessible name.
The headless check inspects the resulting AccessKit node and activates the
control by pointer and keyboard.

After solving it, inspect the switch with a screen reader on your platform.
The automated check cannot confirm the spoken announcement or focus order in
a native assistive technology session.

[35 · Test behavior through GPUI](../../playground/src/exercises/app/behavior_test.rs)
puts the learner in the test author role. The view works already; complete the
headless test by sending a click, checking the loading render, advancing the
simulated clock, and checking the ready render. GPUI's test executor controls
time, so the check never waits for wall-clock time.

[36 · Render a large collection efficiently](../../playground/src/exercises/app/large_list.rs)
uses GPUI Base's virtual list. Its callback receives the visible range of
indices. Construct only those rows; building all 1,000 and then selecting the
range still wastes work. The check measures constructed rows, scrolls to the
last record, and confirms selection remains attached to its stable row ID.

[37 · Compose a reusable component](../../playground/src/exercises/app/reusable.rs)
wraps GPUI Base's switch with an explicit checked input, a change callback,
disabled behavior, and theme colors. Separate child views own separate values.
The check turns one on and off, toggles the other, and verifies that the
disabled instance never changes.

[38 · Capstone: small native workspace](../../playground/src/exercises/app/capstone.rs)
combines observed model entities, a responsive list and detail, a scoped Save
action, focus, simulated async loading and retry, temporary persistence, and
an accessible Save name. Five bugs are reported as symptoms instead of marked
in the source. The check stops at the first symptom it sees, so fix them one at
a time and confirm each in the native preview. Each fix reuses an earlier
chapter: task lifetimes, retry state, breakpoints, entity reads, and focus.
