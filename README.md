# GPUI Lings

A small, native UI workshop. Learn [GPUI](https://github.com/zed-industries/zed/tree/main/crates/gpui)
by repairing a real app, one focused change at a time.

## Start the workshop

You need recent stable Rust and the native prerequisites in the
[GPUI Kit installation guide](https://gpui-kit.com/docs/installation/).
From this repository, run:

```sh
./gpui-lings
```

The terminal opens your saved lesson and launches the playground. Edit the
exercise file, save, and explore the result. The guide checks your change and
rebuilds the app automatically. Passing a check leaves you on the lesson so you
can experiment; type `n` and Enter when you are ready to continue.

The progress bar counts the current lesson as soon as its check passes. While
checking and rebuilding, the previous preview stays open with an outdated notice.
It is replaced only once the new window is ready, keeping the position and size
you chose. If a build or launch fails, the previous preview stays available.
Closing the playground ends the terminal session; quitting or closing the terminal
also closes the playground. Automatic reloads keep the session running. Closing
the playground during a check or rebuild cancels that work without reopening it.
Placement includes the monitor, so rebuilds also work across multiple displays.
The session rechecks reached lessons after source changes and on startup. If a
stash or reset makes an earlier exercise fail again, progress drops and the
session returns to the first failing lesson. Compilation errors keep the current
lesson selected while excluding unverified exercises from progress.

The exercises **intentionally start with failing checks**. They compile and run,
but a small behavior or layout detail is wrong. The native window keeps to your
code's output: a progress bar, the lesson goal, and a live preview. The status
above the preview is red while the check fails, naming the file to edit, and
green once you can move on. **Reset** returns the preview to its initial state
without touching source files. Buttons support Tab / Shift-Tab and Enter / Space.

The window uses shadcn/ui's neutral palette and follows the system's light or
dark appearance; set `GPUI_LINGS_APPEARANCE=light` or `dark` to override it.

## Your terminal companion

The terminal puts the current exercise first: its number and title, source
file, goal, and one status line. Hints, source instructions, and diagnostics appear on demand.
Use `?` for all commands. Type a command and press Enter:

| Command | What it does |
| --- | --- |
| `h` | Show a hint; repeat for a more specific one |
| `?` | Show all session commands |
| `g` | Read the instructions from the current Rust file |
| `l` | See the curriculum |
| `d` | Inspect test and build diagnostics |
| `r` | Check again and reopen the playground |
| `p` | Return to the previous lesson |
| `n` | Verify the current lesson and continue if it passes |
| `q` | Close the session and its playground |

Your place is saved in `.gpui-lings-progress`. Earlier completion records resume
at the first newly added exercise: 06 for the original course, 07 for the
six-exercise course. The guide supports `NO_COLOR` and
plain output when redirected or used in a `TERM=dumb` terminal. `COLUMNS` can
set its wrapping width. Closing standard input also ends the session cleanly.

One-shot commands are available too:

```sh
./gpui-lings list
./gpui-lings check       # Report all lessons; failing exercises are expected
./gpui-lings check 07    # Run one focused check
./gpui-lings hint        # Hint for your saved lesson
./gpui-lings hint 38 2   # Second, more specific hint for lesson 38
./gpui-lings app 07      # Start exploring contexts without changing your saved place
```

## Curriculum

Thirty-eight exercises across ten chapters:

| Lesson | Concept | Check |
| --- | --- | --- |
| 01 · Render a greeting | Element trees and text | Exact rendered input |
| 02 · Update state on click | State transitions and listeners | Two increments |
| 03 · Render derived state | A single source of truth | Boundary values 0, 2, 3, 4 |
| 04 · Compose a row with flex | Parent-owned layout | Rendered positions |
| 05 · Update a child entity | Entity state and notifications | Rendered click interactions |
| 06 · Give a layout room to breathe | Gap versus padding | Both gaps measure 16px |
| 07 · Tell GPUI what changed | `Context<Self>` and `notify` | State, notification, and rendered signal |
| 08 · Connect a handler to its view | `cx.listener` | One handler call per click across redraws |
| 09 · Update the entity, not a copy | `Entity::read` and `update` | Original entity identity and persistent child state |
| 10 · Observe a model | `cx.observe` | Mirrored state after local and external updates |
| 11 · Keep a subscription alive | `EventEmitter`, `emit`, `Subscription` | Repeated event payload delivery |
| 12 · Route a keyboard action | Actions, bindings, key contexts | Scoped Ctrl-K behavior |
| 13 · Move keyboard focus | `FocusHandle` and `Window` | Key delivery while focused, isolation after blur |
| 14 · Inspect a weak handle | `WeakEntity` and ownership | Upgrade before release, safe access afterward |
| 15 · Keep async work alive | `Task`, `AsyncApp`, cancellation | Timed completion and cancellation |
| 16 · Adapt a layout to window width | Window bounds and responsive flex direction | Card positions at wide and narrow sizes, including resize back |
| 17 · Keep content usable in a small window | Vertical overflow and scroll handles | Last row visible after scrolling; heading stays put |
| 18 · Show the state of a control | Hover, focus, selected, disabled | Mouse and keyboard activation respect disabled state |
| 19 · Route a pointer gesture | Mouse down, move, release | Value changes during drag and release outside clears it |
| 20 · Layout checkpoint: compact inspector | Responsive layout, scrolling, selection | Four reported bugs, found without source markers |
| 21 · Move between focus regions | Focus handles and overlays | Key routing across two regions; restore focus on close |
| 22 · Route a nested action | Action propagation | Child handling or parent fallback, once per key |
| 23 · Defer a follow-up update | `cx.defer` and borrow boundaries | Queued then Settled in one effect cycle |
| 24 · Input checkpoint: command menu | Actions, focus, keyboard routing | Three reported bugs, found without source markers |
| 25 · Run work in the background | Background executor and task result | UI responds while pending, then renders the returned sum |
| 26 · Show errors and retry | Async UI states | Error clears immediately on retry; successful result appears |
| 27 · Ignore stale results | Out-of-order completion | Newer selection remains after older request finishes |
| 28 · Async checkpoint: searchable results | Native text input and async search | Three reported bugs, found without source markers |
| 29 · Start a standalone GPUI app | Application, root view, window | New window and visible root view |
| 30 · Share application state | Global and global observation | Both child views follow one setting |
| 31 · Save and restore a setting | File persistence and fallback | Round-trip plus missing and invalid data |
| 32 · Open a second window | Shared entity and window lifecycle | Updates from either window, cleanup, reopening |
| 33 · Follow appearance changes | Window appearance and semantic colors | Legible text in light and dark modes |
| 34 · Expose an accessible control | Role, name, state, keyboard activation | AccessKit node and pointer/Space behavior |
| 35 · Test behavior through GPUI | Headless interaction and simulated time | Learner-written click, loading, and completion checks |
| 36 · Render a large collection efficiently | Virtual list and visible range | Row construction, selection, and last-row scroll |
| 37 · Compose a reusable component | Controlled input and event output | Independent state and disabled behavior |
| 38 · Capstone: small native workspace | Entities, responsive panes, actions, async and save | Five reported bugs, found without source markers |

Each Rust file under [`playground/src/exercises`](playground/src/exercises/)
contains its explanation, task, and check. The terminal's `g` command reads
those comments. A TODO marks the line to change in lessons 01–15 and the
function or element in later lessons. Checkpoints list reported symptoms
instead, and lessons 30 and 32 revisit lesson 11 without a marker. Optional
hints, ordering, and check selection live in [`shared/lessons.rs`](shared/lessons.rs).
The default terminal screen remains brief.

Chapter references connect the ideas without repeating every task:
[Foundations](exercises/01_basics/README.md),
[Views & layout](exercises/02_views/README.md),
[Contexts & handlers](exercises/03_contexts/README.md),
[Actions & focus](exercises/04_interaction/README.md),
[Lifetimes & async](exercises/05_lifetimes/README.md),
[Responsive views](exercises/06_responsive/README.md),
[Deeper contexts & input](exercises/07_deeper/README.md),
[Async data & failure paths](exercises/08_async/README.md),
[Application structure](exercises/09_application/README.md), and
[Ship-quality GPUI](exercises/10_quality/README.md).

Every preview is independent of earlier solutions. Lesson 03 has a working
increment, and 07–38 each create their own view. Reset preview recreates the
active view, releasing its subscriptions and tasks.

## Development checks

The terminal package has no dependencies and starts without compiling GPUI.
The playground is a separate package. GPUI Kit pins its compatible GPUI snapshot;
most lessons use GPUI's basic APIs. Lesson 28 uses GPUI Base's unstyled input
for platform text handling, lesson 34 uses its unstyled switch for accessible
activation, and lesson 36 uses its virtual list for large collections.

```sh
cargo test
cargo clippy --all-targets -- -D warnings
cargo test --manifest-path playground/Cargo.toml --lib -- --skip exercise_
cargo clippy --manifest-path playground/Cargo.toml --all-targets -- -D warnings
python3 scripts/verify_lessons.py
python3 scripts/audit_lessons.py   # Report fix size and TODO markers per lesson
```

The Python check is for the repository's **unsolved starter state**. It checks
that exactly the thirty-eight exercise tests fail, then applies reference
solutions in a temporary copy and checks that every test passes. It never edits
learner files. Run it after native dependencies have been downloaded; it uses Cargo's
offline mode and reuses the local build cache.

See [PLAN.md](PLAN.md) for the remaining curriculum and [QUALITY.md](QUALITY.md)
for the latest verification notes.
