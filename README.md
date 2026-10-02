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
can experiment; press `n` when you are ready to continue.

Open the repository in an editor with rust-analyzer (VS Code, Zed, Helix,
Neovim, …) to get errors, types on hover and completions in every exercise file.
The playground's manifest sits at the repository root for this reason, and
[`rust-analyzer.toml`](rust-analyzer.toml) keeps the editor's checks in their own
target directory, so they never hold up the guide.

While checking and rebuilding, the previous preview stays open with a blue
loading status. It is replaced only once the new window is ready, keeping the
position and size you chose. If a build or launch fails, the previous preview
stays available. Each rebuild opens a fresh window, so anything you focused or
typed in the preview starts over. Closing the playground ends the terminal
session; quitting or closing the terminal also closes the playground. Closing
the playground during a check or rebuild cancels that work without reopening it.
Placement includes the monitor, so rebuilds also work across multiple displays.

The exercises **intentionally start with failing checks**. They compile and run,
but a small behavior or layout detail is wrong. The native window keeps to your
code's output: a progress bar, the lesson goal, and a live preview. The status
above the preview is red while the check fails, naming the file to edit, and
green once you can move on. **Reset** returns the preview to its initial state
without touching source files. Buttons support Tab / Shift-Tab and Enter / Space.

The window uses shadcn/ui's neutral palette and follows the system's light or
dark appearance; set `GPUI_LINGS_APPEARANCE=light` or `dark` to override it.

## Your terminal companion

The guide works like [Rustlings' watch mode](https://github.com/rust-lang/rustlings):
each key acts immediately, without Enter. The screen is aligned to the bottom of
the terminal. Diagnostics come first, then a hint if you asked for one, the
progress bar, the current exercise and its goal, and the keys you can press:

```text
n:next / h:hint / l:list / c:check all / x:reset / q:quit ?
```

| Key | What it does |
| --- | --- |
| `n` | Move on to the next pending exercise. Shown once the current check passes |
| `h` | Show a hint; press again for a more specific one, while there is one |
| `l` | Open the exercise list (below) |
| `c` | Check every exercise, update your progress, and continue at the first pending one if the current exercise is done |
| `x` | Reset the current exercise, after a `y/n` confirmation |
| `q` | Close the session and its playground (Ctrl-C works too) |

**Reset** follows Rustlings' approach for exercises it doesn't embed: it runs
`git stash push -- <file>`, so your version is never lost. `git stash pop`
brings it back.

**The list** shows every exercise with its state. Move with `↓`/`j`, `↑`/`k`,
`home`/`g` and `end`/`G`; `c` continues at the selected exercise, `r` resets it,
`s` or `/` searches by name, `d` and `p` show only done or pending exercises,
and `q` returns to the exercise.

Compiler errors keep Rust's colored source excerpts and suggestions. A failed
check leads with the check's own message, then shows the whole assertion from
the exercise file and the compared values:

```text
error: check failed: after Release target, inspecting must report Released
  --> exercises/05_lifetimes/lifetimes1.rs:118:13
    |
118 |             assert_eq!(
119 |                 panel.read(cx).status,
120 |                 "Released",
121 |                 "after Release target, inspecting must report Released"
122 |             );
    |
    = left:  "Available"
    = right: "Released"
```

Your current exercise and completed exercise names are saved separately in
`.gpui-lings-state.txt`, following [Rustlings' state model](https://github.com/rust-lang/rustlings/blob/main/src/app_state.rs).
Saving checks the current exercise and records whether it passes. Going back
through the list doesn't erase completed exercises, and `n` skips completed
exercises to find the next pending one, wrapping to earlier pending work when
needed. When every exercise is marked done, `n` checks them all once more.
Existing `.gpui-lings-progress` files migrate automatically on the first
session; the old file is preserved. The guide supports `NO_COLOR` and plain
output when redirected or used in a `TERM=dumb` terminal. Without a terminal,
input is read line by line, one key per character. Closing standard input also
ends the session cleanly.

One-shot commands are available too:

```sh
./gpui-lings list
./gpui-lings check       # Report all lessons; failing exercises are expected
./gpui-lings check contexts1  # Names and numeric IDs both work
./gpui-lings hint        # Hint for your saved lesson
./gpui-lings hint 38 2   # Second, more specific hint for lesson 38
./gpui-lings reset 07    # Reset an exercise; your version goes to `git stash`
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
| 17 · Keep content usable in a small window | Vertical overflow and scroll handles | Last row reached by wheel and by Jump to last; heading stays put |
| 18 · Keep a disabled control disabled | Disabled state and activation paths | Mouse and keyboard activation respect disabled state |
| 19 · Route a pointer gesture | Mouse down, move, release | Value changes during drag and release outside clears it |
| 20 · Quiz 1: compact inspector | Responsive layout, scrolling, selection | Four reported bugs, found without source markers |
| 21 · Move between focus regions | Focus handles and overlays | Key routing across two regions; restore focus on close |
| 22 · Route a nested action | Action propagation | Child handling or parent fallback, once per key |
| 23 · Defer a follow-up update | `cx.defer` and borrow boundaries | Queued then Settled in one effect cycle |
| 24 · Quiz 2: command menu | Actions, focus, keyboard routing | Three reported bugs, found without source markers |
| 25 · Run work in the background | Background executor and task result | UI responds while pending, then renders the returned sum |
| 26 · Show errors and retry | Async UI states | Error clears immediately on retry; successful result appears |
| 27 · Ignore stale results | Out-of-order completion | Newer selection remains after older request finishes |
| 28 · Quiz 3: searchable results | Native text input and async search | Three reported bugs, found without source markers |
| 29 · Start a standalone GPUI app | Application, root view, window | New window and visible root view |
| 30 · Share application state | Global and global observation | Both child views follow one setting |
| 31 · Save and restore a setting | File persistence and fallback | Round-trip plus missing and invalid data |
| 32 · Open a second window | Shared entity and window lifecycle | Updates from either window, cleanup, reopening |
| 33 · Follow appearance changes | Window appearance and semantic colors | Legible text in light and dark modes |
| 34 · Expose an accessible control | Role, name, state, keyboard activation | AccessKit node and pointer/Space behavior |
| 35 · Test behavior through GPUI | Headless interaction and simulated time | Learner-written click, loading, and completion checks |
| 36 · Render a large collection efficiently | Virtual list and visible range | Row construction, selection, and last-row scroll |
| 37 · Compose a reusable component | Controlled input and event output | Independent state and disabled behavior |
| 38 · Quiz 4 (capstone): small native workspace | Entities, responsive panes, actions, async and save | Five reported bugs, found without source markers |

Each Rust file under [`exercises`](exercises/)
contains its explanation, task, usage examples, and check. Comments inside the
code explain what each part does, and a TODO says what to change, as in
Rustlings. Files follow Rustlings' topic-and-number convention, such as
`exercises/01_basics/basics1.rs` and `exercises/03_contexts/contexts2.rs` (the
listener lesson). Like Rustlings, the course has
[quizzes](exercises/quizzes/README.md): lessons 20, 24, 28 and 38 combine
earlier lessons and list reported symptoms instead of TODO markers. Optional
hints, ordering, and check selection live in
[`shared/lessons.rs`](shared/lessons.rs).

Chapter references connect the ideas without repeating every task:
[Foundations](exercises/01_basics/README.md),
[Views & layout](exercises/02_views/README.md),
[Contexts & handlers](exercises/03_contexts/README.md),
[Actions & focus](exercises/04_interaction/README.md),
[Lifetimes & async](exercises/05_lifetimes/README.md),
[Responsive views](exercises/06_responsive/README.md),
[Deeper contexts & input](exercises/07_deeper/README.md),
[Async data & failure paths](exercises/08_async/README.md),
[Application structure](exercises/09_application/README.md),
[Ship-quality GPUI](exercises/10_quality/README.md), and the
[quizzes](exercises/quizzes/README.md).

Every preview is independent of earlier solutions. Lesson 03 has a working
increment, and 07–38 each create their own view. Reset preview recreates the
active view, releasing its subscriptions and tasks.

## Development checks

The repository is one Cargo workspace:

- the root package is the **playground** (`playground/src`), the GPUI app that
  compiles every exercise file as a module;
- [`guide`](guide/) is the **terminal guide** behind `./gpui-lings`. It has no
  GPUI dependency and starts without compiling the playground;
- [`shared`](shared/) holds the lesson catalog and preview messages both use.

GPUI Kit pins its compatible GPUI snapshot; most lessons use GPUI's basic APIs.
Lesson 28 uses GPUI Base's unstyled input for platform text handling, lesson 34
uses its unstyled switch for accessible activation, and lesson 36 uses its
virtual list for large collections.

```sh
cargo test -p gpui-lings -p gpui-lings-shared
cargo clippy -p gpui-lings -p gpui-lings-shared --all-targets -- -D warnings
cargo test -p gpui-lings-playground --lib -- --skip exercise_
cargo clippy -p gpui-lings-playground --all-targets -- -D warnings
python3 scripts/verify_lessons.py
python3 scripts/audit_lessons.py   # Report fix size and TODO markers per lesson
```

Start a session at any lesson with `./gpui-lings <lesson>`, for example
`./gpui-lings 12` or `./gpui-lings interaction1`. It saves that lesson as your
place, which is handy when testing one exercise.

The Python check is for the repository's **unsolved starter state**. It checks
that exactly the thirty-eight exercise tests fail, then applies reference
solutions in a temporary copy and checks that every test passes. It never edits
learner files. Run it after native dependencies have been downloaded; it uses Cargo's
offline mode and reuses the local build cache.

See [PLAN.md](PLAN.md) for the remaining curriculum and [QUALITY.md](QUALITY.md)
for the latest verification notes.
